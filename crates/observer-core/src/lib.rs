//! Observer Core — shared types and models for the Inner I Universal Observer.
//!
//! This crate defines the canonical Rust representations of IIOP messages
//! used across the Observer Node, SDK, and Control Center.

pub mod types;
pub mod capability;
pub mod approval;
pub mod receipt;
pub mod residual;
pub mod iiop;

pub use types::*;
pub use capability::*;
pub use approval::*;
pub use receipt::*;
pub use residual::*;
pub use iiop::*;
