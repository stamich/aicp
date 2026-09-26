//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod classification;
pub mod comparison;
pub mod environment;

pub use classification::*;
pub use comparison::*;
pub use environment::*;
