//! CEP-78 mode enums (u8 ABI).

/// Ownership mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OwnershipMode {
    /// Only minter may transfer.
    Minter = 0,
    /// Assigned owner; may not transfer further unless rules allow.
    Assigned = 1,
    /// Freely transferable.
    Transferable = 2,
}

/// NFT kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NftKind {
    /// Physical.
    Physical = 0,
    /// Digital.
    Digital = 1,
    /// Virtual.
    Virtual = 2,
}

/// Who may hold tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HolderMode {
    /// Accounts only.
    Accounts = 0,
    /// Contracts only.
    Contracts = 1,
    /// Mixed.
    Mixed = 2,
}

/// On-chain metadata schema kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NftMetadataKind {
    /// CEP-78 schema.
    Cep78 = 0,
    /// NFT721 schema.
    Nft721 = 1,
    /// Raw string.
    Raw = 2,
    /// Custom validated JSON schema.
    CustomValidated = 3,
}

/// Token identifier mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IdentifierMode {
    /// Ordinal `token_id` (u64).
    Ordinal = 0,
    /// Hash string identifier.
    Hash = 1,
}

/// Whether metadata may change after mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MetadataMutability {
    /// Immutable after mint.
    Immutable = 0,
    /// Mutable via `set_token_metadata`.
    Mutable = 1,
}

/// Who may mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MintingMode {
    /// Installer only.
    Installer = 0,
    /// Public mint.
    Public = 1,
    /// ACL whitelist.
    Acl = 2,
}

/// Burnability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BurnMode {
    /// Burnable.
    Burnable = 0,
    /// Non-burnable.
    NonBurnable = 1,
}

/// ACL whitelist lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WhitelistMode {
    /// Unlocked.
    Unlocked = 0,
    /// Locked.
    Locked = 1,
}

/// Owner reverse-lookup / page receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OwnerReverseLookupMode {
    /// No lookup.
    NoLookup = 0,
    /// Complete.
    Complete = 1,
    /// Transfers only.
    TransfersOnly = 2,
}

/// Named-key naming convention at install.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NamedKeyConventionMode {
    /// `cep78_contract_hash_{collection_name}` style.
    DerivedFromCollectionName = 0,
    /// Legacy v1.0 standard names.
    V1_0Standard = 1,
    /// Caller-supplied access/hash key names.
    V1_0Custom = 2,
}

macro_rules! impl_mode_u8 {
    ($ty:ty) => {
        impl From<$ty> for u8 {
            fn from(value: $ty) -> Self {
                value as u8
            }
        }
    };
}

impl_mode_u8!(OwnershipMode);
impl_mode_u8!(NftKind);
impl_mode_u8!(HolderMode);
impl_mode_u8!(NftMetadataKind);
impl_mode_u8!(IdentifierMode);
impl_mode_u8!(MetadataMutability);
impl_mode_u8!(MintingMode);
impl_mode_u8!(BurnMode);
impl_mode_u8!(WhitelistMode);
impl_mode_u8!(OwnerReverseLookupMode);
impl_mode_u8!(NamedKeyConventionMode);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_transferable_is_two() {
        assert_eq!(u8::from(OwnershipMode::Transferable), 2);
        assert_eq!(u8::from(IdentifierMode::Ordinal), 0);
        assert_eq!(u8::from(NftMetadataKind::Raw), 2);
    }
}
