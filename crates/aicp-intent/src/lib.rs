pub mod error;
pub mod model;
pub mod normalize;
pub mod parser;
pub mod validate;

pub use error::IntentError;
pub use model::{ConstraintsWire, GoalsWire, IntentDocument, LowerF64, Metadata, PreferencesWire, Spec, TargetWire, UpperU64};
pub use normalize::normalize;
pub use parser::{parse_and_normalize, parse_document};
pub use validate::{validate_document, validate_ir};
