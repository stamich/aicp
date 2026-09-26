//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod model;
pub mod registry;
pub mod provider;

pub use model::*;
pub use registry::*;
pub use provider::*;
