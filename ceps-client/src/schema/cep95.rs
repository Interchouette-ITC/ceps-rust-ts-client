//! CEP-95 install and mutate arg fields.

use super::types::{ty, ArgField};

/// Installer session args (Odra cfg + collection).
pub const INSTALL: &[ArgField] = &[
    ArgField::req("odra_cfg_package_hash_key_name", ty::STRING),
    ArgField::req("odra_cfg_allow_key_override", ty::BOOL),
    ArgField::req("odra_cfg_is_upgradable", ty::BOOL),
    ArgField::req("odra_cfg_is_upgrade", ty::BOOL),
    ArgField::req("name", ty::STRING),
    ArgField::req("symbol", ty::STRING),
];

/// Named install fields.
pub mod install {
    use super::ArgField;
    use super::INSTALL;

    /// `odra_cfg_package_hash_key_name`
    pub const PACKAGE_HASH_KEY_NAME: ArgField = INSTALL[0];
    /// `odra_cfg_allow_key_override`
    pub const ALLOW_KEY_OVERRIDE: ArgField = INSTALL[1];
    /// `odra_cfg_is_upgradable`
    pub const IS_UPGRADABLE: ArgField = INSTALL[2];
    /// `odra_cfg_is_upgrade`
    pub const IS_UPGRADE: ArgField = INSTALL[3];
    /// `name`
    pub const NAME: ArgField = INSTALL[4];
    /// `symbol`
    pub const SYMBOL: ArgField = INSTALL[5];
}

/// `transfer_from`
pub const TRANSFER_FROM: &[ArgField] = &[
    ArgField::req("from", ty::KEY),
    ArgField::req("to", ty::KEY),
    ArgField::req("token_id", ty::U256),
];

/// `safe_transfer_from`
pub const SAFE_TRANSFER_FROM: &[ArgField] = &[
    ArgField::req("from", ty::KEY),
    ArgField::req("to", ty::KEY),
    ArgField::req("token_id", ty::U256),
    ArgField::req("data", ty::OPTION_LIST_U8),
];

/// `approve`
pub const APPROVE: &[ArgField] = &[
    ArgField::req("spender", ty::KEY),
    ArgField::req("token_id", ty::U256),
];

/// `revoke_approval`
pub const REVOKE_APPROVAL: &[ArgField] = &[ArgField::req("token_id", ty::U256)];

/// `approve_for_all` / `revoke_approval_for_all`
pub const APPROVE_FOR_ALL: &[ArgField] = &[ArgField::req("operator", ty::KEY)];

/// `mint`
pub const MINT: &[ArgField] = &[
    ArgField::req("to", ty::KEY),
    ArgField::req("token_id", ty::U256),
    ArgField::req("metadata", ty::LIST_STRING_PAIR),
];

/// `burn`
pub const BURN: &[ArgField] = &[ArgField::req("token_id", ty::U256)];

/// `transfer_ownership`
pub const TRANSFER_OWNERSHIP: &[ArgField] = &[ArgField::req("new_owner", ty::KEY)];

/// Entrypoint tables.
pub const ENTRYPOINTS: &[(&str, &[ArgField])] = &[
    ("transfer_from", TRANSFER_FROM),
    ("safe_transfer_from", SAFE_TRANSFER_FROM),
    ("approve", APPROVE),
    ("revoke_approval", REVOKE_APPROVAL),
    ("approve_for_all", APPROVE_FOR_ALL),
    ("revoke_approval_for_all", APPROVE_FOR_ALL),
    ("mint", MINT),
    ("burn", BURN),
    ("transfer_ownership", TRANSFER_OWNERSHIP),
];

/// Mutate field accessors.
pub mod ep {
    use super::ArgField;
    use super::{
        APPROVE, APPROVE_FOR_ALL, BURN, MINT, REVOKE_APPROVAL, SAFE_TRANSFER_FROM, TRANSFER_FROM,
        TRANSFER_OWNERSHIP,
    };

    /// transfer_from.from
    pub const TF_FROM: ArgField = TRANSFER_FROM[0];
    /// transfer_from.to
    pub const TF_TO: ArgField = TRANSFER_FROM[1];
    /// transfer_from.token_id
    pub const TF_TOKEN_ID: ArgField = TRANSFER_FROM[2];

    /// safe_transfer_from.from
    pub const STF_FROM: ArgField = SAFE_TRANSFER_FROM[0];
    /// safe_transfer_from.to
    pub const STF_TO: ArgField = SAFE_TRANSFER_FROM[1];
    /// safe_transfer_from.token_id
    pub const STF_TOKEN_ID: ArgField = SAFE_TRANSFER_FROM[2];
    /// safe_transfer_from.data
    pub const STF_DATA: ArgField = SAFE_TRANSFER_FROM[3];

    /// approve.spender
    pub const APPROVE_SPENDER: ArgField = APPROVE[0];
    /// approve.token_id
    pub const APPROVE_TOKEN_ID: ArgField = APPROVE[1];

    /// revoke_approval.token_id
    pub const REVOKE_TOKEN_ID: ArgField = REVOKE_APPROVAL[0];

    /// approve_for_all.operator
    pub const OPERATOR: ArgField = APPROVE_FOR_ALL[0];

    /// mint.to
    pub const MINT_TO: ArgField = MINT[0];
    /// mint.token_id
    pub const MINT_TOKEN_ID: ArgField = MINT[1];
    /// mint.metadata
    pub const MINT_METADATA: ArgField = MINT[2];

    /// burn.token_id
    pub const BURN_TOKEN_ID: ArgField = BURN[0];

    /// transfer_ownership.new_owner
    pub const NEW_OWNER: ArgField = TRANSFER_OWNERSHIP[0];
}
