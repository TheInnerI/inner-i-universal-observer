use serde::{Deserialize, Serialize};
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub receipt_id: String,
    pub agent_id: String,
    pub observer_id: String,
    pub declared_purpose: String,
    pub capability: crate::capability::CapabilitySpec,
    pub approval_decision_id: String,
    pub approved_at_unix_ms: i64,
    pub executed_at_unix_ms: i64,
    pub outcome: ExecutionOutcome,
    pub consequence_ids: Vec<String>,
    pub residual_ids: Vec<String>,
    pub rollback_status: Option<String>,
    pub artifact_id: Option<String>,
    pub observer_node_id: String,
    pub signature_hex: String,
    pub evidence_hashes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofBundle {
    pub bundle_id: String,
    pub observer_id: String,
    pub receipts: Vec<ExecutionReceipt>,
    pub residuals: Vec<crate::residual::ResidualRecord>,
    pub consequences: Vec<crate::residual::ConsequenceRecord>,
    pub decisions: Vec<ApprovalDecision>,
    pub bundle_signature_hex: String,
    pub exported_at_unix_ms: i64,
}

use crate::approval::ApprovalDecision;
