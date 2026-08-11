//! CEP-85 install / upgrade / security argument types.

use crate::types::EventsMode;

/// CEP-85 security badge (u8 on-chain).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SecurityBadge85 {
    /// Admin.
    Admin = 0,
    /// Minter.
    Minter = 1,
    /// Burner.
    Burner = 2,
    /// Meta / URI role.
    Meta = 3,
    /// No rights.
    None = 4,
}

impl SecurityBadge85 {
    /// Parse from on-chain u8.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Admin),
            1 => Some(Self::Minter),
            2 => Some(Self::Burner),
            3 => Some(Self::Meta),
            4 => Some(Self::None),
            _ => None,
        }
    }

    /// Stable name for logs / CLI.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "Admin",
            Self::Minter => "Minter",
            Self::Burner => "Burner",
            Self::Meta => "Meta",
            Self::None => "None",
        }
    }
}

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
    /// Optional transfer-filter contract key.
    pub transfer_filter_contract: Option<String>,
    /// Optional transfer-filter method name (required when contract is set).
    pub transfer_filter_method: Option<String>,
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
            transfer_filter_contract: None,
            transfer_filter_method: None,
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

    /// Set admin list.
    pub fn with_admin_list(mut self, list: Vec<String>) -> Self {
        self.admin_list = Some(list);
        self
    }

    /// Set minter list.
    pub fn with_minter_list(mut self, list: Vec<String>) -> Self {
        self.minter_list = Some(list);
        self
    }

    /// Set burner list.
    pub fn with_burner_list(mut self, list: Vec<String>) -> Self {
        self.burner_list = Some(list);
        self
    }

    /// Set meta list.
    pub fn with_meta_list(mut self, list: Vec<String>) -> Self {
        self.meta_list = Some(list);
        self
    }

    /// Set transfer filter contract + method.
    pub fn with_transfer_filter(
        mut self,
        contract: impl Into<String>,
        method: impl Into<String>,
    ) -> Self {
        self.transfer_filter_contract = Some(contract.into());
        self.transfer_filter_method = Some(method.into());
        self
    }
}

/// Arguments for CEP-85 upgrade (`upgrade: true` is always sent).
#[derive(Debug, Clone)]
pub struct UpgradeArgs {
    /// Collection name.
    pub name: String,
    /// Optional transfer-filter contract key.
    pub transfer_filter_contract: Option<String>,
    /// Optional transfer-filter method name.
    pub transfer_filter_method: Option<String>,
}

impl UpgradeArgs {
    /// Create upgrade args.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            transfer_filter_contract: None,
            transfer_filter_method: None,
        }
    }

    /// Set transfer filter contract + method.
    pub fn with_transfer_filter(
        mut self,
        contract: impl Into<String>,
        method: impl Into<String>,
    ) -> Self {
        self.transfer_filter_contract = Some(contract.into());
        self.transfer_filter_method = Some(method.into());
        self
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
