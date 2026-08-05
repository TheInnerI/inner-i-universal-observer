use serde::{Deserialize, Serialize};
use crate::types::*;

/// A Residual is the measurable difference between intended/permitted
/// behavior and observed reality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualRecord {
    pub residual_id: String,
    pub event_id: String,
    pub residual_type: ResidualType,
    pub plain_language_summary: String,
    pub expected_behavior: String,
    pub observed_behavior: String,
    pub severity: ResidualSeverity,
    pub response: String,
    pub data_exposed: bool,
    pub reversible: bool,
    pub correction_status: String,
    pub evidence_hash: String,
    pub detected_at_unix_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResidualType {
    IntentDivergence,
    CapabilityViolation,
    InvariantViolation,
    ScopeViolation,
    UnauthorizedDataAccess,
    UnauthorizedDataDisclosure,
    UnauthorizedPurchaseAttempt,
    UnauthorizedNetworkAttempt,
    UnauthorizedProcessSpawn,
    CredentialAccessAttempt,
    UnexpectedStateChange,
    RollbackUnavailable,
    EvidenceGap,
    IdentityMismatch,
    ResourceAnomaly,
    RepeatedViolation,
    UnverifiedConsequence,
    CorrectionFailure,
}

/// A ConsequenceRecord tracks meaningful outcomes of agent actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequenceRecord {
    pub consequence_id: String,
    pub consequence_type: String,
    pub description: String,
    pub affected_resource: String,
    pub intent_id: Option<String>,
    pub grant_id: Option<String>,
    pub event_id: Option<String>,
    pub residual_id: Option<String>,
    pub rollback_id: Option<String>,
    pub recorded_at_unix_ms: i64,
}
