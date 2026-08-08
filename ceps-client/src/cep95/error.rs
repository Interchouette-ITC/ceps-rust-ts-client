//! CEP-95 / Odra tip user-error codes.

use serde::{Deserialize, Serialize};

/// On-chain CEP-95 / Ownable error discriminants from Odra modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum Cep95Error {
    /// Ownable: owner not set.
    OwnerNotSet = 20000,
    /// Ownable: caller is not the owner.
    CallerNotTheOwner = 20001,
    /// Ownable: caller is not the new owner.
    CallerNotTheNewOwner = 20002,
    /// Ownable: missing role.
    MissingRole = 20003,
    /// Ownable: cannot renounce role for another address.
    RoleRenounceForAnotherAddress = 20004,
    /// Value not set.
    ValueNotSet = 40000,
    /// Transfer failed.
    TransferFailed = 40001,
    /// Not an owner or approved.
    NotAnOwnerOrApproved = 40002,
    /// Approval set to current owner.
    ApprovalToCurrentOwner = 40003,
    /// Approve to caller.
    ApproveToCaller = 40004,
    /// Invalid token id.
    InvalidTokenId = 40005,
    /// Token already exists.
    TokenAlreadyExists = 40006,
}

impl Cep95Error {
    /// Map a user-error code to a typed variant.
    pub fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            20000 => Self::OwnerNotSet,
            20001 => Self::CallerNotTheOwner,
            20002 => Self::CallerNotTheNewOwner,
            20003 => Self::MissingRole,
            20004 => Self::RoleRenounceForAnotherAddress,
            40000 => Self::ValueNotSet,
            40001 => Self::TransferFailed,
            40002 => Self::NotAnOwnerOrApproved,
            40003 => Self::ApprovalToCurrentOwner,
            40004 => Self::ApproveToCaller,
            40005 => Self::InvalidTokenId,
            40006 => Self::TokenAlreadyExists,
            _ => return None,
        })
    }

    /// Stable name for docs and CLI.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OwnerNotSet => "OwnerNotSet",
            Self::CallerNotTheOwner => "CallerNotTheOwner",
            Self::CallerNotTheNewOwner => "CallerNotTheNewOwner",
            Self::MissingRole => "MissingRole",
            Self::RoleRenounceForAnotherAddress => "RoleRenounceForAnotherAddress",
            Self::ValueNotSet => "ValueNotSet",
            Self::TransferFailed => "TransferFailed",
            Self::NotAnOwnerOrApproved => "NotAnOwnerOrApproved",
            Self::ApprovalToCurrentOwner => "ApprovalToCurrentOwner",
            Self::ApproveToCaller => "ApproveToCaller",
            Self::InvalidTokenId => "InvalidTokenId",
            Self::TokenAlreadyExists => "TokenAlreadyExists",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_codes() {
        assert_eq!(
            Cep95Error::from_code(40006),
            Some(Cep95Error::TokenAlreadyExists)
        );
        assert_eq!(Cep95Error::from_code(1), None);
    }
}
