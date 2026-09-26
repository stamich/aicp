//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod model;
pub mod provider;
pub mod registry;

pub use model::*;
pub use provider::*;
pub use registry::*;
