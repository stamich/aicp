use crate::model::EngineCapabilities;

/// Source of dynamically discovered capabilities.
pub trait CapabilityProvider {
    /// Discovers current capabilities from the backing engine.
    fn discover_capabilities(&self) -> Result<EngineCapabilities, String>;
}
