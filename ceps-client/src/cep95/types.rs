//! CEP-95 install argument types (Odra session + init).

/// Arguments for Odra CEP-95 / OwnedCep95 install.
#[derive(Debug, Clone)]
pub struct InstallArgs {
    /// Collection name (init).
    pub name: String,
    /// Collection symbol (init).
    pub symbol: String,
    /// Account named key that will hold the package hash (`odra_cfg_package_hash_key_name`).
    pub package_hash_key_name: String,
    /// Allow overwriting an existing package named key.
    pub allow_key_override: bool,
    /// Whether the package is upgradable.
    pub is_upgradable: bool,
    /// Whether this install is an upgrade of an existing package.
    pub is_upgrade: bool,
}

impl InstallArgs {
    /// Required fields with Odra defaults (new upgradable package, no override).
    pub fn new(
        name: impl Into<String>,
        symbol: impl Into<String>,
        package_hash_key_name: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            package_hash_key_name: package_hash_key_name.into(),
            allow_key_override: false,
            is_upgradable: true,
            is_upgrade: false,
        }
    }

    /// Allow overwriting the package named key.
    pub fn with_allow_key_override(mut self, allow: bool) -> Self {
        self.allow_key_override = allow;
        self
    }

    /// Set upgradable flag.
    pub fn with_upgradable(mut self, upgradable: bool) -> Self {
        self.is_upgradable = upgradable;
        self
    }

    /// Mark as upgrade install.
    pub fn with_upgrade(mut self, upgrade: bool) -> Self {
        self.is_upgrade = upgrade;
        self
    }
}
