//! CEP-78 install / upgrade / security argument types.

use super::modes::{
    BurnMode, HolderMode, IdentifierMode, MetadataMutability, MintingMode, NamedKeyConventionMode,
    NftKind, NftMetadataKind, OwnerReverseLookupMode, OwnershipMode, WhitelistMode,
};
use crate::types::EventsMode78;

/// Token identifier for mutate / query helpers.
#[derive(Debug, Clone)]
pub enum TokenIdentifier {
    /// Ordinal mode.
    Id(u64),
    /// Hash mode.
    Hash(String),
}

impl TokenIdentifier {
    /// Ordinal id.
    pub fn id(id: u64) -> Self {
        Self::Id(id)
    }

    /// Hash string.
    pub fn hash(hash: impl Into<String>) -> Self {
        Self::Hash(hash.into())
    }
}

/// Install arguments for CEP-78.
#[derive(Debug, Clone)]
pub struct InstallArgs {
    /// Collection name.
    pub collection_name: String,
    /// Collection symbol.
    pub collection_symbol: String,
    /// Total token supply.
    pub total_token_supply: u64,
    /// Ownership mode.
    pub ownership_mode: OwnershipMode,
    /// NFT metadata kind.
    pub nft_metadata_kind: NftMetadataKind,
    /// Identifier mode.
    pub identifier_mode: IdentifierMode,
    /// Metadata mutability.
    pub metadata_mutability: MetadataMutability,
    /// Optional NFT kind (defaults on-chain when omitted).
    pub nft_kind: Option<NftKind>,
    /// Optional JSON schema string (CustomValidated).
    pub json_schema: Option<String>,
    /// Optional minting mode.
    pub minting_mode: Option<MintingMode>,
    /// Optional allow_minting flag.
    pub allow_minting: Option<bool>,
    /// Optional operator burn mode.
    pub operator_burn_mode: Option<bool>,
    /// Optional package operator mode.
    pub package_operator_mode: Option<bool>,
    /// Optional whitelist mode.
    pub whitelist_mode: Option<WhitelistMode>,
    /// Optional holder mode.
    pub holder_mode: Option<HolderMode>,
    /// Optional ACL package mode.
    pub acl_package_mode: Option<bool>,
    /// Optional ACL whitelist keys.
    pub acl_whitelist: Option<Vec<String>>,
    /// Optional burn mode.
    pub burn_mode: Option<BurnMode>,
    /// Optional owner reverse lookup mode.
    pub owner_reverse_lookup_mode: Option<OwnerReverseLookupMode>,
    /// Optional named key convention.
    pub named_key_convention: Option<NamedKeyConventionMode>,
    /// Custom access key name (V1_0Custom).
    pub access_key_name: Option<String>,
    /// Custom hash key name (V1_0Custom).
    pub hash_key_name: Option<String>,
    /// Optional events mode.
    pub events_mode: Option<EventsMode78>,
    /// Optional transfer filter contract key.
    pub transfer_filter_contract: Option<String>,
}

impl InstallArgs {
    /// Required fields constructor with sensible defaults for a transferable ordinal collection.
    pub fn new(
        collection_name: impl Into<String>,
        collection_symbol: impl Into<String>,
        total_token_supply: u64,
    ) -> Self {
        Self {
            collection_name: collection_name.into(),
            collection_symbol: collection_symbol.into(),
            total_token_supply,
            ownership_mode: OwnershipMode::Transferable,
            nft_metadata_kind: NftMetadataKind::Raw,
            identifier_mode: IdentifierMode::Ordinal,
            metadata_mutability: MetadataMutability::Immutable,
            nft_kind: Some(NftKind::Digital),
            json_schema: None,
            minting_mode: Some(MintingMode::Installer),
            allow_minting: None,
            operator_burn_mode: None,
            package_operator_mode: None,
            whitelist_mode: None,
            holder_mode: Some(HolderMode::Accounts),
            acl_package_mode: None,
            acl_whitelist: None,
            burn_mode: Some(BurnMode::Burnable),
            owner_reverse_lookup_mode: Some(OwnerReverseLookupMode::NoLookup),
            named_key_convention: None,
            access_key_name: None,
            hash_key_name: None,
            events_mode: None,
            transfer_filter_contract: None,
        }
    }

    /// Set ownership mode.
    pub fn with_ownership_mode(mut self, mode: OwnershipMode) -> Self {
        self.ownership_mode = mode;
        self
    }

    /// Set metadata kind.
    pub fn with_nft_metadata_kind(mut self, kind: NftMetadataKind) -> Self {
        self.nft_metadata_kind = kind;
        self
    }

    /// Set identifier mode.
    pub fn with_identifier_mode(mut self, mode: IdentifierMode) -> Self {
        self.identifier_mode = mode;
        self
    }

    /// Set metadata mutability.
    pub fn with_metadata_mutability(mut self, mode: MetadataMutability) -> Self {
        self.metadata_mutability = mode;
        self
    }

    /// Set events mode.
    pub fn with_events_mode(mut self, mode: EventsMode78) -> Self {
        self.events_mode = Some(mode);
        self
    }

    /// Set minting mode.
    pub fn with_minting_mode(mut self, mode: MintingMode) -> Self {
        self.minting_mode = Some(mode);
        self
    }

    /// Set burn mode.
    pub fn with_burn_mode(mut self, mode: BurnMode) -> Self {
        self.burn_mode = Some(mode);
        self
    }

    /// Set holder mode.
    pub fn with_holder_mode(mut self, mode: HolderMode) -> Self {
        self.holder_mode = Some(mode);
        self
    }

    /// Set owner reverse lookup mode.
    pub fn with_owner_reverse_lookup_mode(mut self, mode: OwnerReverseLookupMode) -> Self {
        self.owner_reverse_lookup_mode = Some(mode);
        self
    }
}

/// Upgrade arguments.
#[derive(Debug, Clone)]
pub struct UpgradeArgs {
    /// Collection name (must match installed).
    pub collection_name: String,
    /// Optional new total supply.
    pub total_token_supply: Option<u64>,
    /// Optional events mode.
    pub events_mode: Option<EventsMode78>,
    /// Optional ACL package mode.
    pub acl_package_mode: Option<bool>,
    /// Optional package operator mode.
    pub package_operator_mode: Option<bool>,
    /// Optional operator burn mode.
    pub operator_burn_mode: Option<bool>,
}

impl UpgradeArgs {
    /// Create upgrade args for `collection_name`.
    pub fn new(collection_name: impl Into<String>) -> Self {
        Self {
            collection_name: collection_name.into(),
            total_token_supply: None,
            events_mode: None,
            acl_package_mode: None,
            package_operator_mode: None,
            operator_burn_mode: None,
        }
    }
}

/// Arguments for `set_variables`.
#[derive(Debug, Clone, Default)]
pub struct SetVariablesArgs {
    /// Allow minting flag.
    pub allow_minting: Option<bool>,
    /// ACL whitelist keys.
    pub acl_whitelist: Option<Vec<String>>,
    /// ACL package mode.
    pub acl_package_mode: Option<bool>,
    /// Package operator mode.
    pub package_operator_mode: Option<bool>,
    /// Operator burn mode.
    pub operator_burn_mode: Option<bool>,
}
