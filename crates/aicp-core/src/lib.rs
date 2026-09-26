//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod assurance;
pub mod engine;
pub mod intent;
pub mod plan;
pub mod telemetry;

pub use assurance::*;
pub use engine::*;
pub use intent::*;
pub use plan::*;
pub use telemetry::*;
