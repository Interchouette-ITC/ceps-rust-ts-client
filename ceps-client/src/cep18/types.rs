//! CEP-18 install / upgrade / security argument types.

use crate::types::EventsMode;

/// Arguments for CEP-18 `install`.
#[derive(Debug, Clone)]
pub struct InstallArgs {
    /// Token name.
    pub name: String,
    /// Token symbol.
    pub symbol: String,
    /// Decimals.
    pub decimals: u8,
    /// Initial total supply as decimal string (U256).
    pub total_supply: String,
    /// Optional events mode.
    pub events_mode: Option<EventsMode>,
    /// Enable mint and burn entrypoints.
    pub enable_mint_and_burn: Option<bool>,
    /// Optional admin list (prefixed keys).
    pub admin_list: Option<Vec<String>>,
    /// Optional minter list (prefixed keys).
    pub minter_list: Option<Vec<String>>,
}

impl InstallArgs {
    /// Required fields constructor.
    pub fn new(
        name: impl Into<String>,
        symbol: impl Into<String>,
        decimals: u8,
        total_supply: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            total_supply: total_supply.into(),
            events_mode: None,
            enable_mint_and_burn: None,
            admin_list: None,
            minter_list: None,
        }
    }

    /// Set events mode.
    pub fn with_events_mode(mut self, mode: EventsMode) -> Self {
        self.events_mode = Some(mode);
        self
    }

    /// Enable mint/burn.
    pub fn with_mint_and_burn(mut self, enable: bool) -> Self {
        self.enable_mint_and_burn = Some(enable);
        self
    }
}

/// Arguments for CEP-18 `upgrade`.
#[derive(Debug, Clone)]
pub struct UpgradeArgs {
    /// Token / collection name used as named-key anchor.
    pub name: String,
    /// Optional new events mode.
    pub events_mode: Option<EventsMode>,
}

impl UpgradeArgs {
    /// Create upgrade args.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            events_mode: None,
        }
    }
}

/// Arguments for `change_security`.
#[derive(Debug, Clone, Default)]
pub struct ChangeSecurityArgs {
    /// Accounts granted admin rights.
    pub admin_list: Option<Vec<String>>,
    /// Accounts granted minter rights.
    pub minter_list: Option<Vec<String>>,
    /// Accounts revoked to none.
    pub none_list: Option<Vec<String>>,
}
