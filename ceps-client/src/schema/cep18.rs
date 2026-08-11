//! CEP-18 install and mutate arg fields.

use super::types::{ty, ArgField};

/// Installer session args.
pub const INSTALL: &[ArgField] = &[
    ArgField::req("name", ty::STRING),
    ArgField::req("symbol", ty::STRING),
    ArgField::req("decimals", ty::U8),
    ArgField::req("total_supply", ty::U256),
    ArgField::opt("events_mode", ty::U8),
    ArgField::opt("enable_mint_burn", ty::U8),
    ArgField::opt("admin_list", ty::LIST_KEY),
    ArgField::opt("minter_list", ty::LIST_KEY),
];

/// Named install fields (SoT for builders).
pub mod install {
    use super::ArgField;
    use super::INSTALL;

    /// `name`
    pub const NAME: ArgField = INSTALL[0];
    /// `symbol`
    pub const SYMBOL: ArgField = INSTALL[1];
    /// `decimals`
    pub const DECIMALS: ArgField = INSTALL[2];
    /// `total_supply`
    pub const TOTAL_SUPPLY: ArgField = INSTALL[3];
    /// `events_mode`
    pub const EVENTS_MODE: ArgField = INSTALL[4];
    /// `enable_mint_burn`
    pub const ENABLE_MINT_BURN: ArgField = INSTALL[5];
    /// `admin_list`
    pub const ADMIN_LIST: ArgField = INSTALL[6];
    /// `minter_list`
    pub const MINTER_LIST: ArgField = INSTALL[7];
}

/// Upgrade session args.
pub const UPGRADE: &[ArgField] = &[
    ArgField::req("name", ty::STRING),
    ArgField::opt("events_mode", ty::U8),
];

/// `transfer`
pub const TRANSFER: &[ArgField] = &[
    ArgField::req("recipient", ty::KEY),
    ArgField::req("amount", ty::U256),
];

/// `transfer_from`
pub const TRANSFER_FROM: &[ArgField] = &[
    ArgField::req("owner", ty::KEY),
    ArgField::req("recipient", ty::KEY),
    ArgField::req("amount", ty::U256),
];

/// `approve` / allowance mutates share spender+amount.
pub const APPROVE: &[ArgField] = &[
    ArgField::req("spender", ty::KEY),
    ArgField::req("amount", ty::U256),
];

/// `mint` / `burn`
pub const MINT_BURN: &[ArgField] = &[
    ArgField::req("owner", ty::KEY),
    ArgField::req("amount", ty::U256),
];

/// `change_security`
pub const CHANGE_SECURITY: &[ArgField] = &[
    ArgField::opt("admin_list", ty::LIST_KEY),
    ArgField::opt("minter_list", ty::LIST_KEY),
    ArgField::opt("none_list", ty::LIST_KEY),
];

/// `change_events_mode`
pub const CHANGE_EVENTS_MODE: &[ArgField] = &[ArgField::req("events_mode", ty::U8)];

/// Entrypoint tables for export.
pub const ENTRYPOINTS: &[(&str, &[ArgField])] = &[
    ("upgrade", UPGRADE),
    ("transfer", TRANSFER),
    ("transfer_from", TRANSFER_FROM),
    ("approve", APPROVE),
    ("increase_allowance", APPROVE),
    ("decrease_allowance", APPROVE),
    ("mint", MINT_BURN),
    ("burn", MINT_BURN),
    ("change_security", CHANGE_SECURITY),
    ("change_events_mode", CHANGE_EVENTS_MODE),
];

/// Field accessors for mutate builders.
pub mod ep {
    use super::ArgField;
    use super::{
        APPROVE, CHANGE_EVENTS_MODE, CHANGE_SECURITY, MINT_BURN, TRANSFER, TRANSFER_FROM, UPGRADE,
    };

    /// upgrade.name
    pub const UPGRADE_NAME: ArgField = UPGRADE[0];
    /// upgrade.events_mode
    pub const UPGRADE_EVENTS_MODE: ArgField = UPGRADE[1];

    /// transfer.recipient
    pub const TRANSFER_RECIPIENT: ArgField = TRANSFER[0];
    /// transfer.amount
    pub const TRANSFER_AMOUNT: ArgField = TRANSFER[1];

    /// transfer_from.owner
    pub const TRANSFER_FROM_OWNER: ArgField = TRANSFER_FROM[0];
    /// transfer_from.recipient
    pub const TRANSFER_FROM_RECIPIENT: ArgField = TRANSFER_FROM[1];
    /// transfer_from.amount
    pub const TRANSFER_FROM_AMOUNT: ArgField = TRANSFER_FROM[2];

    /// approve.spender
    pub const APPROVE_SPENDER: ArgField = APPROVE[0];
    /// approve.amount
    pub const APPROVE_AMOUNT: ArgField = APPROVE[1];

    /// mint/burn.owner
    pub const MINT_BURN_OWNER: ArgField = MINT_BURN[0];
    /// mint/burn.amount
    pub const MINT_BURN_AMOUNT: ArgField = MINT_BURN[1];

    /// change_security.admin_list
    pub const CS_ADMIN_LIST: ArgField = CHANGE_SECURITY[0];
    /// change_security.minter_list
    pub const CS_MINTER_LIST: ArgField = CHANGE_SECURITY[1];
    /// change_security.none_list
    pub const CS_NONE_LIST: ArgField = CHANGE_SECURITY[2];

    /// change_events_mode.events_mode
    pub const CHANGE_EVENTS_MODE_EVENTS: ArgField = CHANGE_EVENTS_MODE[0];
}
