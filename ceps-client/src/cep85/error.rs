//! CEP-85 user-error codes (`1..=91`).

use serde::{Deserialize, Serialize};

/// On-chain CEP-85 `CEP85Error` user codes (subset of common ones; full range 1..=91).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum CEP85Error {
    /// Burn disabled.
    BurnDisabled = 1,
    /// Insufficient balance.
    InsufficientBalance = 2,
    /// Insufficient rights.
    InsufficientRights = 3,
    /// Overflow.
    Overflow = 4,
    /// Not approved.
    NotApproved = 5,
    /// Self transfer.
    SelfTransfer = 6,
    /// Self operator approval.
    SelfOperatorApproval = 7,
}

impl CEP85Error {
    /// Map a known code when it matches a listed variant.
    pub fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            1 => Self::BurnDisabled,
            2 => Self::InsufficientBalance,
            3 => Self::InsufficientRights,
            4 => Self::Overflow,
            5 => Self::NotApproved,
            6 => Self::SelfTransfer,
            7 => Self::SelfOperatorApproval,
            _ => return None,
        })
    }

    /// Stable name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BurnDisabled => "BurnDisabled",
            Self::InsufficientBalance => "InsufficientBalance",
            Self::InsufficientRights => "InsufficientRights",
            Self::Overflow => "Overflow",
            Self::NotApproved => "NotApproved",
            Self::SelfTransfer => "SelfTransfer",
            Self::SelfOperatorApproval => "SelfOperatorApproval",
        }
    }
}
