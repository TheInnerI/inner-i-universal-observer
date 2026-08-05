//! Capability Broker — evaluates, grants, and revokes agent capabilities.
//!
//! This is the central authorization engine. It evaluates capability requests
//! against the active policy, generates approval requests for human review,
//! and tracks active grants with expiration.

use observer_core::capability::{CapabilitySpec, CapabilityDuration};
use observer_core::types::ProtectionLevel;
use policy_engine::{Policy, PolicyDecision};

/// A pending capability request awaiting human decision.
#[derive(Debug, Clone)]
pub struct PendingRequest {
    pub request_id: String,
    pub agent_id: String,
    pub agent_display_name: String,
    pub capability: CapabilitySpec,
    pub decision: PolicyDecision,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
}

/// An active capability grant.
#[derive(Debug, Clone)]
pub struct ActiveGrant {
    pub grant_id: String,
    pub agent_id: String,
    pub capability: CapabilitySpec,
    pub granted_at_ms: i64,
    pub expires_at_ms: Option<i64>,
}

/// The Capability Broker.
pub struct CapabilityBroker {
    policy: Policy,
    pending_requests: Vec<PendingRequest>,
    active_grants: Vec<ActiveGrant>,
}

impl CapabilityBroker {
    pub fn new(protection_level: ProtectionLevel) -> Result<Self, anyhow::Error> {
        let policy = policy_engine::loader::load_default_policy(&protection_level)?;
        Ok(CapabilityBroker {
            policy,
            pending_requests: Vec::new(),
            active_grants: Vec::new(),
        })
    }

    /// Evaluate a capability request. Returns either an immediate allow/deny,
    /// or a pending request that needs human approval.
    pub fn evaluate_request(
        &mut self,
        agent_id: &str,
        agent_display_name: &str,
        capability: &CapabilitySpec,
    ) -> CapabilityEvaluation {
        let decision = self.policy.evaluate(&capability.action, &capability.resource);

        match decision {
            PolicyDecision::Allow => {
                let grant = self.grant(agent_id, capability, None);
                CapabilityEvaluation::Allowed { grant }
            }
            PolicyDecision::Deny { reason } => {
                CapabilityEvaluation::Denied { reason }
            }
            PolicyDecision::AskUser { risk_level, reasons } => {
                let request_id = uuid::Uuid::new_v4().to_string();
                let now = chrono::Utc::now().timestamp_millis();
                let pending = PendingRequest {
                    request_id: request_id.clone(),
                    agent_id: agent_id.to_string(),
                    agent_display_name: agent_display_name.to_string(),
                    capability: capability.clone(),
                    decision: PolicyDecision::AskUser {
                        risk_level: risk_level.clone(),
                        reasons: reasons.clone(),
                    },
                    created_at_ms: now,
                    expires_at_ms: now + 300_000, // 5 min expiry
                };
                self.pending_requests.push(pending.clone());
                CapabilityEvaluation::PendingApproval {
                    request_id,
                    risk_level,
                    reasons,
                    pending,
                }
            }
        }
    }

    /// Record a human approval decision.
    pub fn record_decision(
        &mut self,
        request_id: &str,
        approved: bool,
        duration_seconds: Option<u64>,
    ) -> Result<Option<ActiveGrant>, String> {
        // Find and remove the pending request
        let idx = self
            .pending_requests
            .iter()
            .position(|r| r.request_id == request_id)
            .ok_or_else(|| format!("Request {} not found", request_id))?;

        let request = self.pending_requests.remove(idx);

        if !approved {
            return Ok(None);
        }

        let expires = duration_seconds.map(|s| {
            chrono::Utc::now().timestamp_millis() + (s as i64 * 1000)
        });

        let grant = self.grant(&request.agent_id, &request.capability, expires);
        Ok(Some(grant))
    }

    /// Grant a capability to an agent.
    fn grant(
        &mut self,
        agent_id: &str,
        capability: &CapabilitySpec,
        expires_at_ms: Option<i64>,
    ) -> ActiveGrant {
        let grant = ActiveGrant {
            grant_id: uuid::Uuid::new_v4().to_string(),
            agent_id: agent_id.to_string(),
            capability: capability.clone(),
            granted_at_ms: chrono::Utc::now().timestamp_millis(),
            expires_at_ms: expires_at_ms,
        };
        self.active_grants.push(grant.clone());
        grant
    }

    /// Revoke all grants for an agent.
    pub fn revoke_agent(&mut self, agent_id: &str) -> usize {
        let before = self.active_grants.len();
        self.active_grants.retain(|g| g.agent_id != agent_id);
        before - self.active_grants.len()
    }

    /// Check if an agent has an active grant for a given action+resource.
    pub fn check_grant(&self, agent_id: &str, action: &str, resource: &str) -> bool {
        let now = chrono::Utc::now().timestamp_millis();
        self.active_grants.iter().any(|g| {
            g.agent_id == agent_id
                && g.capability.action == action
                && (g.capability.resource == resource || g.capability.resource == "*")
                && g
                    .expires_at_ms
                    .map_or(true, |exp| now < exp)
        })
    }

    /// Get all pending requests.
    pub fn pending_requests(&self) -> &[PendingRequest] {
        &self.pending_requests
    }

    /// Get all active grants.
    pub fn active_grants(&self) -> &[ActiveGrant] {
        &self.active_grants
    }
}

/// Result of capability evaluation.
#[derive(Debug, Clone)]
pub enum CapabilityEvaluation {
    Allowed { grant: ActiveGrant },
    Denied { reason: String },
    PendingApproval {
        request_id: String,
        risk_level: String,
        reasons: Vec<String>,
        pending: PendingRequest,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_capability() -> CapabilitySpec {
        CapabilitySpec {
            action: "files.read".to_string(),
            resource: "~/Documents".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::Minutes(15),
            scopes: vec![],
        }
    }

    #[test]
    fn test_observe_allows_safe_read() {
        let mut broker = CapabilityBroker::new(ProtectionLevel::Observe).unwrap();
        let result = broker.evaluate_request("agent_1", "Test Agent", &test_capability());
        assert!(matches!(result, CapabilityEvaluation::Allowed { .. }));
    }

    #[test]
    fn test_credentials_always_denied() {
        let mut broker = CapabilityBroker::new(ProtectionLevel::Observe).unwrap();
        let cred_cap = CapabilitySpec {
            action: "credentials.request".to_string(),
            resource: "".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::OneTime,
            scopes: vec![],
        };
        let result = broker.evaluate_request("agent_1", "Test Agent", &cred_cap);
        assert!(matches!(result, CapabilityEvaluation::Denied { .. }));
    }

    #[test]
    fn test_strict_asks_before_email() {
        let mut broker = CapabilityBroker::new(ProtectionLevel::Strict).unwrap();
        let email_cap = CapabilitySpec {
            action: "email.send".to_string(),
            resource: "alex@example.com".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::OneTime,
            scopes: vec![],
        };
        let result = broker.evaluate_request("agent_1", "Test Agent", &email_cap);
        assert!(matches!(result, CapabilityEvaluation::PendingApproval { .. }));
    }

    #[test]
    fn test_approve_pending_request() {
        let mut broker = CapabilityBroker::new(ProtectionLevel::Strict).unwrap();
        let email_cap = CapabilitySpec {
            action: "email.send".to_string(),
            resource: "alex@example.com".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::OneTime,
            scopes: vec![],
        };
        let result = broker.evaluate_request("agent_1", "Test Agent", &email_cap);
        let request_id = match result {
            CapabilityEvaluation::PendingApproval { request_id, .. } => request_id,
            _ => panic!("Expected pending"),
        };

        let grant = broker.record_decision(&request_id, true, Some(900)).unwrap();
        assert!(grant.is_some());
        assert!(broker.pending_requests().is_empty());
        assert!(!broker.active_grants().is_empty());
    }

    #[test]
    fn test_deny_pending_request() {
        let mut broker = CapabilityBroker::new(ProtectionLevel::Strict).unwrap();
        let email_cap = CapabilitySpec {
            action: "email.send".to_string(),
            resource: "alex@example.com".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::OneTime,
            scopes: vec![],
        };
        let result = broker.evaluate_request("agent_1", "Test Agent", &email_cap);
        let request_id = match result {
            CapabilityEvaluation::PendingApproval { request_id, .. } => request_id,
            _ => panic!("Expected pending"),
        };

        let grant = broker.record_decision(&request_id, false, None).unwrap();
        assert!(grant.is_none());
        assert!(broker.pending_requests().is_empty());
    }

    #[test]
    fn test_revoke_agent() {
        let mut broker = CapabilityBroker::new(ProtectionLevel::Observe).unwrap();
        let _ = broker.evaluate_request("agent_1", "Test Agent", &test_capability());
        let cap2 = CapabilitySpec {
            action: "calendar.read".to_string(),
            resource: "*".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::OneTime,
            scopes: vec![],
        };
        let _ = broker.evaluate_request("agent_1", "Test Agent", &cap2);
        assert_eq!(broker.active_grants().len(), 2);

        let revoked = broker.revoke_agent("agent_1");
        assert_eq!(revoked, 2);
        assert_eq!(broker.active_grants().len(), 0);
    }
}