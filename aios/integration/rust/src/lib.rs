//! AIOS — AI Software Engineering Operating System
//! 
//! This crate provides a zero-dependency integration layer for the AIOS
//! framework. It reads the AIOS registry, resolves capabilities, routes
//! requests, and manages module loading — all from the filesystem without
//! requiring a network stack or runtime.

pub mod registry;
pub mod loader;
pub mod router;
pub mod types;
pub mod error;

pub use registry::AiosRegistry;
pub use loader::AiosLoader;
pub use router::AiosRouter;
pub use error::AiosError;