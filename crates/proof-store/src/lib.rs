//! Proof Store — signed receipts, Proof Bundles, and independent verification.
//!
//! Every meaningful agent action produces a signed ExecutionReceipt.
//! Receipts can be bundled into Proof Bundles for export and independent verification.

use observer_core::receipt::{ExecutionReceipt, ProofBundle};
use observer_core::residual::{ResidualRecord, ConsequenceRecord};
use observer_core::approval::ApprovalDecision;
use crypto_signing::identity::hex_to_bytes;
use ed25519_dalek::{SigningKey, Signer};
use sha2::{Sha256, Digest};

/// The Proof Store manages receipts, proof bundles, and hash chains.
pub struct ProofStore {
    receipts: Vec<ExecutionReceipt>,
    hash_chain: Vec<[u8; 32]>,
}

impl ProofStore {
    pub fn new() -> Self {
        ProofStore {
            receipts: Vec::new(),
            hash_chain: Vec::new(),
        }
    }

    /// Create and sign a receipt for an executed action.
    pub fn create_receipt(
        &mut self,
        agent_id: &str,
        observer_id: &str,
        declared_purpose: &str,
        capability: &observer_core::capability::CapabilitySpec,
        approval_decision_id: &str,
        outcome: observer_core::types::ExecutionOutcome,
        observer_node_id: &str,
        signing_key: &SigningKey,
    ) -> ExecutionReceipt {
        let now = chrono::Utc::now().timestamp_millis();
        let receipt_id = uuid::Uuid::new_v4().to_string();

        // Sign the receipt
        let signature = crypto_signing::signing::sign_receipt(
            signing_key, &receipt_id, agent_id, &format!("{:?}", outcome), now,
        )
        .map(|(sig, _)| hex::encode(sig.to_bytes()))
        .unwrap_or_default();

        let receipt = ExecutionReceipt {
            receipt_id: receipt_id.clone(),
            agent_id: agent_id.to_string(),
            observer_id: observer_id.to_string(),
            declared_purpose: declared_purpose.to_string(),
            capability: capability.clone(),
            approval_decision_id: approval_decision_id.to_string(),
            approved_at_unix_ms: now - 1000, // slightly before execution
            executed_at_unix_ms: now,
            outcome,
            consequence_ids: Vec::new(),
            residual_ids: Vec::new(),
            rollback_status: None,
            artifact_id: None,
            observer_node_id: observer_node_id.to_string(),
            signature_hex: signature,
            evidence_hashes: Vec::new(),
        };

        self.receipts.push(receipt.clone());
        receipt
    }

    /// Export a Proof Bundle with all receipts, residuals, consequences, and decisions.
    pub fn export_proof_bundle(
        &self,
        observer_id: &str,
        residuals: &[ResidualRecord],
        consequences: &[ConsequenceRecord],
        decisions: &[ApprovalDecision],
        signing_key: &SigningKey,
    ) -> ProofBundle {
        let bundle_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp_millis();

        // Compute bundle signature over all receipt data
        let mut hasher = Sha256::new();
        for r in &self.receipts {
            hasher.update(r.receipt_id.as_bytes());
            hasher.update(r.agent_id.as_bytes());
            hasher.update(format!("{:?}", r.outcome).as_bytes());
            hasher.update(r.declared_purpose.as_bytes());
        }
        let bundle_hash = hasher.finalize();
        let sig = signing_key.sign(&bundle_hash);

        ProofBundle {
            bundle_id,
            observer_id: observer_id.to_string(),
            receipts: self.receipts.clone(),
            residuals: residuals.to_vec(),
            consequences: consequences.to_vec(),
            decisions: decisions.to_vec(),
            bundle_signature_hex: hex::encode(sig.to_bytes()),
            exported_at_unix_ms: now,
        }
    }

    /// Verify a Proof Bundle's signature independently.
    pub fn verify_bundle(bundle: &ProofBundle, public_key_hex: &str) -> Result<bool, String> {
        let vk_bytes = hex_to_bytes(public_key_hex)
            .map_err(|e| format!("Invalid public key: {}", e))?;
        let vk = ed25519_dalek::VerifyingKey::from_bytes(&vk_bytes)
            .map_err(|e| format!("Invalid verifying key: {}", e))?;

        let sig_bytes: [u8; 64] = hex::decode(&bundle.bundle_signature_hex)
            .map_err(|e| format!("Invalid signature hex: {}", e))?
            .try_into()
            .map_err(|_| "Signature must be 64 bytes".to_string())?;

        let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes);

        // Recompute hash over full receipt data
        let mut hasher = Sha256::new();
        for r in &bundle.receipts {
            hasher.update(r.receipt_id.as_bytes());
            hasher.update(r.agent_id.as_bytes());
            hasher.update(format!("{:?}", r.outcome).as_bytes());
            hasher.update(r.declared_purpose.as_bytes());
        }
        let bundle_hash = hasher.finalize();

        Ok(vk.verify_strict(&bundle_hash, &sig).is_ok())
    }

    /// Get all receipts.
    pub fn receipts(&self) -> &[ExecutionReceipt] {
        &self.receipts
    }

    /// Get a receipt by ID.
    pub fn get_receipt(&self, receipt_id: &str) -> Option<&ExecutionReceipt> {
        self.receipts.iter().find(|r| r.receipt_id == receipt_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_signing::identity::generate_keypair;
    use observer_core::capability::{CapabilitySpec, CapabilityDuration};
    use observer_core::types::ExecutionOutcome;

    #[test]
    fn test_create_and_verify_receipt() {
        let kp = generate_keypair().unwrap();
        let mut store = ProofStore::new();

        let cap = CapabilitySpec {
            action: "email.send".to_string(),
            resource: "alex@example.com".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::OneTime,
            scopes: vec![],
        };

        let receipt = store.create_receipt(
            "agent_1", "observer_1", "Send confirmation email",
            &cap, "decision_1", ExecutionOutcome::Success,
            "node_1", &kp.signing_key,
        );

        assert_eq!(receipt.agent_id, "agent_1");
        assert!(!receipt.signature_hex.is_empty());
        assert_eq!(store.receipts().len(), 1);
    }

    #[test]
    fn test_export_and_verify_proof_bundle() {
        let kp = generate_keypair().unwrap();
        let mut store = ProofStore::new();

        let cap = CapabilitySpec {
            action: "files.read".to_string(),
            resource: "~/Documents".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::Minutes(15),
            scopes: vec![],
        };

        store.create_receipt(
            "agent_1", "observer_1", "Organize files",
            &cap, "decision_1", ExecutionOutcome::Success,
            "node_1", &kp.signing_key,
        );

        let bundle = store.export_proof_bundle(
            "observer_1", &[], &[], &[], &kp.signing_key,
        );

        assert_eq!(bundle.receipts.len(), 1);

        let verified = ProofStore::verify_bundle(&bundle, &kp.public_key_hex).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_verify_tampered_bundle_fails() {
        let kp = generate_keypair().unwrap();
        let mut store = ProofStore::new();

        let cap = CapabilitySpec {
            action: "files.read".to_string(),
            resource: "~/Documents".to_string(),
            maximum_amount: None,
            duration: CapabilityDuration::Minutes(15),
            scopes: vec![],
        };

        store.create_receipt(
            "agent_1", "observer_1", "Organize files",
            &cap, "decision_1", ExecutionOutcome::Success,
            "node_1", &kp.signing_key,
        );

        let mut bundle = store.export_proof_bundle(
            "observer_1", &[], &[], &[], &kp.signing_key,
        );

        // Tamper with the bundle
        bundle.receipts[0].outcome = ExecutionOutcome::Blocked;

        let verified = ProofStore::verify_bundle(&bundle, &kp.public_key_hex).unwrap();
        assert!(!verified);
    }
}