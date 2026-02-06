//! # Agentropic
//!
//! A comprehensive Rust framework for building intelligent, autonomous multi-agent systems.
//!
//! ## Overview
//!
//! Agentropic provides everything you need to build production-ready multi-agent systems:
//!
//! - **Agent Primitives** - Core agent traits and lifecycle management
//! - **Messaging** - FIPA-compliant communication protocols
//! - **Cognition** - BDI (Belief-Desire-Intention) architecture
//! - **Patterns** - 8 organizational patterns (hierarchy, swarm, market, etc.)
//! - **Runtime** - Production-ready async execution with fault tolerance
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use agentropic::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create runtime
//!     let runtime = Runtime::new(RuntimeConfig::default())?;
//!     
//!     // Your agent code here
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! Currently, all features are enabled by default.
//!
//! ## Examples
//!
//! See the `examples/` directory for complete examples:
//!
//! - `quickstart.rs` - Simple agent creation
//! - `messaging.rs` - Agent communication
//! - `swarm.rs` - Swarm coordination
//! - `full_stack.rs` - Complete system
//!

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
