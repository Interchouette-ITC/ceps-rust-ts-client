//! CEP-78 user error codes (subset used by the client).

/// Known CEP-78 user error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Cep78Error {
    /// Permission denied.
    PermissionDenied = 1,
    /// Invalid holder mode.
    InvalidHolderMode = 2,
    /// Invalid NFT kind.
    InvalidNFTKind = 3,
    /// Invalid metadata kind.
    InvalidNFTMetadataKind = 4,
    /// Invalid ownership mode.
    InvalidOwnershipMode = 5,
    /// Invalid minting mode.
    InvalidMintingMode = 6,
    /// Invalid identifier mode.
    InvalidIdentifierMode = 7,
    /// Invalid metadata mutability.
    InvalidMetadataMutability = 8,
    /// Token does not exist.
    UnknownTokenId = 28,
    /// Insufficient rights.
    InsufficientRights = 34,
}

impl Cep78Error {
    /// Map from on-chain user error code when known.
    pub fn from_user_code(code: u16) -> Option<Self> {
        Some(match code {
            1 => Self::PermissionDenied,
            2 => Self::InvalidHolderMode,
            3 => Self::InvalidNFTKind,
            4 => Self::InvalidNFTMetadataKind,
            5 => Self::InvalidOwnershipMode,
            6 => Self::InvalidMintingMode,
            7 => Self::InvalidIdentifierMode,
            8 => Self::InvalidMetadataMutability,
            28 => Self::UnknownTokenId,
            34 => Self::InsufficientRights,
            _ => return None,
        })
    }

    /// Stable name for logs.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PermissionDenied => "PermissionDenied",
            Self::InvalidHolderMode => "InvalidHolderMode",
            Self::InvalidNFTKind => "InvalidNFTKind",
            Self::InvalidNFTMetadataKind => "InvalidNFTMetadataKind",
            Self::InvalidOwnershipMode => "InvalidOwnershipMode",
            Self::InvalidMintingMode => "InvalidMintingMode",
            Self::InvalidIdentifierMode => "InvalidIdentifierMode",
            Self::InvalidMetadataMutability => "InvalidMetadataMutability",
            Self::UnknownTokenId => "UnknownTokenId",
            Self::InsufficientRights => "InsufficientRights",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_unknown_token() {
        assert_eq!(
            Cep78Error::from_user_code(28),
            Some(Cep78Error::UnknownTokenId)
        );
    }
}
