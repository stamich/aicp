pub mod drift;
pub mod fingerprint;
pub mod model;
pub mod policy;

pub use drift::{detect_storage_drift, Drift};
pub use fingerprint::fingerprint;
pub use model::{DatasetState, EngineHealth, ObservedState, ResourceSnapshot};
pub use policy::AdaptationPolicy;
