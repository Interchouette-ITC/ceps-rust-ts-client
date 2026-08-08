//! Ownership / metadata / minting modes for CEP-78 (`u8` ABI).

/// Who may transfer a token after mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OwnershipMode {
    /// Only the minter may transfer.
    Minter = 0,
    /// Owner is assigned; further transfer rules apply.
    Assigned = 1,
    /// Freely transferable.
    Transferable = 2,
}

/// Physical / digital / virtual classification stored on-chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NftKind {
    /// Physical asset.
    Physical = 0,
    /// Digital asset.
    Digital = 1,
    /// Virtual asset.
    Virtual = 2,
}

/// Whether accounts, contracts, or both may hold tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HolderMode {
    /// Account keys only.
    Accounts = 0,
    /// Contract keys only.
    Contracts = 1,
    /// Accounts and contracts.
    Mixed = 2,
}

/// Metadata schema stored for each token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NftMetadataKind {
    /// CEP-78 JSON schema.
    Cep78 = 0,
    /// NFT-721 style schema.
    Nft721 = 1,
    /// Opaque string payload.
    Raw = 2,
    /// Custom schema validated on-chain.
    CustomValidated = 3,
}

/// How tokens are identified (`token_id` vs `token_hash`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IdentifierMode {
    /// Sequential `u64` id.
    Ordinal = 0,
    /// Caller-supplied hash string.
    Hash = 1,
}

/// Whether `set_token_metadata` may change metadata after mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MetadataMutability {
    /// Metadata fixed at mint.
    Immutable = 0,
    /// Metadata may be updated.
    Mutable = 1,
}

/// Who is allowed to mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MintingMode {
    /// Installer only.
    Installer = 0,
    /// Anyone.
    Public = 1,
    /// ACL whitelist.
    Acl = 2,
}

/// Whether tokens may be burned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BurnMode {
    /// Burn entrypoint enabled.
    Burnable = 0,
    /// Burn entrypoint disabled.
    NonBurnable = 1,
}

/// Whether the ACL whitelist can still change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WhitelistMode {
    /// Whitelist can be edited.
    Unlocked = 0,
    /// Whitelist frozen.
    Locked = 1,
}

/// Owner page / reverse-lookup behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OwnerReverseLookupMode {
    /// No owner pages.
    NoLookup = 0,
    /// Full reverse lookup.
    Complete = 1,
    /// Track transfers only.
    TransfersOnly = 2,
}

/// How installer named keys are derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NamedKeyConventionMode {
    /// `cep78_contract_hash_{collection_name}` style.
    DerivedFromCollectionName = 0,
    /// Legacy v1.0 standard key names.
    V1_0Standard = 1,
    /// Requires `access_key_name` and `hash_key_name` at install.
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
