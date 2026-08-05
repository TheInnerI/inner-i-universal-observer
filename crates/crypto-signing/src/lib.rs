//! Inner I Crypto Signing — Ed25519 identity, signing, and verification.
//!
//! Every Inner I identity (observer, device, agent, node) uses Ed25519 keypairs.
//! Private keys are stored in platform-secure storage or encrypted fallback.
//! All approvals, revocations, emergency stops, and receipts are signed.

pub mod identity;
pub mod signing;
pub mod verification;
pub mod error;

pub use identity::*;
pub use signing::*;
pub use verification::*;
pub use error::*;