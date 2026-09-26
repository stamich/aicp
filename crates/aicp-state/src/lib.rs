//! Public API façade. Implementation lives in responsibility-focused modules.

pub mod model;
pub mod policy;
pub mod fingerprint;
pub mod drift;

pub use model::*;
pub use policy::*;
pub use fingerprint::*;
pub use drift::*;
