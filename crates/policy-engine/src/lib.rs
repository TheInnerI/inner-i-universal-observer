//! Policy Engine — evaluates protection levels, capability requests, and auto-rules.
//!
//! Loads YAML policy files, matches action patterns against capability requests,
//! and determines whether an action should be auto-allowed, auto-denied, or escalated
//! for human approval.

pub mod loader;
pub mod evaluator;

pub use loader::*;
pub use evaluator::*;

use serde::{Deserialize, Serialize};
use observer_core::types::ProtectionLevel;

/// A loaded policy that can evaluate capability requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub protection_level: ProtectionLevel,
    pub description: String,
    pub auto_allow: Vec<PolicyRule>,
    pub ask_before: Vec<PolicyRule>,
    pub always_deny: Vec<PolicyRule>,
    pub emergency_protections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub action_pattern: String,
    pub scope: Option<String>,
    pub reason: Option<String>,
}

/// Result of evaluating a capability request against a policy.
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyDecision {
    /// Action is safe, proceed without asking
    Allow,
    /// Action needs human approval
    AskUser {
        risk_level: String,
        reasons: Vec<String>,
    },
    /// Action is permanently denied
    Deny {
        reason: String,
    },
}