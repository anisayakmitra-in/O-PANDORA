use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::manifest::PackageManifest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageState {
    Installed,

    Loaded,

    Active,

    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedPackage {
    pub manifest: PackageManifest,

    pub state: PackageState,
}

#[derive(Debug, Default)]
pub struct RuntimePackageLoader {
    packages: HashMap<String, LoadedPackage>,
}

impl RuntimePackageLoader {
    pub fn install(&mut self, manifest: PackageManifest) {
        let package = LoadedPackage {
            manifest: manifest.clone(),

            state: PackageState::Installed,
        };

        self.packages.insert(manifest.package_name.clone(), package);
    }

    pub fn activate(&mut self, package_name: &str) {
        if let Some(package) = self.packages.get_mut(package_name) {
            package.state = PackageState::Active;
        }
    }

    pub fn disable(&mut self, package_name: &str) {
        if let Some(package) = self.packages.get_mut(package_name) {
            package.state = PackageState::Disabled;
        }
    }

    pub fn uninstall(&mut self, package_name: &str) {
        self.packages.remove(package_name);
    }

    pub fn active_packages(&self) -> Vec<&LoadedPackage> {
        self.packages
            .values()
            .filter(|package| matches!(package.state, PackageState::Active))
            .collect()
    }
}
