

#![warn(missing_docs)]
#![warn(clippy::all)]

// Re-export all crates at the root
pub use agentropic_core as core;
pub use agentropic_messaging as messaging;
pub use agentropic_cognition as cognition;
pub use agentropic_patterns as patterns;
pub use agentropic_runtime as runtime;

/// Unified prelude for convenient imports
pub mod prelude;

// Re-export commonly used types at crate root for convenience
pub use core::{Agent, AgentId, Identity, Context, State, Error, Result};
pub use messaging::{Message, Performative, Router, Mailbox};
pub use runtime::{Runtime, RuntimeConfig};
