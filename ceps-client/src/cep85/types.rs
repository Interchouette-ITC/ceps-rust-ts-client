//! CEP-85 install argument types.

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
}

impl InstallArgs {
    /// Required fields constructor.
    pub fn new(name: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uri: uri.into(),
            events_mode: None,
            enable_burn: None,
        }
    }
}
