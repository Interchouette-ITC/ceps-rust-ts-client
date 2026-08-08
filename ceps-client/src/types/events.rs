//! Events-mode enums for CEP-18/85 and CEP-78.

use serde::{Deserialize, Serialize};

/// Events mode for CEP-18 and CEP-85 (no CEP-47 variant).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EventsMode {
    /// No events emitted.
    NoEvents = 0,
    /// Casper Event Standard (CES).
    Ces = 1,
    /// Native events.
    Native = 2,
    /// Native events as raw bytes.
    NativeBytes = 3,
}

impl EventsMode {
    /// Convert from the on-chain `u8` storage value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::NoEvents),
            1 => Some(Self::Ces),
            2 => Some(Self::Native),
            3 => Some(Self::NativeBytes),
            _ => None,
        }
    }

    /// Human-readable name matching the JS client.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoEvents => "NoEvents",
            Self::Ces => "CES",
            Self::Native => "Native",
            Self::NativeBytes => "NativeBytes",
        }
    }
}

impl From<EventsMode> for u8 {
    fn from(value: EventsMode) -> Self {
        value as u8
    }
}

/// Events mode for CEP-78 (includes CEP-47).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EventsMode78 {
    /// No events emitted.
    NoEvents = 0,
    /// CEP-47 style events.
    Cep47 = 1,
    /// Casper Event Standard (CES).
    Ces = 2,
    /// Native events.
    Native = 3,
    /// Native events as raw bytes.
    NativeBytes = 4,
}

impl EventsMode78 {
    /// Convert from the on-chain `u8` storage value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::NoEvents),
            1 => Some(Self::Cep47),
            2 => Some(Self::Ces),
            3 => Some(Self::Native),
            4 => Some(Self::NativeBytes),
            _ => None,
        }
    }

    /// Human-readable name matching the JS client.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoEvents => "NoEvents",
            Self::Cep47 => "CEP47",
            Self::Ces => "CES",
            Self::Native => "Native",
            Self::NativeBytes => "NativeBytes",
        }
    }
}

impl From<EventsMode78> for u8 {
    fn from(value: EventsMode78) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_mode_roundtrip() {
        assert_eq!(EventsMode::from_u8(1), Some(EventsMode::Ces));
        assert_eq!(u8::from(EventsMode::NativeBytes), 3);
        assert_eq!(EventsMode78::from_u8(1), Some(EventsMode78::Cep47));
        assert_eq!(EventsMode78::from_u8(2), Some(EventsMode78::Ces));
    }
}
