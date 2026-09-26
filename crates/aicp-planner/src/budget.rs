//! Planning and adaptation budgets.

/// Hard limits preventing candidate explosion and excessive adaptation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlanningBudget {
    /// Maximum candidate plans evaluated in one cycle.
    pub max_candidates: usize,
}

impl Default for PlanningBudget {
    /// Returns a conservative limit for milestone 0.3.
    fn default() -> Self {
        Self { max_candidates: 32 }
    }
}

/// Budget limiting how aggressively a running system may adapt.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdaptationBudget {
    /// Maximum accepted abstract migration cost for a single plan.
    pub max_migration_cost_units: f64,
    /// Maximum adaptations allowed during one operational hour.
    pub max_changes_per_hour: u32,
}

impl Default for AdaptationBudget {
    /// Returns demo-safe adaptation limits.
    fn default() -> Self {
        Self {
            max_migration_cost_units: 30.0,
            max_changes_per_hour: 6,
        }
    }
}
