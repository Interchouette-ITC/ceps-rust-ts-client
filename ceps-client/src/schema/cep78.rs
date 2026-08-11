//! CEP-78 install and mutate arg fields.

use super::types::{ty, ArgField};

/// Installer session args.
pub const INSTALL: &[ArgField] = &[
    ArgField::req("collection_name", ty::STRING),
    ArgField::req("collection_symbol", ty::STRING),
    ArgField::req("total_token_supply", ty::U64),
    ArgField::req("ownership_mode", ty::U8),
    ArgField::req("nft_metadata_kind", ty::U8),
    ArgField::req("identifier_mode", ty::U8),
    ArgField::req("metadata_mutability", ty::U8),
    ArgField::opt("nft_kind", ty::U8),
    ArgField::opt("json_schema", ty::STRING),
    ArgField::opt("minting_mode", ty::U8),
    ArgField::opt("allow_minting", ty::BOOL),
    ArgField::opt("operator_burn_mode", ty::BOOL),
    ArgField::opt("package_operator_mode", ty::BOOL),
    ArgField::opt("whitelist_mode", ty::U8),
    ArgField::opt("holder_mode", ty::U8),
    ArgField::opt("acl_package_mode", ty::BOOL),
    ArgField::opt("acl_whitelist", ty::LIST_KEY),
    ArgField::opt("burn_mode", ty::U8),
    ArgField::opt("owner_reverse_lookup_mode", ty::U8),
    ArgField::opt("named_key_convention", ty::U8),
    ArgField::opt("access_key_name", ty::STRING),
    ArgField::opt("hash_key_name", ty::STRING),
    ArgField::opt("events_mode", ty::U8),
    ArgField::opt("transfer_filter_contract", ty::KEY),
];

/// Named install fields.
pub mod install {
    use super::ArgField;
    use super::INSTALL;

    /// `collection_name`
    pub const COLLECTION_NAME: ArgField = INSTALL[0];
    /// `collection_symbol`
    pub const COLLECTION_SYMBOL: ArgField = INSTALL[1];
    /// `total_token_supply`
    pub const TOTAL_TOKEN_SUPPLY: ArgField = INSTALL[2];
    /// `ownership_mode`
    pub const OWNERSHIP_MODE: ArgField = INSTALL[3];
    /// `nft_metadata_kind`
    pub const NFT_METADATA_KIND: ArgField = INSTALL[4];
    /// `identifier_mode`
    pub const IDENTIFIER_MODE: ArgField = INSTALL[5];
    /// `metadata_mutability`
    pub const METADATA_MUTABILITY: ArgField = INSTALL[6];
    /// `nft_kind`
    pub const NFT_KIND: ArgField = INSTALL[7];
    /// `json_schema`
    pub const JSON_SCHEMA: ArgField = INSTALL[8];
    /// `minting_mode`
    pub const MINTING_MODE: ArgField = INSTALL[9];
    /// `allow_minting`
    pub const ALLOW_MINTING: ArgField = INSTALL[10];
    /// `operator_burn_mode`
    pub const OPERATOR_BURN_MODE: ArgField = INSTALL[11];
    /// `package_operator_mode`
    pub const PACKAGE_OPERATOR_MODE: ArgField = INSTALL[12];
    /// `whitelist_mode`
    pub const WHITELIST_MODE: ArgField = INSTALL[13];
    /// `holder_mode`
    pub const HOLDER_MODE: ArgField = INSTALL[14];
    /// `acl_package_mode`
    pub const ACL_PACKAGE_MODE: ArgField = INSTALL[15];
    /// `acl_whitelist`
    pub const ACL_WHITELIST: ArgField = INSTALL[16];
    /// `burn_mode`
    pub const BURN_MODE: ArgField = INSTALL[17];
    /// `owner_reverse_lookup_mode`
    pub const OWNER_REVERSE_LOOKUP_MODE: ArgField = INSTALL[18];
    /// `named_key_convention`
    pub const NAMED_KEY_CONVENTION: ArgField = INSTALL[19];
    /// `access_key_name`
    pub const ACCESS_KEY_NAME: ArgField = INSTALL[20];
    /// `hash_key_name`
    pub const HASH_KEY_NAME: ArgField = INSTALL[21];
    /// `events_mode`
    pub const EVENTS_MODE: ArgField = INSTALL[22];
    /// `transfer_filter_contract`
    pub const TRANSFER_FILTER_CONTRACT: ArgField = INSTALL[23];
}

/// Token id-or-hash pair (mutually exclusive at call time).
pub const TOKEN_ID_OR_HASH: &[ArgField] = &[
    ArgField::opt("token_id", ty::U64),
    ArgField::opt("token_hash", ty::STRING),
];

/// Upgrade args.
pub const UPGRADE: &[ArgField] = &[
    ArgField::req("collection_name", ty::STRING),
    ArgField::opt("total_token_supply", ty::U64),
    ArgField::opt("events_mode", ty::U8),
    ArgField::opt("acl_package_mode", ty::BOOL),
    ArgField::opt("package_operator_mode", ty::BOOL),
    ArgField::opt("operator_burn_mode", ty::BOOL),
];

/// `mint`
pub const MINT: &[ArgField] = &[
    ArgField::req("token_owner", ty::KEY),
    ArgField::req("token_meta_data", ty::STRING),
    ArgField::opt("token_hash", ty::STRING),
];

/// `burn`
pub const BURN: &[ArgField] = TOKEN_ID_OR_HASH;

/// `transfer`
pub const TRANSFER: &[ArgField] = &[
    ArgField::req("source_key", ty::KEY),
    ArgField::req("target_key", ty::KEY),
    ArgField::opt("token_id", ty::U64),
    ArgField::opt("token_hash", ty::STRING),
];

/// `register_owner`
pub const REGISTER_OWNER: &[ArgField] = &[ArgField::req("token_owner", ty::KEY)];

/// `approve` / `revoke`
pub const APPROVE: &[ArgField] = &[
    ArgField::req("operator", ty::KEY),
    ArgField::opt("token_id", ty::U64),
    ArgField::opt("token_hash", ty::STRING),
];

/// `set_approval_for_all`
pub const SET_APPROVAL_FOR_ALL: &[ArgField] = &[
    ArgField::req("operator", ty::KEY),
    ArgField::req("approve_all", ty::BOOL),
];

/// `set_token_metadata`
pub const SET_TOKEN_METADATA: &[ArgField] = &[
    ArgField::req("token_meta_data", ty::STRING),
    ArgField::opt("token_id", ty::U64),
    ArgField::opt("token_hash", ty::STRING),
];

/// `set_variables`
pub const SET_VARIABLES: &[ArgField] = &[
    ArgField::opt("allow_minting", ty::BOOL),
    ArgField::opt("acl_whitelist", ty::LIST_KEY),
    ArgField::opt("acl_package_mode", ty::BOOL),
    ArgField::opt("package_operator_mode", ty::BOOL),
    ArgField::opt("operator_burn_mode", ty::BOOL),
];

/// Entrypoint tables.
pub const ENTRYPOINTS: &[(&str, &[ArgField])] = &[
    ("upgrade", UPGRADE),
    ("mint", MINT),
    ("burn", BURN),
    ("transfer", TRANSFER),
    ("register_owner", REGISTER_OWNER),
    ("approve", APPROVE),
    ("revoke", APPROVE),
    ("set_approval_for_all", SET_APPROVAL_FOR_ALL),
    ("set_token_metadata", SET_TOKEN_METADATA),
    ("set_variables", SET_VARIABLES),
];

/// Mutate field accessors.
pub mod ep {
    use super::ArgField;
    use super::{
        APPROVE, MINT, REGISTER_OWNER, SET_APPROVAL_FOR_ALL, SET_TOKEN_METADATA, SET_VARIABLES,
        TOKEN_ID_OR_HASH, TRANSFER, UPGRADE,
    };

    /// token_id
    pub const TOKEN_ID: ArgField = TOKEN_ID_OR_HASH[0];
    /// token_hash
    pub const TOKEN_HASH: ArgField = TOKEN_ID_OR_HASH[1];

    /// upgrade.collection_name
    pub const UPGRADE_COLLECTION_NAME: ArgField = UPGRADE[0];
    /// upgrade.total_token_supply
    pub const UPGRADE_TOTAL_TOKEN_SUPPLY: ArgField = UPGRADE[1];
    /// upgrade.events_mode
    pub const UPGRADE_EVENTS_MODE: ArgField = UPGRADE[2];
    /// upgrade.acl_package_mode
    pub const UPGRADE_ACL_PACKAGE_MODE: ArgField = UPGRADE[3];
    /// upgrade.package_operator_mode
    pub const UPGRADE_PACKAGE_OPERATOR_MODE: ArgField = UPGRADE[4];
    /// upgrade.operator_burn_mode
    pub const UPGRADE_OPERATOR_BURN_MODE: ArgField = UPGRADE[5];

    /// mint.token_owner
    pub const MINT_TOKEN_OWNER: ArgField = MINT[0];
    /// mint.token_meta_data
    pub const MINT_TOKEN_META_DATA: ArgField = MINT[1];
    /// mint.token_hash
    pub const MINT_TOKEN_HASH: ArgField = MINT[2];

    /// transfer.source_key
    pub const TRANSFER_SOURCE: ArgField = TRANSFER[0];
    /// transfer.target_key
    pub const TRANSFER_TARGET: ArgField = TRANSFER[1];

    /// register_owner.token_owner
    pub const REGISTER_OWNER_TOKEN_OWNER: ArgField = REGISTER_OWNER[0];

    /// approve.operator
    pub const APPROVE_OPERATOR: ArgField = APPROVE[0];

    /// set_approval_for_all.operator
    pub const SET_APPROVAL_OPERATOR: ArgField = SET_APPROVAL_FOR_ALL[0];
    /// set_approval_for_all.approve_all
    pub const SET_APPROVAL_APPROVE_ALL: ArgField = SET_APPROVAL_FOR_ALL[1];

    /// set_token_metadata.token_meta_data
    pub const SET_META_DATA: ArgField = SET_TOKEN_METADATA[0];

    /// set_variables.allow_minting
    pub const SV_ALLOW_MINTING: ArgField = SET_VARIABLES[0];
    /// set_variables.acl_whitelist
    pub const SV_ACL_WHITELIST: ArgField = SET_VARIABLES[1];
    /// set_variables.acl_package_mode
    pub const SV_ACL_PACKAGE_MODE: ArgField = SET_VARIABLES[2];
    /// set_variables.package_operator_mode
    pub const SV_PACKAGE_OPERATOR_MODE: ArgField = SET_VARIABLES[3];
    /// set_variables.operator_burn_mode
    pub const SV_OPERATOR_BURN_MODE: ArgField = SET_VARIABLES[4];
}
