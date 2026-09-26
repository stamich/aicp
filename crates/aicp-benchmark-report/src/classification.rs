use crate::model::RegressionClass;

/// Classifies a statistically meaningful relative percentage change.
pub fn classify(change_percent: f64, p_value: Option<f64>) -> RegressionClass {
    if p_value.is_some_and(|p| p >= 0.05) {
        return RegressionClass::Stable;
    }
    if change_percent <= -5.0 {
        RegressionClass::Improved
    } else if change_percent <= 5.0 {
        RegressionClass::Stable
    } else if change_percent <= 10.0 {
        RegressionClass::Warning
    } else {
        RegressionClass::Regression
    }
}
