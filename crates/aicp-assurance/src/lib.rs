//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod evaluator;
pub mod drift;

pub use evaluator::*;
pub use drift::*;
