//! Policy loader — loads YAML policy files from disk.

use super::{Policy, PolicyRule, ProtectionLevel};
use serde::Deserialize;

#[derive(Deserialize)]
struct PolicyFile {
    protection_level: String,
    description: String,
    auto_allow: Vec<RuleFile>,
    ask_before: Vec<RuleFile>,
    always_deny: Vec<RuleFile>,
    emergency_protections: Vec<String>,
}

#[derive(Deserialize)]
struct RuleFile {
    action_pattern: String,
    scope: Option<String>,
    reason: Option<String>,
}

/// Load a policy from a YAML string.
pub fn load_policy(yaml: &str) -> Result<Policy, anyhow::Error> {
    let pf: PolicyFile = serde_yaml::from_str(yaml)?;

    let protection_level = match pf.protection_level.as_str() {
        "observe" => ProtectionLevel::Observe,
        "ask_me" => ProtectionLevel::AskMe,
        "strict" => ProtectionLevel::Strict,
        "sovereign" => ProtectionLevel::Sovereign,
        other => anyhow::bail!("Unknown protection level: {}", other),
    };

    Ok(Policy {
        protection_level,
        description: pf.description,
        auto_allow: pf.auto_allow.into_iter().map(convert_rule).collect(),
        ask_before: pf.ask_before.into_iter().map(convert_rule).collect(),
        always_deny: pf.always_deny.into_iter().map(convert_rule).collect(),
        emergency_protections: pf.emergency_protections,
    })
}

/// Load a policy from a file path.
pub fn load_policy_file(path: &str) -> Result<Policy, anyhow::Error> {
    let yaml = std::fs::read_to_string(path)?;
    load_policy(&yaml)
}

/// Load the default policy for a protection level.
pub fn load_default_policy(level: &ProtectionLevel) -> Result<Policy, anyhow::Error> {
    let yaml = match level {
        ProtectionLevel::Observe => include_str!("../../../policies/observe.yaml"),
        ProtectionLevel::AskMe => include_str!("../../../policies/ask-me.yaml"),
        ProtectionLevel::Strict => include_str!("../../../policies/strict.yaml"),
        ProtectionLevel::Sovereign => include_str!("../../../policies/sovereign.yaml"),
    };
    load_policy(yaml)
}

fn convert_rule(rf: RuleFile) -> PolicyRule {
    PolicyRule {
        action_pattern: rf.action_pattern,
        scope: rf.scope,
        reason: rf.reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_observe_policy() {
        let policy = load_default_policy(&ProtectionLevel::Observe).unwrap();
        assert_eq!(policy.protection_level, ProtectionLevel::Observe);
        assert!(!policy.auto_allow.is_empty());
        assert!(!policy.emergency_protections.is_empty());
    }

    #[test]
    fn test_load_all_policies() {
        for level in &[ProtectionLevel::Observe, ProtectionLevel::AskMe, ProtectionLevel::Strict, ProtectionLevel::Sovereign] {
            let policy = load_default_policy(level).unwrap();
            assert_eq!(policy.protection_level, *level);
            assert!(!policy.description.is_empty());
        }
    }
}