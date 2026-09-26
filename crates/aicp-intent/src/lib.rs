//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod error;
pub mod model;
pub mod parser;
pub mod validate;

pub use error::*;
pub use parser::*;
pub use validate::*;
