//! Package Health — automatic health status derived from CI, telemetry, security.
//!
//! Every package in K-O-Palace has a health state computed from install failures,
//! CI status, security reports, and compatibility data.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PackageHealth {
    #[default]
    Healthy,
    Warning,
    Broken,
    Abandoned,
    Maintained,
    Deprecated,
    Archived,
}

impl PackageHealth {
    pub fn label(&self) -> &str {
        match self {
            Self::Healthy => "healthy",
            Self::Warning => "warning",
            Self::Broken => "broken",
            Self::Abandoned => "abandoned",
            Self::Maintained => "maintained",
            Self::Deprecated => "deprecated",
            Self::Archived => "archived",
        }
    }

    /// Derive health from signals — CI failures, install errors, age.
    pub fn derive(ci_passing: bool, install_success_rate: f64, days_since_update: u64) -> Self {
        if !ci_passing {
            return Self::Broken;
        }
        if install_success_rate < 0.5 {
            return Self::Warning;
        }
        if days_since_update > 365 {
            return Self::Abandoned;
        }
        if days_since_update < 30 {
            return Self::Maintained;
        }
        Self::Healthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ci_failure_is_broken() {
        let h = PackageHealth::derive(false, 1.0, 1);
        assert_eq!(h, PackageHealth::Broken);
    }

    #[test]
    fn low_success_is_warning() {
        let h = PackageHealth::derive(true, 0.3, 10);
        assert_eq!(h, PackageHealth::Warning);
    }

    #[test]
    fn old_package_is_abandoned() {
        let h = PackageHealth::derive(true, 0.9, 400);
        assert_eq!(h, PackageHealth::Abandoned);
    }

    #[test]
    fn recent_is_maintained() {
        let h = PackageHealth::derive(true, 0.9, 10);
        assert_eq!(h, PackageHealth::Maintained);
    }
}
