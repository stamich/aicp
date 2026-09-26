//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod adaptation;
pub mod budget;
pub mod candidate;
pub mod error;
pub mod explain;
pub mod feasibility;
pub mod planner;
pub mod result;

pub use adaptation::*;
pub use budget::*;
pub use error::*;
pub use explain::*;
pub use planner::*;
pub use result::*;
