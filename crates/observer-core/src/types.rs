use serde::{Deserialize, Serialize};

/// Protection level for an Observer profile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProtectionLevel {
    /// Record activity, notify about important actions, minimal auto-blocking.
    Observe,
    /// Ask before sensitive actions, auto-allow low-risk within scope.
    AskMe,
    /// Deny undeclared actions, require approval for sends/spends/deletes.
    Strict,
    /// Local-first, no cloud, deny network unless approved, require signed evidence.
    Sovereign,
}

impl ProtectionLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "observe" => Some(Self::Observe),
            "ask_me" | "askme" => Some(Self::AskMe),
            "strict" => Some(Self::Strict),
            "sovereign" => Some(Self::Sovereign),
            _ => None,
        }
    }
}

/// Enforcement level reported by an Observer Node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementLevel {
    Cooperative,
    ProcessRestricted,
    FilesystemScoped,
    NetworkRestricted,
    StrongOsSandbox,
    CapabilitySandboxedWasm,
}

/// Decision type for an approval.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionType {
    AllowOnce,
    AllowForDuration { seconds: u64 },
    AllowUntil { unix_ms: i64 },
    AlwaysAllowWithinScope,
    DenyOnce,
    AlwaysDeny,
    RequestMoreInformation,
    StopAgent,
}

/// Risk levels for capability requests.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Reversibility of an action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Reversibility {
    None,
    Limited,
    Full,
    Unknown,
}

/// Residual severity levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResidualSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Outcome of an observed execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOutcome {
    Success,
    Failure,
    Blocked,
    Partial,
}
