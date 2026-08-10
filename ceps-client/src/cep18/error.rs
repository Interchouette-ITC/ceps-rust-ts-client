//! CEP-18 contract user-error codes (`60000+`).

use serde::{Deserialize, Serialize};

/// On-chain CEP-18 `ApiError` user codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum CEP18Error {
    /// Invalid calling context.
    InvalidContext = 60000,
    /// Insufficient token balance.
    InsufficientBalance = 60001,
    /// Insufficient allowance.
    InsufficientAllowance = 60002,
    /// Arithmetic overflow.
    Overflow = 60003,
    /// Package hash missing.
    PackageHashMissing = 60004,
    /// Package hash is not a package.
    PackageHashNotPackage = 60005,
    /// Invalid events mode.
    InvalidEventsMode = 60006,
    /// Events mode missing.
    MissingEventsMode = 60007,
    /// Phantom / unknown.
    Phantom = 60008,
    /// Failed to read arg bytes.
    FailedToGetArgBytes = 60009,
    /// Insufficient security rights.
    InsufficientRights = 60010,
    /// Invalid admin list.
    InvalidAdminList = 60011,
    /// Invalid minter list.
    InvalidMinterList = 60012,
    /// Invalid none list.
    InvalidNoneList = 60013,
    /// Invalid mint/burn enable flag.
    InvalidEnableMbFlag = 60014,
    /// Already initialized.
    AlreadyInitialized = 60015,
    /// Mint/burn disabled.
    MintBurnDisabled = 60016,
    /// Cannot target self.
    CannotTargetSelfUser = 60017,
    /// Invalid burn target.
    InvalidBurnTarget = 60018,
    /// Missing package hash for upgrade.
    MissingPackageHashForUpgrade = 60019,
    /// Missing contract hash for upgrade.
    MissingContractHashForUpgrade = 60020,
    /// Invalid key type.
    InvalidKeyType = 60021,
    /// JSON conversion failed.
    FailedToConvertToJson = 60022,
    /// Entry point result failed.
    FailedToReturnEntryPointResult = 60023,
    /// Dictionary create failed.
    FailedToCreateDictionary = 60024,
    /// Bytes conversion failed.
    FailedToConvertBytes = 60025,
    /// Total supply change failed.
    FailedToChangeTotalSupply = 60026,
    /// Storage read failed.
    FailedToReadFromStorage = 60027,
    /// Key retrieval failed.
    FailedToGetKey = 60028,
    /// Disable contract version failed.
    FailedToDisableContractVersion = 60029,
    /// Security list insert failed.
    FailedToInsertToSecurityList = 60030,
    /// URef not found.
    UrefNotFound = 60031,
    /// Old contract hash key missing.
    FailedToGetOldContractHashKey = 60032,
    /// Old package key missing.
    FailedToGetOldPackageKey = 60033,
    /// Package key missing.
    FailedToGetPackageKey = 60034,
    /// Storage uref missing.
    MissingStorageUref = 60035,
    /// Storage uref invalid.
    InvalidStorageUref = 60036,
    /// Version contract key missing.
    MissingVersionContractKey = 60037,
    /// Version contract key invalid.
    InvalidVersionContractKey = 60038,
}

impl CEP18Error {
    /// Map a user-error code to a typed variant.
    pub fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            60000 => Self::InvalidContext,
            60001 => Self::InsufficientBalance,
            60002 => Self::InsufficientAllowance,
            60003 => Self::Overflow,
            60004 => Self::PackageHashMissing,
            60005 => Self::PackageHashNotPackage,
            60006 => Self::InvalidEventsMode,
            60007 => Self::MissingEventsMode,
            60008 => Self::Phantom,
            60009 => Self::FailedToGetArgBytes,
            60010 => Self::InsufficientRights,
            60011 => Self::InvalidAdminList,
            60012 => Self::InvalidMinterList,
            60013 => Self::InvalidNoneList,
            60014 => Self::InvalidEnableMbFlag,
            60015 => Self::AlreadyInitialized,
            60016 => Self::MintBurnDisabled,
            60017 => Self::CannotTargetSelfUser,
            60018 => Self::InvalidBurnTarget,
            60019 => Self::MissingPackageHashForUpgrade,
            60020 => Self::MissingContractHashForUpgrade,
            60021 => Self::InvalidKeyType,
            60022 => Self::FailedToConvertToJson,
            60023 => Self::FailedToReturnEntryPointResult,
            60024 => Self::FailedToCreateDictionary,
            60025 => Self::FailedToConvertBytes,
            60026 => Self::FailedToChangeTotalSupply,
            60027 => Self::FailedToReadFromStorage,
            60028 => Self::FailedToGetKey,
            60029 => Self::FailedToDisableContractVersion,
            60030 => Self::FailedToInsertToSecurityList,
            60031 => Self::UrefNotFound,
            60032 => Self::FailedToGetOldContractHashKey,
            60033 => Self::FailedToGetOldPackageKey,
            60034 => Self::FailedToGetPackageKey,
            60035 => Self::MissingStorageUref,
            60036 => Self::InvalidStorageUref,
            60037 => Self::MissingVersionContractKey,
            60038 => Self::InvalidVersionContractKey,
            _ => return None,
        })
    }

    /// Stable name for docs and CLI.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidContext => "InvalidContext",
            Self::InsufficientBalance => "InsufficientBalance",
            Self::InsufficientAllowance => "InsufficientAllowance",
            Self::Overflow => "Overflow",
            Self::PackageHashMissing => "PackageHashMissing",
            Self::PackageHashNotPackage => "PackageHashNotPackage",
            Self::InvalidEventsMode => "InvalidEventsMode",
            Self::MissingEventsMode => "MissingEventsMode",
            Self::Phantom => "Phantom",
            Self::FailedToGetArgBytes => "FailedToGetArgBytes",
            Self::InsufficientRights => "InsufficientRights",
            Self::InvalidAdminList => "InvalidAdminList",
            Self::InvalidMinterList => "InvalidMinterList",
            Self::InvalidNoneList => "InvalidNoneList",
            Self::InvalidEnableMbFlag => "InvalidEnableMbFlag",
            Self::AlreadyInitialized => "AlreadyInitialized",
            Self::MintBurnDisabled => "MintBurnDisabled",
            Self::CannotTargetSelfUser => "CannotTargetSelfUser",
            Self::InvalidBurnTarget => "InvalidBurnTarget",
            Self::MissingPackageHashForUpgrade => "MissingPackageHashForUpgrade",
            Self::MissingContractHashForUpgrade => "MissingContractHashForUpgrade",
            Self::InvalidKeyType => "InvalidKeyType",
            Self::FailedToConvertToJson => "FailedToConvertToJson",
            Self::FailedToReturnEntryPointResult => "FailedToReturnEntryPointResult",
            Self::FailedToCreateDictionary => "FailedToCreateDictionary",
            Self::FailedToConvertBytes => "FailedToConvertBytes",
            Self::FailedToChangeTotalSupply => "FailedToChangeTotalSupply",
            Self::FailedToReadFromStorage => "FailedToReadFromStorage",
            Self::FailedToGetKey => "FailedToGetKey",
            Self::FailedToDisableContractVersion => "FailedToDisableContractVersion",
            Self::FailedToInsertToSecurityList => "FailedToInsertToSecurityList",
            Self::UrefNotFound => "UrefNotFound",
            Self::FailedToGetOldContractHashKey => "FailedToGetOldContractHashKey",
            Self::FailedToGetOldPackageKey => "FailedToGetOldPackageKey",
            Self::FailedToGetPackageKey => "FailedToGetPackageKey",
            Self::MissingStorageUref => "MissingStorageUref",
            Self::InvalidStorageUref => "InvalidStorageUref",
            Self::MissingVersionContractKey => "MissingVersionContractKey",
            Self::InvalidVersionContractKey => "InvalidVersionContractKey",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_codes() {
        assert_eq!(
            CEP18Error::from_code(60001),
            Some(CEP18Error::InsufficientBalance)
        );
        assert_eq!(CEP18Error::from_code(1), None);
    }
}
