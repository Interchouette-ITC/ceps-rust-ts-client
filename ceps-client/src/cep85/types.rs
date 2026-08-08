//! CEP-85 install / upgrade / security argument types.

use crate::types::EventsMode;

/// Arguments for CEP-85 install.
#[derive(Debug, Clone)]
pub struct InstallArgs {
    /// Collection name.
    pub name: String,
    /// URI template.
    pub uri: String,
    /// Optional events mode.
    pub events_mode: Option<EventsMode>,
    /// Enable burn.
    pub enable_burn: Option<bool>,
    /// Optional admin list (entity-prefixed keys).
    pub admin_list: Option<Vec<String>>,
    /// Optional minter list.
    pub minter_list: Option<Vec<String>>,
    /// Optional burner list.
    pub burner_list: Option<Vec<String>>,
    /// Optional meta list.
    pub meta_list: Option<Vec<String>>,
}

impl InstallArgs {
    /// Required fields constructor.
    pub fn new(name: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uri: uri.into(),
            events_mode: None,
            enable_burn: None,
            admin_list: None,
            minter_list: None,
            burner_list: None,
            meta_list: None,
        }
    }

    /// Set events mode.
    pub fn with_events_mode(mut self, mode: EventsMode) -> Self {
        self.events_mode = Some(mode);
        self
    }

    /// Enable burn.
    pub fn with_enable_burn(mut self, enable: bool) -> Self {
        self.enable_burn = Some(enable);
        self
    }
}

/// Arguments for CEP-85 upgrade (`upgrade: true` is always sent).
#[derive(Debug, Clone)]
pub struct UpgradeArgs {
    /// Collection name.
    pub name: String,
}

impl UpgradeArgs {
    /// Create upgrade args.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Arguments for `change_security`.
#[derive(Debug, Clone, Default)]
pub struct ChangeSecurityArgs {
    /// Admin list.
    pub admin_list: Option<Vec<String>>,
    /// Minter list.
    pub minter_list: Option<Vec<String>>,
    /// Burner list.
    pub burner_list: Option<Vec<String>>,
    /// Meta list.
    pub meta_list: Option<Vec<String>>,
    /// None list.
    pub none_list: Option<Vec<String>>,
}
