use ed25519_dalek::{Signer, SigningKey, Signature};
use sha2::{Sha256, Digest};
use crate::error::CryptoError;

/// Sign arbitrary data with an Ed25519 signing key.
pub fn sign_data(signing_key: &SigningKey, data: &[u8]) -> Signature {
    signing_key.sign(data)
}

/// Sign a message with prefix for domain separation ("IIOP-SIGNATURE:").
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> (Signature, [u8; 32]) {
    let mut hasher = Sha256::new();
    hasher.update(b"IIOP-SIGNATURE:");
    hasher.update(message);
    let message_hash: [u8; 32] = hasher.finalize().into();
    let signature = signing_key.sign(&message_hash);
    (signature, message_hash)
}

/// Sign an approval decision with domain separation.
pub fn sign_approval(
    signing_key: &SigningKey,
    approval_id: &str,
    decision_type: &str,
    timestamp_ms: i64,
) -> Result<(Signature, Vec<u8>), CryptoError> {
    let payload = format!("APPROVAL:{}:{}:{}", approval_id, decision_type, timestamp_ms);
    let sig = signing_key.sign(payload.as_bytes());
    Ok((sig, payload.into_bytes()))
}

/// Sign an emergency stop command.
pub fn sign_emergency_stop(
    signing_key: &SigningKey,
    observer_id: &str,
    reason: &str,
    timestamp_ms: i64,
) -> Result<(Signature, Vec<u8>), CryptoError> {
    let payload = format!("EMERGENCY_STOP:{}:{}:{}", observer_id, reason, timestamp_ms);
    let sig = signing_key.sign(payload.as_bytes());
    Ok((sig, payload.into_bytes()))
}

/// Sign a receipt for an executed action.
pub fn sign_receipt(
    signing_key: &SigningKey,
    receipt_id: &str,
    agent_id: &str,
    outcome: &str,
    timestamp_ms: i64,
) -> Result<(Signature, Vec<u8>), CryptoError> {
    let payload = format!("RECEIPT:{}:{}:{}:{}", receipt_id, agent_id, outcome, timestamp_ms);
    let sig = signing_key.sign(payload.as_bytes());
    Ok((sig, payload.into_bytes()))
}

/// Create a hash-chain link: hash(prev_hash || new_data).
pub fn chain_hash(prev_hash: &[u8; 32], new_data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(prev_hash);
    hasher.update(new_data);
    hasher.finalize().into()
}

/// Compute SHA-256 hash of data.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::generate_keypair;
    use ed25519_dalek::Verifier;

    #[test]
    fn test_sign_and_verify() {
        let kp = generate_keypair().unwrap();
        let data = b"test message";
        let sig = sign_data(&kp.signing_key, data);
        kp.verifying_key.verify(data, &sig).unwrap();
    }

    #[test]
    fn test_sign_approval() {
        let kp = generate_keypair().unwrap();
        let (sig, payload) = sign_approval(
            &kp.signing_key,
            "approval_123",
            "ALLOW_ONCE",
            1000,
        ).unwrap();
        kp.verifying_key.verify(&payload, &sig).unwrap();
    }

    #[test]
    fn test_sign_emergency_stop() {
        let kp = generate_keypair().unwrap();
        let (sig, payload) = sign_emergency_stop(
            &kp.signing_key,
            "observer_1",
            "security concern",
            1000,
        ).unwrap();
        kp.verifying_key.verify(&payload, &sig).unwrap();
    }

    #[test]
    fn test_chain_hash() {
        let prev = sha256(b"block_1");
        let next = chain_hash(&prev, b"block_2");
        // Same inputs produce same hash
        let next2 = chain_hash(&prev, b"block_2");
        assert_eq!(next, next2);
        // Different inputs produce different hash
        let different = chain_hash(&prev, b"block_3");
        assert_ne!(next, different);
    }

    #[test]
    fn test_sign_receipt() {
        let kp = generate_keypair().unwrap();
        let (sig, payload) = sign_receipt(
            &kp.signing_key,
            "receipt_1",
            "agent_1",
            "success",
            1000,
        ).unwrap();
        kp.verifying_key.verify(&payload, &sig).unwrap();
    }
}
