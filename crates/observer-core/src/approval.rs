use serde::{Deserialize, Serialize};
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub approval_id: String,
    pub capability_request_id: String,
    pub agent_id: String,
    pub agent_display_name: String,
    pub action_description: String,
    pub requested: crate::capability::CapabilitySpec,
    pub data_involved: Vec<String>,
    pub prohibited: Vec<String>,
    pub risk_level: RiskLevel,
    pub reversibility: Reversibility,
    pub created_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    pub decision_id: String,
    pub approval_id: String,
    pub decision: DecisionType,
    pub observer_id: String,
    /// Ed25519 signature in hex
    pub signature_hex: String,
    /// Optional platform biometric attestation
    pub biometric_proof: Option<String>,
    pub decided_at_unix_ms: i64,
}
