use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::error::CryptoError;

/// An Inner I identity backed by an Ed25519 keypair.
#[derive(Serialize, Deserialize)]
pub struct ObserverIdentity {
    pub observer_id: String,
    pub display_name: String,
    pub public_key_hex: String,
    pub created_at_unix_ms: i64,
    pub device_fingerprint: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub device_id: String,
    pub device_type: DeviceType,
    pub os_name: String,
    pub os_version: String,
    pub app_version: String,
    pub public_key_hex: String,
    pub paired_at_unix_ms: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Mobile,
    Desktop,
    Server,
    Embedded,
}

#[derive(Serialize, Deserialize)]
pub struct AgentIdentity {
    pub agent_id: String,
    pub display_name: String,
    pub provider: String,
    pub declared_purpose: String,
    pub observer_node_id: String,
    pub public_key_hex: String,
    pub registered_at_unix_ms: i64,
}

/// A generated Ed25519 keypair ready for identity creation.
pub struct GeneratedKeypair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub public_key_hex: String,
}

impl Zeroize for GeneratedKeypair {
    fn zeroize(&mut self) {
        self.signing_key = SigningKey::from(&[0u8; 32]);
    }
}

impl Drop for GeneratedKeypair {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl ObserverIdentity {
    pub fn new(display_name: &str) -> Result<(Self, GeneratedKeypair), CryptoError> {
        let keypair = generate_keypair()?;
        let now = chrono::Utc::now().timestamp_millis();

        let identity = ObserverIdentity {
            observer_id: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
            public_key_hex: keypair.public_key_hex.clone(),
            created_at_unix_ms: now,
            device_fingerprint: None,
        };

        Ok((identity, keypair))
    }

    pub fn public_key_bytes(&self) -> Result<[u8; 32], CryptoError> {
        hex_to_bytes(&self.public_key_hex)
    }
}

impl DeviceIdentity {
    pub fn new(
        device_type: DeviceType,
        os_name: &str,
        os_version: &str,
        app_version: &str,
    ) -> Result<(Self, GeneratedKeypair), CryptoError> {
        let keypair = generate_keypair()?;
        let now = chrono::Utc::now().timestamp_millis();

        let identity = DeviceIdentity {
            device_id: Uuid::new_v4().to_string(),
            device_type,
            os_name: os_name.to_string(),
            os_version: os_version.to_string(),
            app_version: app_version.to_string(),
            public_key_hex: keypair.public_key_hex.clone(),
            paired_at_unix_ms: now,
        };

        Ok((identity, keypair))
    }
}

impl AgentIdentity {
    pub fn new(
        display_name: &str,
        provider: &str,
        declared_purpose: &str,
        observer_node_id: &str,
    ) -> Result<(Self, GeneratedKeypair), CryptoError> {
        let keypair = generate_keypair()?;
        let now = chrono::Utc::now().timestamp_millis();

        let identity = AgentIdentity {
            agent_id: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
            provider: provider.to_string(),
            declared_purpose: declared_purpose.to_string(),
            observer_node_id: observer_node_id.to_string(),
            public_key_hex: keypair.public_key_hex.clone(),
            registered_at_unix_ms: now,
        };

        Ok((identity, keypair))
    }
}

// ---- Key Generation ----

/// Generate a fresh Ed25519 keypair using OS randomness.
pub fn generate_keypair() -> Result<GeneratedKeypair, CryptoError> {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let signing_key = SigningKey::from(&bytes);
    let verifying_key = signing_key.verifying_key();
    // Key material is now in SigningKey (Zeroize on drop)

    Ok(GeneratedKeypair {
        public_key_hex: bytes_to_hex(verifying_key.as_bytes()),
        signing_key,
        verifying_key,
    })
}

/// Reconstruct signing key from raw bytes (for loading from secure storage).
pub fn signing_key_from_bytes(bytes: &[u8; 32]) -> SigningKey {
    SigningKey::from(bytes)
}

/// Reconstruct verifying key from raw bytes (for signature verification).
pub fn verifying_key_from_bytes(bytes: &[u8; 32]) -> Result<VerifyingKey, CryptoError> {
    VerifyingKey::from_bytes(bytes).map_err(|e| CryptoError::InvalidPublicKey(e.to_string()))
}

// ---- Hex Helpers ----

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_to_bytes(hex_str: &str) -> Result<[u8; 32], CryptoError> {
    let bytes = hex::decode(hex_str).map_err(|e| CryptoError::HexDecode(e.to_string()))?;
    if bytes.len() != 32 {
        return Err(CryptoError::InvalidKeyLength(bytes.len()));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let kp = generate_keypair().unwrap();
        assert_eq!(kp.public_key_hex.len(), 64);
        assert_eq!(hex_to_bytes(&kp.public_key_hex).unwrap().len(), 32);
    }

    #[test]
    fn test_create_observer_identity() {
        let (identity, kp) = ObserverIdentity::new("Test Observer").unwrap();
        assert_eq!(identity.display_name, "Test Observer");
        assert!(identity.observer_id.len() > 0);
        assert_eq!(identity.public_key_hex.len(), 64);
        assert_eq!(kp.public_key_hex, identity.public_key_hex);
    }

    #[test]
    fn test_keypair_roundtrip() {
        let kp = generate_keypair().unwrap();
        let bytes: [u8; 32] = hex_to_bytes(&kp.public_key_hex).unwrap();
        let vk = verifying_key_from_bytes(&bytes).unwrap();
        assert_eq!(vk.as_bytes(), kp.verifying_key.as_bytes());
    }
}