//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod intent;
pub mod engine;
pub mod plan;
pub mod telemetry;
pub mod assurance;

pub use intent::*;
pub use engine::*;
pub use plan::*;
pub use telemetry::*;
pub use assurance::*;
