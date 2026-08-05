//! Residual Engine — detects, records, and classifies discrepancies between
//! intended/permitted behavior and observed reality.
//!
//! A Residual is the measurable gap between what an agent declared it would do
//! and what it actually did. This engine generates ResidualRecords, produces
//! human-readable summaries, and tracks correction status.

use observer_core::residual::{ResidualRecord, ResidualType};
use observer_core::types::ResidualSeverity;

/// The Residual Engine.
pub struct ResidualEngine;

impl ResidualEngine {
    /// Generate a residual for a capability violation.
    pub fn capability_violation(
        agent_id: &str,
        expected_action: &str,
        observed_action: &str,
        data_exposed: bool,
    ) -> ResidualRecord {
        let severity = if data_exposed {
            ResidualSeverity::Critical
        } else {
            ResidualSeverity::High
        };

        let summary = format!(
            "Agent {} tried to perform '{}' but was only permitted '{}'",
            agent_id, observed_action, expected_action
        );

        ResidualRecord {
            residual_id: uuid::Uuid::new_v4().to_string(),
            event_id: String::new(),
            residual_type: ResidualType::CapabilityViolation,
            plain_language_summary: summary,
            expected_behavior: format!("Agent should only perform: {}", expected_action),
            observed_behavior: format!("Agent attempted: {}", observed_action),
            severity,
            response: "blocked".to_string(),
            data_exposed,
            reversible: false,
            correction_status: "none".to_string(),
            evidence_hash: String::new(),
            detected_at_unix_ms: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Generate a residual for intent divergence.
    pub fn intent_divergence(
        agent_id: &str,
        declared_purpose: &str,
        observed_behavior: &str,
    ) -> ResidualRecord {
        let summary = format!(
            "Agent {} declared purpose '{}' but exhibited '{}'",
            agent_id, declared_purpose, observed_behavior
        );

        ResidualRecord {
            residual_id: uuid::Uuid::new_v4().to_string(),
            event_id: String::new(),
            residual_type: ResidualType::IntentDivergence,
            plain_language_summary: summary,
            expected_behavior: format!("Behavior aligned with: {}", declared_purpose),
            observed_behavior: observed_behavior.to_string(),
            severity: ResidualSeverity::High,
            response: "blocked".to_string(),
            data_exposed: false,
            reversible: false,
            correction_status: "none".to_string(),
            evidence_hash: String::new(),
            detected_at_unix_ms: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Generate a residual for unauthorized credential access.
    pub fn credential_access_attempt(
        agent_id: &str,
        resource: &str,
    ) -> ResidualRecord {
        ResidualRecord {
            residual_id: uuid::Uuid::new_v4().to_string(),
            event_id: String::new(),
            residual_type: ResidualType::CredentialAccessAttempt,
            plain_language_summary: format!(
                "Agent {} tried to access credentials at '{}'",
                agent_id, resource
            ),
            expected_behavior: "Agent should never access credentials".to_string(),
            observed_behavior: format!("Attempted to read: {}", resource),
            severity: ResidualSeverity::Critical,
            response: "blocked".to_string(),
            data_exposed: false,
            reversible: false,
            correction_status: "none".to_string(),
            evidence_hash: String::new(),
            detected_at_unix_ms: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Generate a residual for scope violation.
    pub fn scope_violation(
        agent_id: &str,
        permitted_scope: &str,
        attempted_scope: &str,
    ) -> ResidualRecord {
        ResidualRecord {
            residual_id: uuid::Uuid::new_v4().to_string(),
            event_id: String::new(),
            residual_type: ResidualType::ScopeViolation,
            plain_language_summary: format!(
                "Agent {} was scoped to '{}' but tried '{}'",
                agent_id, permitted_scope, attempted_scope
            ),
            expected_behavior: format!("Operations scoped to: {}", permitted_scope),
            observed_behavior: format!("Attempted scope: {}", attempted_scope),
            severity: ResidualSeverity::Medium,
            response: "blocked".to_string(),
            data_exposed: false,
            reversible: false,
            correction_status: "none".to_string(),
            evidence_hash: String::new(),
            detected_at_unix_ms: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Compute an evidence hash for a residual.
    pub fn compute_evidence_hash(record: &ResidualRecord) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(record.residual_id.as_bytes());
        hasher.update(record.observed_behavior.as_bytes());
        hasher.update(severity_to_string(&record.severity).as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Attach evidence hash to a residual record.
    pub fn seal(record: &mut ResidualRecord) {
        record.evidence_hash = Self::compute_evidence_hash(record);
    }
}

/// Convert a ResidualSeverity to its string representation.
fn severity_to_string(s: &ResidualSeverity) -> &str {
    match s {
        ResidualSeverity::Low => "low",
        ResidualSeverity::Medium => "medium",
        ResidualSeverity::High => "high",
        ResidualSeverity::Critical => "critical",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_violation_residual() {
        let r = ResidualEngine::capability_violation(
            "agent_1",
            "files.read ~/Documents",
            "credentials.read ~/.browser/credentials",
            false,
        );
        assert_eq!(r.residual_type, ResidualType::CapabilityViolation);
        assert_eq!(r.severity, ResidualSeverity::High);
        assert_eq!(r.data_exposed, false);
        assert!(!r.plain_language_summary.is_empty());
    }

    #[test]
    fn test_credential_access_critical() {
        let r = ResidualEngine::credential_access_attempt("agent_1", "~/.ssh/id_rsa");
        assert_eq!(r.residual_type, ResidualType::CredentialAccessAttempt);
        assert_eq!(r.severity, ResidualSeverity::Critical);
    }

    #[test]
    fn test_evidence_hash_stable() {
        let r1 = ResidualEngine::capability_violation("a", "read docs", "read creds", false);
        let h1 = ResidualEngine::compute_evidence_hash(&r1);
        let h2 = ResidualEngine::compute_evidence_hash(&r1);
        assert_eq!(h1, h2); // Same input = same hash
    }

    #[test]
    fn test_seal_attaches_hash() {
        let mut r = ResidualEngine::capability_violation("a", "read docs", "read creds", false);
        assert!(r.evidence_hash.is_empty());
        ResidualEngine::seal(&mut r);
        assert!(!r.evidence_hash.is_empty());
    }
}