use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use crate::error::CryptoError;
use crate::identity::hex_to_bytes;

/// Verify an Ed25519 signature against data.
pub fn verify_signature(
    public_key_hex: &str,
    data: &[u8],
    signature_bytes: &[u8; 64],
) -> Result<bool, CryptoError> {
    let vk_bytes = hex_to_bytes(public_key_hex)?;
    let vk = VerifyingKey::from_bytes(&vk_bytes)
        .map_err(|e| CryptoError::InvalidPublicKey(e.to_string()))?;
    let sig = Signature::from_bytes(signature_bytes);
    Ok(vk.verify(data, &sig).is_ok())
}

/// Verify an approval signature with domain separation.
pub fn verify_approval(
    public_key_hex: &str,
    approval_id: &str,
    decision_type: &str,
    timestamp_ms: i64,
    signature_bytes: &[u8; 64],
) -> Result<bool, CryptoError> {
    let payload = format!("APPROVAL:{}:{}:{}", approval_id, decision_type, timestamp_ms);
    verify_signature(public_key_hex, payload.as_bytes(), signature_bytes)
}

/// Verify an emergency stop signature.
pub fn verify_emergency_stop(
    public_key_hex: &str,
    observer_id: &str,
    reason: &str,
    timestamp_ms: i64,
    signature_bytes: &[u8; 64],
) -> Result<bool, CryptoError> {
    let payload = format!("EMERGENCY_STOP:{}:{}:{}", observer_id, reason, timestamp_ms);
    verify_signature(public_key_hex, payload.as_bytes(), signature_bytes)
}

/// Verify a receipt signature.
pub fn verify_receipt(
    public_key_hex: &str,
    receipt_id: &str,
    agent_id: &str,
    outcome: &str,
    timestamp_ms: i64,
    signature_bytes: &[u8; 64],
) -> Result<bool, CryptoError> {
    let payload = format!("RECEIPT:{}:{}:{}:{}", receipt_id, agent_id, outcome, timestamp_ms);
    verify_signature(public_key_hex, payload.as_bytes(), signature_bytes)
}

/// Batch-verify multiple signatures against the same public key.
pub fn verify_batch(
    public_key_hex: &str,
    items: &[(Vec<u8>, [u8; 64])],
) -> Result<Vec<bool>, CryptoError> {
    let vk_bytes = hex_to_bytes(public_key_hex)?;
    let vk = VerifyingKey::from_bytes(&vk_bytes)
        .map_err(|e| CryptoError::InvalidPublicKey(e.to_string()))?;

    let results: Vec<bool> = items
        .iter()
        .map(|(data, sig_bytes)| {
            let sig = Signature::from_bytes(sig_bytes);
            vk.verify(data, &sig).is_ok()
        })
        .collect();

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::generate_keypair;
    use crate::signing::*;

    #[test]
    fn test_verify_valid_signature() {
        let kp = generate_keypair().unwrap();
        let data = b"verify me";
        let sig = sign_data(&kp.signing_key, data);
        let sig_bytes: [u8; 64] = sig.to_bytes();

        let valid = verify_signature(&kp.public_key_hex, data, &sig_bytes).unwrap();
        assert!(valid);
    }

    #[test]
    fn test_verify_tampered_data() {
        let kp = generate_keypair().unwrap();
        let sig = sign_data(&kp.signing_key, b"original");
        let sig_bytes: [u8; 64] = sig.to_bytes();

        let valid = verify_signature(&kp.public_key_hex, b"tampered", &sig_bytes).unwrap();
        assert!(!valid);
    }

    #[test]
    fn test_verify_approval_roundtrip() {
        let kp = generate_keypair().unwrap();
        let (sig, _payload) = sign_approval(&kp.signing_key, "app_1", "ALLOW_ONCE", 1000).unwrap();

        let valid = verify_approval(
            &kp.public_key_hex,
            "app_1",
            "ALLOW_ONCE",
            1000,
            &sig.to_bytes(),
        ).unwrap();
        assert!(valid);

        // Tampered decision should fail
        let valid_tampered = verify_approval(
            &kp.public_key_hex,
            "app_1",
            "DENY_ONCE", // different decision
            1000,
            &sig.to_bytes(),
        ).unwrap();
        assert!(!valid_tampered);
    }

    #[test]
    fn test_verify_batch() {
        let kp = generate_keypair().unwrap();

        let sig1 = sign_data(&kp.signing_key, b"item 1").to_bytes();
        let sig2 = sign_data(&kp.signing_key, b"item 2").to_bytes();

        let items = vec![
            (b"item 1".to_vec(), sig1),
            (b"item 2".to_vec(), sig2),
        ];

        let results = verify_batch(&kp.public_key_hex, &items).unwrap();
        assert_eq!(results, vec![true, true]);
    }
}
