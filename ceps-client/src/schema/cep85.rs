//! CEP-85 install and mutate arg fields.

use super::types::{ty, ArgField};

/// Installer session args.
pub const INSTALL: &[ArgField] = &[
    ArgField::req("name", ty::STRING),
    ArgField::req("uri", ty::STRING),
    ArgField::opt("events_mode", ty::U8),
    ArgField::opt("enable_burn", ty::BOOL),
    ArgField::opt("admin_list", ty::LIST_KEY),
    ArgField::opt("minter_list", ty::LIST_KEY),
    ArgField::opt("burner_list", ty::LIST_KEY),
    ArgField::opt("meta_list", ty::LIST_KEY),
    ArgField::opt("transfer_filter_contract", ty::KEY),
    ArgField::opt("transfer_filter_method", ty::STRING),
];

/// Named install fields.
pub mod install {
    use super::ArgField;
    use super::INSTALL;

    /// `name`
    pub const NAME: ArgField = INSTALL[0];
    /// `uri`
    pub const URI: ArgField = INSTALL[1];
    /// `events_mode`
    pub const EVENTS_MODE: ArgField = INSTALL[2];
    /// `enable_burn`
    pub const ENABLE_BURN: ArgField = INSTALL[3];
    /// `admin_list`
    pub const ADMIN_LIST: ArgField = INSTALL[4];
    /// `minter_list`
    pub const MINTER_LIST: ArgField = INSTALL[5];
    /// `burner_list`
    pub const BURNER_LIST: ArgField = INSTALL[6];
    /// `meta_list`
    pub const META_LIST: ArgField = INSTALL[7];
    /// `transfer_filter_contract`
    pub const TRANSFER_FILTER_CONTRACT: ArgField = INSTALL[8];
    /// `transfer_filter_method`
    pub const TRANSFER_FILTER_METHOD: ArgField = INSTALL[9];
}

/// Upgrade args.
pub const UPGRADE: &[ArgField] = &[
    ArgField::req("name", ty::STRING),
    ArgField::req("upgrade", ty::BOOL),
    ArgField::opt("transfer_filter_contract", ty::KEY),
    ArgField::opt("transfer_filter_method", ty::STRING),
];

/// `mint`
pub const MINT: &[ArgField] = &[
    ArgField::req("recipient", ty::KEY),
    ArgField::req("id", ty::U256),
    ArgField::req("amount", ty::U256),
    ArgField::opt("uri", ty::STRING),
];

/// `batch_mint`
pub const BATCH_MINT: &[ArgField] = &[
    ArgField::req("recipient", ty::KEY),
    ArgField::req("ids", ty::LIST_U256),
    ArgField::req("amounts", ty::LIST_U256),
    ArgField::opt("uri", ty::STRING),
];

/// `burn`
pub const BURN: &[ArgField] = &[
    ArgField::req("owner", ty::KEY),
    ArgField::req("id", ty::U256),
    ArgField::req("amount", ty::U256),
];

/// `batch_burn`
pub const BATCH_BURN: &[ArgField] = &[
    ArgField::req("owner", ty::KEY),
    ArgField::req("ids", ty::LIST_U256),
    ArgField::req("amounts", ty::LIST_U256),
];

/// `transfer_from`
pub const TRANSFER_FROM: &[ArgField] = &[
    ArgField::req("from", ty::KEY),
    ArgField::req("to", ty::KEY),
    ArgField::req("id", ty::U256),
    ArgField::req("amount", ty::U256),
    ArgField::opt("data", ty::LIST_U8),
];

/// `batch_transfer_from`
pub const BATCH_TRANSFER_FROM: &[ArgField] = &[
    ArgField::req("from", ty::KEY),
    ArgField::req("to", ty::KEY),
    ArgField::req("ids", ty::LIST_U256),
    ArgField::req("amounts", ty::LIST_U256),
    ArgField::opt("data", ty::LIST_U8),
];

/// `set_approval_for_all`
pub const SET_APPROVAL_FOR_ALL: &[ArgField] = &[
    ArgField::req("operator", ty::KEY),
    ArgField::req("approved", ty::BOOL),
];

/// `set_uri`
pub const SET_URI: &[ArgField] = &[
    ArgField::req("uri", ty::STRING),
    ArgField::opt("id", ty::U256),
];

/// `set_total_supply_of`
pub const SET_TOTAL_SUPPLY_OF: &[ArgField] = &[
    ArgField::req("id", ty::U256),
    ArgField::req("total_supply", ty::U256),
];

/// `set_total_supply_of_batch`
pub const SET_TOTAL_SUPPLY_OF_BATCH: &[ArgField] = &[
    ArgField::req("ids", ty::LIST_U256),
    ArgField::req("total_supplies", ty::LIST_U256),
];

/// `change_security`
pub const CHANGE_SECURITY: &[ArgField] = &[
    ArgField::opt("admin_list", ty::LIST_KEY),
    ArgField::opt("minter_list", ty::LIST_KEY),
    ArgField::opt("burner_list", ty::LIST_KEY),
    ArgField::opt("meta_list", ty::LIST_KEY),
    ArgField::opt("none_list", ty::LIST_KEY),
];

/// `set_modalities`
pub const SET_MODALITIES: &[ArgField] = &[
    ArgField::opt("enable_burn", ty::BOOL),
    ArgField::opt("events_mode", ty::U8),
];

/// Entrypoint tables.
pub const ENTRYPOINTS: &[(&str, &[ArgField])] = &[
    ("upgrade", UPGRADE),
    ("mint", MINT),
    ("batch_mint", BATCH_MINT),
    ("burn", BURN),
    ("batch_burn", BATCH_BURN),
    ("transfer_from", TRANSFER_FROM),
    ("batch_transfer_from", BATCH_TRANSFER_FROM),
    ("set_approval_for_all", SET_APPROVAL_FOR_ALL),
    ("set_uri", SET_URI),
    ("set_total_supply_of", SET_TOTAL_SUPPLY_OF),
    ("set_total_supply_of_batch", SET_TOTAL_SUPPLY_OF_BATCH),
    ("change_security", CHANGE_SECURITY),
    ("set_modalities", SET_MODALITIES),
];

/// Mutate field accessors.
pub mod ep {
    use super::ArgField;
    use super::{
        BATCH_BURN, BATCH_MINT, BATCH_TRANSFER_FROM, BURN, CHANGE_SECURITY, MINT,
        SET_APPROVAL_FOR_ALL, SET_MODALITIES, SET_TOTAL_SUPPLY_OF, SET_TOTAL_SUPPLY_OF_BATCH,
        SET_URI, TRANSFER_FROM, UPGRADE,
    };

    /// upgrade.name
    pub const UPGRADE_NAME: ArgField = UPGRADE[0];
    /// upgrade.upgrade
    pub const UPGRADE_FLAG: ArgField = UPGRADE[1];
    /// upgrade.transfer_filter_contract
    pub const UPGRADE_FILTER_CONTRACT: ArgField = UPGRADE[2];
    /// upgrade.transfer_filter_method
    pub const UPGRADE_FILTER_METHOD: ArgField = UPGRADE[3];

    /// mint.recipient
    pub const MINT_RECIPIENT: ArgField = MINT[0];
    /// mint.id
    pub const MINT_ID: ArgField = MINT[1];
    /// mint.amount
    pub const MINT_AMOUNT: ArgField = MINT[2];
    /// mint.uri
    pub const MINT_URI: ArgField = MINT[3];

    /// batch_mint.recipient
    pub const BATCH_MINT_RECIPIENT: ArgField = BATCH_MINT[0];
    /// batch_mint.ids
    pub const BATCH_MINT_IDS: ArgField = BATCH_MINT[1];
    /// batch_mint.amounts
    pub const BATCH_MINT_AMOUNTS: ArgField = BATCH_MINT[2];
    /// batch_mint.uri
    pub const BATCH_MINT_URI: ArgField = BATCH_MINT[3];

    /// burn.owner
    pub const BURN_OWNER: ArgField = BURN[0];
    /// burn.id
    pub const BURN_ID: ArgField = BURN[1];
    /// burn.amount
    pub const BURN_AMOUNT: ArgField = BURN[2];

    /// batch_burn.owner
    pub const BATCH_BURN_OWNER: ArgField = BATCH_BURN[0];
    /// batch_burn.ids
    pub const BATCH_BURN_IDS: ArgField = BATCH_BURN[1];
    /// batch_burn.amounts
    pub const BATCH_BURN_AMOUNTS: ArgField = BATCH_BURN[2];

    /// transfer_from.from
    pub const TF_FROM: ArgField = TRANSFER_FROM[0];
    /// transfer_from.to
    pub const TF_TO: ArgField = TRANSFER_FROM[1];
    /// transfer_from.id
    pub const TF_ID: ArgField = TRANSFER_FROM[2];
    /// transfer_from.amount
    pub const TF_AMOUNT: ArgField = TRANSFER_FROM[3];
    /// transfer_from.data
    pub const TF_DATA: ArgField = TRANSFER_FROM[4];

    /// batch_transfer_from.from
    pub const BTF_FROM: ArgField = BATCH_TRANSFER_FROM[0];
    /// batch_transfer_from.to
    pub const BTF_TO: ArgField = BATCH_TRANSFER_FROM[1];
    /// batch_transfer_from.ids
    pub const BTF_IDS: ArgField = BATCH_TRANSFER_FROM[2];
    /// batch_transfer_from.amounts
    pub const BTF_AMOUNTS: ArgField = BATCH_TRANSFER_FROM[3];
    /// batch_transfer_from.data
    pub const BTF_DATA: ArgField = BATCH_TRANSFER_FROM[4];

    /// set_approval_for_all.operator
    pub const APPROVAL_OPERATOR: ArgField = SET_APPROVAL_FOR_ALL[0];
    /// set_approval_for_all.approved
    pub const APPROVAL_APPROVED: ArgField = SET_APPROVAL_FOR_ALL[1];

    /// set_uri.uri
    pub const SET_URI_URI: ArgField = SET_URI[0];
    /// set_uri.id
    pub const SET_URI_ID: ArgField = SET_URI[1];

    /// set_total_supply_of.id
    pub const STS_ID: ArgField = SET_TOTAL_SUPPLY_OF[0];
    /// set_total_supply_of.total_supply
    pub const STS_TOTAL_SUPPLY: ArgField = SET_TOTAL_SUPPLY_OF[1];

    /// set_total_supply_of_batch.ids
    pub const STSB_IDS: ArgField = SET_TOTAL_SUPPLY_OF_BATCH[0];
    /// set_total_supply_of_batch.total_supplies
    pub const STSB_TOTAL_SUPPLIES: ArgField = SET_TOTAL_SUPPLY_OF_BATCH[1];

    /// change_security.admin_list
    pub const CS_ADMIN_LIST: ArgField = CHANGE_SECURITY[0];
    /// change_security.minter_list
    pub const CS_MINTER_LIST: ArgField = CHANGE_SECURITY[1];
    /// change_security.burner_list
    pub const CS_BURNER_LIST: ArgField = CHANGE_SECURITY[2];
    /// change_security.meta_list
    pub const CS_META_LIST: ArgField = CHANGE_SECURITY[3];
    /// change_security.none_list
    pub const CS_NONE_LIST: ArgField = CHANGE_SECURITY[4];

    /// set_modalities.enable_burn
    pub const SM_ENABLE_BURN: ArgField = SET_MODALITIES[0];
    /// set_modalities.events_mode
    pub const SM_EVENTS_MODE: ArgField = SET_MODALITIES[1];
}
