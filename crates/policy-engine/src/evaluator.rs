//! Policy evaluator — matches capability requests against policy rules.

use super::{Policy, PolicyDecision};
use regex::Regex;

impl Policy {
    /// Evaluate a capability request against this policy.
    ///
    /// Order of precedence:
    /// 1. Emergency protections (always deny)
    /// 2. Always-deny rules
    /// 3. Auto-allow rules
    /// 4. Ask-before rules
    /// 5. Default: ask user for non-trivial actions
    pub fn evaluate(&self, action: &str, resource: &str) -> PolicyDecision {
        // 1. Check emergency protections first
        for ep in &self.emergency_protections {
            if matches_emergency(ep, action, resource) {
                return PolicyDecision::Deny {
                    reason: format!("Emergency protection: {}", ep),
                };
            }
        }

        // 2. Check always-deny rules
        for rule in &self.always_deny {
            if matches_rule(&rule.action_pattern, action)
                && scope_matches(rule.scope.as_deref(), resource)
            {
                return PolicyDecision::Deny {
                    reason: rule.reason.clone().unwrap_or_else(|| {
                        format!("'{}' is always denied by policy", action)
                    }),
                };
            }
        }

        // 3. Check auto-allow rules
        for rule in &self.auto_allow {
            if matches_rule(&rule.action_pattern, action)
                && scope_matches(rule.scope.as_deref(), resource)
            {
                return PolicyDecision::Allow;
            }
        }

        // 4. Check ask-before rules
        for rule in &self.ask_before {
            if matches_rule(&rule.action_pattern, action) {
                let mut reasons = vec![format!("'{}' requires approval", action)];
                if let Some(ref scope) = rule.scope {
                    if !scope_matches(Some(scope), resource) {
                        continue; // scope doesn't match, skip this rule
                    }
                }
                if let Some(ref reason) = rule.reason {
                    reasons.push(reason.clone());
                }
                return PolicyDecision::AskUser {
                    risk_level: risk_level_for_action(action),
                    reasons,
                };
            }
        }

        // 5. Default: ask for anything not explicitly covered
        match self.protection_level {
            observer_core::types::ProtectionLevel::Observe => PolicyDecision::Allow,
            observer_core::types::ProtectionLevel::Sovereign => PolicyDecision::AskUser {
                risk_level: "medium".to_string(),
                reasons: vec!["Sovereign mode: all actions require approval".to_string()],
            },
            _ => PolicyDecision::AskUser {
                risk_level: "low".to_string(),
                reasons: vec![format!("'{}' is not in auto-allow list", action)],
            },
        }
    }
}

/// Match a glob-style action pattern against an action string.
/// Supports: exact match, prefix match (`files.*`), wildcard match (`*`)
fn matches_rule(pattern: &str, action: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern == action {
        return true;
    }
    // Convert glob pattern to regex
    let regex_str = format!(
        "^{}$",
        regex::escape(pattern).replace(r"\*", ".*")
    );
    if let Ok(re) = Regex::new(&regex_str) {
        re.is_match(action)
    } else {
        false
    }
}

/// Check if a resource/scope matches. None means "any scope".
fn scope_matches(expected: Option<&str>, actual: &str) -> bool {
    match expected {
        None => true,
        Some(scope) => actual.contains(scope) || actual == scope,
    }
}

/// Determine risk level based on action class.
fn risk_level_for_action(action: &str) -> String {
    if action.contains("payment") || action.contains("purchase") || action.contains("transfer") {
        "high".to_string()
    } else if action.contains("credentials") || action.contains("secret") {
        "critical".to_string()
    } else if action.contains("delete") || action.contains("share") || action.contains("send") {
        "medium".to_string()
    } else {
        "low".to_string()
    }
}

/// Match emergency protection patterns.
fn matches_emergency(protection: &str, action: &str, resource: &str) -> bool {
    match protection {
        "prevent_credential_access" => {
            action.contains("credential") || resource.contains("credential")
        }
        "prevent_system_modification" => {
            action.contains("process.spawn") && (resource.contains("system") || resource.is_empty())
        }
        "prevent_network_listen" => action.contains("network.listen"),
        "prevent_network_connect" => action.contains("network.connect"),
        "prevent_undeclared_capability_expansion" => false, // checked at registration
        "require_signed_evidence_for_all" => false,         // checked at proof level
        "local_execution_only" => false,                     // checked at runtime
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::load_default_policy;
    use observer_core::types::ProtectionLevel;

    #[test]
    fn test_observe_allows_file_read() {
        let policy = load_default_policy(&ProtectionLevel::Observe).unwrap();
        let result = policy.evaluate("files.read", "~/Documents");
        assert_eq!(result, PolicyDecision::Allow);
    }

    #[test]
    fn test_observe_denies_credentials() {
        let policy = load_default_policy(&ProtectionLevel::Observe).unwrap();
        let result = policy.evaluate("credentials.request", "");
        assert!(matches!(result, PolicyDecision::Deny { .. }));
    }

    #[test]
    fn test_strict_asks_before_email_send() {
        let policy = load_default_policy(&ProtectionLevel::Strict).unwrap();
        let result = policy.evaluate("email.send", "alex@example.com");
        assert!(matches!(result, PolicyDecision::AskUser { .. }));
    }

    #[test]
    fn test_sovereign_asks_before_everything() {
        let policy = load_default_policy(&ProtectionLevel::Sovereign).unwrap();
        let result = policy.evaluate("calendar.read", "");
        assert!(matches!(result, PolicyDecision::AskUser { .. }));
    }

    #[test]
    fn test_wildcard_pattern() {
        assert!(matches_rule("email.*", "email.send"));
        assert!(matches_rule("email.*", "email.draft"));
        assert!(matches_rule("email.*", "email.read"));
        assert!(!matches_rule("email.*", "calendar.read"));
    }

    #[test]
    fn test_exact_match() {
        assert!(matches_rule("files.read", "files.read"));
        assert!(!matches_rule("files.read", "files.write"));
    }

    #[test]
    fn test_match_all() {
        assert!(matches_rule("*", "anything.here"));
        assert!(matches_rule("*", ""));
    }

    #[test]
    fn test_credential_emergency_always_denied() {
        for level in &[ProtectionLevel::Observe, ProtectionLevel::AskMe, ProtectionLevel::Strict, ProtectionLevel::Sovereign] {
            let policy = load_default_policy(level).unwrap();
            let result = policy.evaluate("credentials.request", "");
            assert!(matches!(result, PolicyDecision::Deny { .. }),
                "Level {:?} should deny credential access", level);
        }
    }
}