//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod adapter;
pub mod client;
pub mod in_memory;
pub mod model;

pub use adapter::*;
pub use client::*;
pub use in_memory::*;
pub use model::*;
