use serde::{Deserialize, Serialize};

/// IIOP Protocol Envelope — wraps every message on the wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IIOPEnvelope {
    pub protocol_version: String,
    pub message_id: String,
    pub message_type: String,
    pub timestamp_unix_ms: i64,
    pub sender_id: String,
    pub recipient_id: String,
    pub observer_id: String,
    pub session_id: String,
    pub parent_message_id: Option<String>,
    pub payload_hash_sha256: String,
    pub previous_record_hash: Option<String>,
    pub signature_hex: String,
    pub encryption: Option<EncryptionMetadata>,
    /// Base64-encoded serialized payload
    pub payload_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    pub algorithm: String,
    pub sender_public_key_hex: String,
    pub recipient_public_key_hex: String,
    pub nonce_hex: String,
}

impl IIOPEnvelope {
    /// Create a new envelope for a given message type and payload.
    pub fn new(
        message_type: &str,
        sender_id: &str,
        recipient_id: &str,
        observer_id: &str,
        session_id: &str,
        payload_base64: &str,
        signature_hex: &str,
        previous_record_hash: Option<&str>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let message_id = uuid::Uuid::new_v4().to_string();
        let payload_hash = crate::hash_payload(payload_base64);

        IIOPEnvelope {
            protocol_version: "0.1".to_string(),
            message_id,
            message_type: message_type.to_string(),
            timestamp_unix_ms: now,
            sender_id: sender_id.to_string(),
            recipient_id: recipient_id.to_string(),
            observer_id: observer_id.to_string(),
            session_id: session_id.to_string(),
            parent_message_id: None,
            payload_hash_sha256: payload_hash,
            previous_record_hash: previous_record_hash.map(|s| s.to_string()),
            signature_hex: signature_hex.to_string(),
            encryption: None,
            payload_base64: payload_base64.to_string(),
        }
    }
}

/// Hash a base64 payload for the envelope.
pub fn hash_payload(payload_base64: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(payload_base64.as_bytes());
    hex::encode(hasher.finalize())
}
