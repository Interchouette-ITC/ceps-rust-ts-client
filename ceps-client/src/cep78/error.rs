//! CEP-78 user error codes (on-chain `NFTCoreError`, codes 1..=180).

/// On-chain CEP-78 user error discriminants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CEP78Error {
    /// `InvalidAccount` (user error 1).
    InvalidAccount = 1,
    /// `MissingInstaller` (user error 2).
    MissingInstaller = 2,
    /// `InvalidInstaller` (user error 3).
    InvalidInstaller = 3,
    /// `UnexpectedKeyVariant` (user error 4).
    UnexpectedKeyVariant = 4,
    /// `MissingTokenOwner` (user error 5).
    MissingTokenOwner = 5,
    /// `InvalidTokenOwner` (user error 6).
    InvalidTokenOwner = 6,
    /// `FailedToGetArgBytes` (user error 7).
    FailedToGetArgBytes = 7,
    /// `FailedToCreateDictionary` (user error 8).
    FailedToCreateDictionary = 8,
    /// `MissingStorageUref` (user error 9).
    MissingStorageUref = 9,
    /// `InvalidStorageUref` (user error 10).
    InvalidStorageUref = 10,
    /// `MissingOwnersUref` (user error 11).
    MissingOwnersUref = 11,
    /// `InvalidOwnersUref` (user error 12).
    InvalidOwnersUref = 12,
    /// `FailedToAccessStorageDictionary` (user error 13).
    FailedToAccessStorageDictionary = 13,
    /// `FailedToAccessOwnershipDictionary` (user error 14).
    FailedToAccessOwnershipDictionary = 14,
    /// `DuplicateMinted` (user error 15).
    DuplicateMinted = 15,
    /// `FailedToConvertToCLValue` (user error 16).
    FailedToConvertToCLValue = 16,
    /// `MissingCollectionName` (user error 17).
    MissingCollectionName = 17,
    /// `InvalidCollectionName` (user error 18).
    InvalidCollectionName = 18,
    /// `FailedToSerializeMetaData` (user error 19).
    FailedToSerializeMetaData = 19,
    /// `MissingAccount` (user error 20).
    MissingAccount = 20,
    /// `MissingMintingStatus` (user error 21).
    MissingMintingStatus = 21,
    /// `InvalidMintingStatus` (user error 22).
    InvalidMintingStatus = 22,
    /// `MissingCollectionSymbol` (user error 23).
    MissingCollectionSymbol = 23,
    /// `InvalidCollectionSymbol` (user error 24).
    InvalidCollectionSymbol = 24,
    /// `MissingTotalTokenSupply` (user error 25).
    MissingTotalTokenSupply = 25,
    /// `InvalidTotalTokenSupply` (user error 26).
    InvalidTotalTokenSupply = 26,
    /// `MissingTokenID` (user error 27).
    MissingTokenID = 27,
    /// `InvalidTokenIdentifier` (user error 28).
    InvalidTokenIdentifier = 28,
    /// `MissingTokenOwners` (user error 29).
    MissingTokenOwners = 29,
    /// `MissingAccountHash` (user error 30).
    MissingAccountHash = 30,
    /// `InvalidAccountHash` (user error 31).
    InvalidAccountHash = 31,
    /// `TokenSupplyDepleted` (user error 32).
    TokenSupplyDepleted = 32,
    /// `MissingOwnedTokensDictionary` (user error 33).
    MissingOwnedTokensDictionary = 33,
    /// `TokenAlreadyBelongsToMinterFatal` (user error 34).
    TokenAlreadyBelongsToMinterFatal = 34,
    /// `FatalTokenIdDuplication` (user error 35).
    FatalTokenIdDuplication = 35,
    /// `InvalidMinter` (user error 36).
    InvalidMinter = 36,
    /// `MissingMintingMode` (user error 37).
    MissingMintingMode = 37,
    /// `InvalidMintingMode` (user error 38).
    InvalidMintingMode = 38,
    /// `MissingInstallerKey` (user error 39).
    MissingInstallerKey = 39,
    /// `FailedToConvertToAccountHash` (user error 40).
    FailedToConvertToAccountHash = 40,
    /// `InvalidBurner` (user error 41).
    InvalidBurner = 41,
    /// `PreviouslyBurntToken` (user error 42).
    PreviouslyBurntToken = 42,
    /// `MissingAllowMinting` (user error 43).
    MissingAllowMinting = 43,
    /// `InvalidAllowMinting` (user error 44).
    InvalidAllowMinting = 44,
    /// `MissingNumberOfMintedTokens` (user error 45).
    MissingNumberOfMintedTokens = 45,
    /// `InvalidNumberOfMintedTokens` (user error 46).
    InvalidNumberOfMintedTokens = 46,
    /// `MissingTokenMetaData` (user error 47).
    MissingTokenMetaData = 47,
    /// `InvalidTokenMetaData` (user error 48).
    InvalidTokenMetaData = 48,
    /// `MissingApprovedAccountHash` (user error 49).
    MissingApprovedAccountHash = 49,
    /// `InvalidApprovedAccountHash` (user error 50).
    InvalidApprovedAccountHash = 50,
    /// `MissingApprovedTokensDictionary` (user error 51).
    MissingApprovedTokensDictionary = 51,
    /// `TokenAlreadyApproved` (user error 52).
    TokenAlreadyApproved = 52,
    /// `MissingApproveAll` (user error 53).
    MissingApproveAll = 53,
    /// `InvalidApproveAll` (user error 54).
    InvalidApproveAll = 54,
    /// `MissingOperator` (user error 55).
    MissingOperator = 55,
    /// `InvalidOperator` (user error 56).
    InvalidOperator = 56,
    /// `Phantom` (user error 57).
    Phantom = 57,
    /// `ContractAlreadyInitialized` (user error 58).
    ContractAlreadyInitialized = 58,
    /// `MintingIsPaused` (user error 59).
    MintingIsPaused = 59,
    /// `FailureToParseAccountHash` (user error 60).
    FailureToParseAccountHash = 60,
    /// `VacantValueInDictionary` (user error 61).
    VacantValueInDictionary = 61,
    /// `MissingOwnershipMode` (user error 62).
    MissingOwnershipMode = 62,
    /// `InvalidOwnershipMode` (user error 63).
    InvalidOwnershipMode = 63,
    /// `InvalidTokenMinter` (user error 64).
    InvalidTokenMinter = 64,
    /// `MissingOwnedTokens` (user error 65).
    MissingOwnedTokens = 65,
    /// `InvalidAccountKeyInDictionary` (user error 66).
    InvalidAccountKeyInDictionary = 66,
    /// `MissingJsonSchema` (user error 67).
    MissingJsonSchema = 67,
    /// `InvalidJsonSchema` (user error 68).
    InvalidJsonSchema = 68,
    /// `InvalidKey` (user error 69).
    InvalidKey = 69,
    /// `InvalidOwnedTokens` (user error 70).
    InvalidOwnedTokens = 70,
    /// `MissingTokenURI` (user error 71).
    MissingTokenURI = 71,
    /// `InvalidTokenURI` (user error 72).
    InvalidTokenURI = 72,
    /// `MissingNftKind` (user error 73).
    MissingNftKind = 73,
    /// `InvalidNftKind` (user error 74).
    InvalidNftKind = 74,
    /// `MissingHolderMode` (user error 75).
    MissingHolderMode = 75,
    /// `InvalidHolderMode` (user error 76).
    InvalidHolderMode = 76,
    /// `MissingWhitelistMode` (user error 77).
    MissingWhitelistMode = 77,
    /// `InvalidWhitelistMode` (user error 78).
    InvalidWhitelistMode = 78,
    /// `MissingContractWhiteList` (user error 79).
    MissingContractWhiteList = 79,
    /// `InvalidContractWhitelist` (user error 80).
    InvalidContractWhitelist = 80,
    /// `UnlistedContractHash` (user error 81).
    UnlistedContractHash = 81,
    /// `InvalidContract` (user error 82).
    InvalidContract = 82,
    /// `EmptyContractWhitelist` (user error 83).
    EmptyContractWhitelist = 83,
    /// `MissingReceiptName` (user error 84).
    MissingReceiptName = 84,
    /// `InvalidReceiptName` (user error 85).
    InvalidReceiptName = 85,
    /// `InvalidJsonMetadata` (user error 86).
    InvalidJsonMetadata = 86,
    /// `InvalidJsonFormat` (user error 87).
    InvalidJsonFormat = 87,
    /// `FailedToParseCep99Metadata` (user error 88).
    FailedToParseCep99Metadata = 88,
    /// `FailedToParse721Metadata` (user error 89).
    FailedToParse721Metadata = 89,
    /// `FailedToParseCustomMetadata` (user error 90).
    FailedToParseCustomMetadata = 90,
    /// `InvalidCEP99Metadata` (user error 91).
    InvalidCEP99Metadata = 91,
    /// `FailedToJsonifyCEP99Metadata` (user error 92).
    FailedToJsonifyCEP99Metadata = 92,
    /// `InvalidNFT721Metadata` (user error 93).
    InvalidNFT721Metadata = 93,
    /// `FailedToJsonifyNFT721Metadata` (user error 94).
    FailedToJsonifyNFT721Metadata = 94,
    /// `InvalidCustomMetadata` (user error 95).
    InvalidCustomMetadata = 95,
    /// `MissingNFTMetadataKind` (user error 96).
    MissingNFTMetadataKind = 96,
    /// `InvalidNFTMetadataKind` (user error 97).
    InvalidNFTMetadataKind = 97,
    /// `MissingIdentifierMode` (user error 98).
    MissingIdentifierMode = 98,
    /// `InvalidIdentifierMode` (user error 99).
    InvalidIdentifierMode = 99,
    /// `FailedToParseTokenId` (user error 100).
    FailedToParseTokenId = 100,
    /// `MissingMetadataMutability` (user error 101).
    MissingMetadataMutability = 101,
    /// `InvalidMetadataMutability` (user error 102).
    InvalidMetadataMutability = 102,
    /// `FailedToJsonifyCustomMetadata` (user error 103).
    FailedToJsonifyCustomMetadata = 103,
    /// `ForbiddenMetadataUpdate` (user error 104).
    ForbiddenMetadataUpdate = 104,
    /// `MissingBurnMode` (user error 105).
    MissingBurnMode = 105,
    /// `InvalidBurnMode` (user error 106).
    InvalidBurnMode = 106,
    /// `MissingHashByIndex` (user error 107).
    MissingHashByIndex = 107,
    /// `InvalidHashByIndex` (user error 108).
    InvalidHashByIndex = 108,
    /// `MissingIndexByHash` (user error 109).
    MissingIndexByHash = 109,
    /// `InvalidIndexByHash` (user error 110).
    InvalidIndexByHash = 110,
    /// `MissingPageTableURef` (user error 111).
    MissingPageTableURef = 111,
    /// `InvalidPageTableURef` (user error 112).
    InvalidPageTableURef = 112,
    /// `MissingPageLimit` (user error 113).
    MissingPageLimit = 113,
    /// `InvalidPageLimit` (user error 114).
    InvalidPageLimit = 114,
    /// `InvalidPageNumber` (user error 115).
    InvalidPageNumber = 115,
    /// `InvalidPageIndex` (user error 116).
    InvalidPageIndex = 116,
    /// `MissingUnmatchedHashCount` (user error 117).
    MissingUnmatchedHashCount = 117,
    /// `InvalidUnmatchedHashCount` (user error 118).
    InvalidUnmatchedHashCount = 118,
    /// `MissingPackageHashForUpgrade` (user error 119).
    MissingPackageHashForUpgrade = 119,
    /// `MissingPageUref` (user error 120).
    MissingPageUref = 120,
    /// `InvalidPageUref` (user error 121).
    InvalidPageUref = 121,
    /// `CannotUpgradeWithZeroSupply` (user error 122).
    CannotUpgradeWithZeroSupply = 122,
    /// `CannotInstallWithZeroSupply` (user error 123).
    CannotInstallWithZeroSupply = 123,
    /// `MissingMigrationFlag` (user error 124).
    MissingMigrationFlag = 124,
    /// `InvalidMigrationFlag` (user error 125).
    InvalidMigrationFlag = 125,
    /// `ContractAlreadyMigrated` (user error 126).
    ContractAlreadyMigrated = 126,
    /// `UnregisteredOwnerInMint` (user error 127).
    UnregisteredOwnerInMint = 127,
    /// `UnregisteredOwnerInTransfer` (user error 128).
    UnregisteredOwnerInTransfer = 128,
    /// `MissingReportingMode` (user error 129).
    MissingReportingMode = 129,
    /// `InvalidReportingMode` (user error 130).
    InvalidReportingMode = 130,
    /// `MissingPage` (user error 131).
    MissingPage = 131,
    /// `UnregisteredOwnerFromMigration` (user error 132).
    UnregisteredOwnerFromMigration = 132,
    /// `ExceededMaxTotalSupply` (user error 133).
    ExceededMaxTotalSupply = 133,
    /// `MissingCEP78PackageHash` (user error 134).
    MissingCEP78PackageHash = 134,
    /// `InvalidCEP78InvalidHash` (user error 135).
    InvalidCEP78InvalidHash = 135,
    /// `InvalidPackageHashName` (user error 136).
    InvalidPackageHashName = 136,
    /// `InvalidAccessKeyName` (user error 137).
    InvalidAccessKeyName = 137,
    /// `InvalidCheckForUpgrade` (user error 138).
    InvalidCheckForUpgrade = 138,
    /// `InvalidNamedKeyConvention` (user error 139).
    InvalidNamedKeyConvention = 139,
    /// `OwnerReverseLookupModeNotTransferable` (user error 140).
    OwnerReverseLookupModeNotTransferable = 140,
    /// `InvalidAdditionalRequiredMetadata` (user error 141).
    InvalidAdditionalRequiredMetadata = 141,
    /// `InvalidOptionalMetadata` (user error 142).
    InvalidOptionalMetadata = 142,
    /// `MissingOptionalNFTMetadataKind` (user error 143).
    MissingOptionalNFTMetadataKind = 143,
    /// `InvalidOptionalNFTMetadataKind` (user error 144).
    InvalidOptionalNFTMetadataKind = 144,
    /// `MissingAdditionalNFTMetadataKind` (user error 145).
    MissingAdditionalNFTMetadataKind = 145,
    /// `InvalidAdditionalNFTMetadataKind` (user error 146).
    InvalidAdditionalNFTMetadataKind = 146,
    /// `InvalidRequirement` (user error 147).
    InvalidRequirement = 147,
    /// `MissingEventsMode` (user error 148).
    MissingEventsMode = 148,
    /// `InvalidEventsMode` (user error 149).
    InvalidEventsMode = 149,
    /// `CannotUpgradeToMoreSupply` (user error 150).
    CannotUpgradeToMoreSupply = 150,
    /// `MissingOperatorDict` (user error 151).
    MissingOperatorDict = 151,
    /// `MissingApprovedDict` (user error 152).
    MissingApprovedDict = 152,
    /// `MissingSpenderAccountHash` (user error 153).
    MissingSpenderAccountHash = 153,
    /// `InvalidSpenderAccountHash` (user error 154).
    InvalidSpenderAccountHash = 154,
    /// `MissingOwnerTokenIdentifierKey` (user error 155).
    MissingOwnerTokenIdentifierKey = 155,
    /// `InvalidTransferFilterContract` (user error 156).
    InvalidTransferFilterContract = 156,
    /// `MissingTransferFilterContract` (user error 157).
    MissingTransferFilterContract = 157,
    /// `TransferFilterContractNeedsTransferableMode` (user error 158).
    TransferFilterContractNeedsTransferableMode = 158,
    /// `TransferFilterContractDenied` (user error 159).
    TransferFilterContractDenied = 159,
    /// `MissingACLWhiteList` (user error 160).
    MissingACLWhiteList = 160,
    /// `InvalidACLWhitelist` (user error 161).
    InvalidACLWhitelist = 161,
    /// `EmptyACLWhitelist` (user error 162).
    EmptyACLWhitelist = 162,
    /// `InvalidACLPackageMode` (user error 163).
    InvalidACLPackageMode = 163,
    /// `MissingACLPackageMode` (user error 164).
    MissingACLPackageMode = 164,
    /// `InvalidPackageOperatorMode` (user error 165).
    InvalidPackageOperatorMode = 165,
    /// `MissingPackageOperatorMode` (user error 166).
    MissingPackageOperatorMode = 166,
    /// `InvalidOperatorBurnMode` (user error 167).
    InvalidOperatorBurnMode = 167,
    /// `MissingOperatorBurnMode` (user error 168).
    MissingOperatorBurnMode = 168,
    /// `InvalidIdentifier` (user error 169).
    InvalidIdentifier = 169,
    /// `DuplicateIdentifier` (user error 170).
    DuplicateIdentifier = 170,
    /// `CannotInsertArg` (user error 171).
    CannotInsertArg = 171,
    /// `InvalidPackageHash` (user error 172).
    InvalidPackageHash = 172,
    /// `InvalidUrefMigrationKey` (user error 173).
    InvalidUrefMigrationKey = 173,
    /// `InvalidRloKey` (user error 174).
    InvalidRloKey = 174,
    /// `UnlistedEntity` (user error 175).
    UnlistedEntity = 175,
    /// `FailedToConvertToEntityHash` (user error 176).
    FailedToConvertToEntityHash = 176,
    /// `InvalidMinterEntity` (user error 177).
    InvalidMinterEntity = 177,
    /// `FailedToConvertEventToJson` (user error 178).
    FailedToConvertEventToJson = 178,
    /// `MissingVersionContractKey` (user error 179).
    MissingVersionContractKey = 179,
    /// `InvalidVersionContractKey` (user error 180).
    InvalidVersionContractKey = 180,
}

impl CEP78Error {
    /// Map from on-chain user error code when known.
    pub fn from_user_code(code: u16) -> Option<Self> {
        match code {
            1 => Some(Self::InvalidAccount),
            2 => Some(Self::MissingInstaller),
            3 => Some(Self::InvalidInstaller),
            4 => Some(Self::UnexpectedKeyVariant),
            5 => Some(Self::MissingTokenOwner),
            6 => Some(Self::InvalidTokenOwner),
            7 => Some(Self::FailedToGetArgBytes),
            8 => Some(Self::FailedToCreateDictionary),
            9 => Some(Self::MissingStorageUref),
            10 => Some(Self::InvalidStorageUref),
            11 => Some(Self::MissingOwnersUref),
            12 => Some(Self::InvalidOwnersUref),
            13 => Some(Self::FailedToAccessStorageDictionary),
            14 => Some(Self::FailedToAccessOwnershipDictionary),
            15 => Some(Self::DuplicateMinted),
            16 => Some(Self::FailedToConvertToCLValue),
            17 => Some(Self::MissingCollectionName),
            18 => Some(Self::InvalidCollectionName),
            19 => Some(Self::FailedToSerializeMetaData),
            20 => Some(Self::MissingAccount),
            21 => Some(Self::MissingMintingStatus),
            22 => Some(Self::InvalidMintingStatus),
            23 => Some(Self::MissingCollectionSymbol),
            24 => Some(Self::InvalidCollectionSymbol),
            25 => Some(Self::MissingTotalTokenSupply),
            26 => Some(Self::InvalidTotalTokenSupply),
            27 => Some(Self::MissingTokenID),
            28 => Some(Self::InvalidTokenIdentifier),
            29 => Some(Self::MissingTokenOwners),
            30 => Some(Self::MissingAccountHash),
            31 => Some(Self::InvalidAccountHash),
            32 => Some(Self::TokenSupplyDepleted),
            33 => Some(Self::MissingOwnedTokensDictionary),
            34 => Some(Self::TokenAlreadyBelongsToMinterFatal),
            35 => Some(Self::FatalTokenIdDuplication),
            36 => Some(Self::InvalidMinter),
            37 => Some(Self::MissingMintingMode),
            38 => Some(Self::InvalidMintingMode),
            39 => Some(Self::MissingInstallerKey),
            40 => Some(Self::FailedToConvertToAccountHash),
            41 => Some(Self::InvalidBurner),
            42 => Some(Self::PreviouslyBurntToken),
            43 => Some(Self::MissingAllowMinting),
            44 => Some(Self::InvalidAllowMinting),
            45 => Some(Self::MissingNumberOfMintedTokens),
            46 => Some(Self::InvalidNumberOfMintedTokens),
            47 => Some(Self::MissingTokenMetaData),
            48 => Some(Self::InvalidTokenMetaData),
            49 => Some(Self::MissingApprovedAccountHash),
            50 => Some(Self::InvalidApprovedAccountHash),
            51 => Some(Self::MissingApprovedTokensDictionary),
            52 => Some(Self::TokenAlreadyApproved),
            53 => Some(Self::MissingApproveAll),
            54 => Some(Self::InvalidApproveAll),
            55 => Some(Self::MissingOperator),
            56 => Some(Self::InvalidOperator),
            57 => Some(Self::Phantom),
            58 => Some(Self::ContractAlreadyInitialized),
            59 => Some(Self::MintingIsPaused),
            60 => Some(Self::FailureToParseAccountHash),
            61 => Some(Self::VacantValueInDictionary),
            62 => Some(Self::MissingOwnershipMode),
            63 => Some(Self::InvalidOwnershipMode),
            64 => Some(Self::InvalidTokenMinter),
            65 => Some(Self::MissingOwnedTokens),
            66 => Some(Self::InvalidAccountKeyInDictionary),
            67 => Some(Self::MissingJsonSchema),
            68 => Some(Self::InvalidJsonSchema),
            69 => Some(Self::InvalidKey),
            70 => Some(Self::InvalidOwnedTokens),
            71 => Some(Self::MissingTokenURI),
            72 => Some(Self::InvalidTokenURI),
            73 => Some(Self::MissingNftKind),
            74 => Some(Self::InvalidNftKind),
            75 => Some(Self::MissingHolderMode),
            76 => Some(Self::InvalidHolderMode),
            77 => Some(Self::MissingWhitelistMode),
            78 => Some(Self::InvalidWhitelistMode),
            79 => Some(Self::MissingContractWhiteList),
            80 => Some(Self::InvalidContractWhitelist),
            81 => Some(Self::UnlistedContractHash),
            82 => Some(Self::InvalidContract),
            83 => Some(Self::EmptyContractWhitelist),
            84 => Some(Self::MissingReceiptName),
            85 => Some(Self::InvalidReceiptName),
            86 => Some(Self::InvalidJsonMetadata),
            87 => Some(Self::InvalidJsonFormat),
            88 => Some(Self::FailedToParseCep99Metadata),
            89 => Some(Self::FailedToParse721Metadata),
            90 => Some(Self::FailedToParseCustomMetadata),
            91 => Some(Self::InvalidCEP99Metadata),
            92 => Some(Self::FailedToJsonifyCEP99Metadata),
            93 => Some(Self::InvalidNFT721Metadata),
            94 => Some(Self::FailedToJsonifyNFT721Metadata),
            95 => Some(Self::InvalidCustomMetadata),
            96 => Some(Self::MissingNFTMetadataKind),
            97 => Some(Self::InvalidNFTMetadataKind),
            98 => Some(Self::MissingIdentifierMode),
            99 => Some(Self::InvalidIdentifierMode),
            100 => Some(Self::FailedToParseTokenId),
            101 => Some(Self::MissingMetadataMutability),
            102 => Some(Self::InvalidMetadataMutability),
            103 => Some(Self::FailedToJsonifyCustomMetadata),
            104 => Some(Self::ForbiddenMetadataUpdate),
            105 => Some(Self::MissingBurnMode),
            106 => Some(Self::InvalidBurnMode),
            107 => Some(Self::MissingHashByIndex),
            108 => Some(Self::InvalidHashByIndex),
            109 => Some(Self::MissingIndexByHash),
            110 => Some(Self::InvalidIndexByHash),
            111 => Some(Self::MissingPageTableURef),
            112 => Some(Self::InvalidPageTableURef),
            113 => Some(Self::MissingPageLimit),
            114 => Some(Self::InvalidPageLimit),
            115 => Some(Self::InvalidPageNumber),
            116 => Some(Self::InvalidPageIndex),
            117 => Some(Self::MissingUnmatchedHashCount),
            118 => Some(Self::InvalidUnmatchedHashCount),
            119 => Some(Self::MissingPackageHashForUpgrade),
            120 => Some(Self::MissingPageUref),
            121 => Some(Self::InvalidPageUref),
            122 => Some(Self::CannotUpgradeWithZeroSupply),
            123 => Some(Self::CannotInstallWithZeroSupply),
            124 => Some(Self::MissingMigrationFlag),
            125 => Some(Self::InvalidMigrationFlag),
            126 => Some(Self::ContractAlreadyMigrated),
            127 => Some(Self::UnregisteredOwnerInMint),
            128 => Some(Self::UnregisteredOwnerInTransfer),
            129 => Some(Self::MissingReportingMode),
            130 => Some(Self::InvalidReportingMode),
            131 => Some(Self::MissingPage),
            132 => Some(Self::UnregisteredOwnerFromMigration),
            133 => Some(Self::ExceededMaxTotalSupply),
            134 => Some(Self::MissingCEP78PackageHash),
            135 => Some(Self::InvalidCEP78InvalidHash),
            136 => Some(Self::InvalidPackageHashName),
            137 => Some(Self::InvalidAccessKeyName),
            138 => Some(Self::InvalidCheckForUpgrade),
            139 => Some(Self::InvalidNamedKeyConvention),
            140 => Some(Self::OwnerReverseLookupModeNotTransferable),
            141 => Some(Self::InvalidAdditionalRequiredMetadata),
            142 => Some(Self::InvalidOptionalMetadata),
            143 => Some(Self::MissingOptionalNFTMetadataKind),
            144 => Some(Self::InvalidOptionalNFTMetadataKind),
            145 => Some(Self::MissingAdditionalNFTMetadataKind),
            146 => Some(Self::InvalidAdditionalNFTMetadataKind),
            147 => Some(Self::InvalidRequirement),
            148 => Some(Self::MissingEventsMode),
            149 => Some(Self::InvalidEventsMode),
            150 => Some(Self::CannotUpgradeToMoreSupply),
            151 => Some(Self::MissingOperatorDict),
            152 => Some(Self::MissingApprovedDict),
            153 => Some(Self::MissingSpenderAccountHash),
            154 => Some(Self::InvalidSpenderAccountHash),
            155 => Some(Self::MissingOwnerTokenIdentifierKey),
            156 => Some(Self::InvalidTransferFilterContract),
            157 => Some(Self::MissingTransferFilterContract),
            158 => Some(Self::TransferFilterContractNeedsTransferableMode),
            159 => Some(Self::TransferFilterContractDenied),
            160 => Some(Self::MissingACLWhiteList),
            161 => Some(Self::InvalidACLWhitelist),
            162 => Some(Self::EmptyACLWhitelist),
            163 => Some(Self::InvalidACLPackageMode),
            164 => Some(Self::MissingACLPackageMode),
            165 => Some(Self::InvalidPackageOperatorMode),
            166 => Some(Self::MissingPackageOperatorMode),
            167 => Some(Self::InvalidOperatorBurnMode),
            168 => Some(Self::MissingOperatorBurnMode),
            169 => Some(Self::InvalidIdentifier),
            170 => Some(Self::DuplicateIdentifier),
            171 => Some(Self::CannotInsertArg),
            172 => Some(Self::InvalidPackageHash),
            173 => Some(Self::InvalidUrefMigrationKey),
            174 => Some(Self::InvalidRloKey),
            175 => Some(Self::UnlistedEntity),
            176 => Some(Self::FailedToConvertToEntityHash),
            177 => Some(Self::InvalidMinterEntity),
            178 => Some(Self::FailedToConvertEventToJson),
            179 => Some(Self::MissingVersionContractKey),
            180 => Some(Self::InvalidVersionContractKey),
            _ => None,
        }
    }

    /// Stable name for logs.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidAccount => "InvalidAccount",
            Self::MissingInstaller => "MissingInstaller",
            Self::InvalidInstaller => "InvalidInstaller",
            Self::UnexpectedKeyVariant => "UnexpectedKeyVariant",
            Self::MissingTokenOwner => "MissingTokenOwner",
            Self::InvalidTokenOwner => "InvalidTokenOwner",
            Self::FailedToGetArgBytes => "FailedToGetArgBytes",
            Self::FailedToCreateDictionary => "FailedToCreateDictionary",
            Self::MissingStorageUref => "MissingStorageUref",
            Self::InvalidStorageUref => "InvalidStorageUref",
            Self::MissingOwnersUref => "MissingOwnersUref",
            Self::InvalidOwnersUref => "InvalidOwnersUref",
            Self::FailedToAccessStorageDictionary => "FailedToAccessStorageDictionary",
            Self::FailedToAccessOwnershipDictionary => "FailedToAccessOwnershipDictionary",
            Self::DuplicateMinted => "DuplicateMinted",
            Self::FailedToConvertToCLValue => "FailedToConvertToCLValue",
            Self::MissingCollectionName => "MissingCollectionName",
            Self::InvalidCollectionName => "InvalidCollectionName",
            Self::FailedToSerializeMetaData => "FailedToSerializeMetaData",
            Self::MissingAccount => "MissingAccount",
            Self::MissingMintingStatus => "MissingMintingStatus",
            Self::InvalidMintingStatus => "InvalidMintingStatus",
            Self::MissingCollectionSymbol => "MissingCollectionSymbol",
            Self::InvalidCollectionSymbol => "InvalidCollectionSymbol",
            Self::MissingTotalTokenSupply => "MissingTotalTokenSupply",
            Self::InvalidTotalTokenSupply => "InvalidTotalTokenSupply",
            Self::MissingTokenID => "MissingTokenID",
            Self::InvalidTokenIdentifier => "InvalidTokenIdentifier",
            Self::MissingTokenOwners => "MissingTokenOwners",
            Self::MissingAccountHash => "MissingAccountHash",
            Self::InvalidAccountHash => "InvalidAccountHash",
            Self::TokenSupplyDepleted => "TokenSupplyDepleted",
            Self::MissingOwnedTokensDictionary => "MissingOwnedTokensDictionary",
            Self::TokenAlreadyBelongsToMinterFatal => "TokenAlreadyBelongsToMinterFatal",
            Self::FatalTokenIdDuplication => "FatalTokenIdDuplication",
            Self::InvalidMinter => "InvalidMinter",
            Self::MissingMintingMode => "MissingMintingMode",
            Self::InvalidMintingMode => "InvalidMintingMode",
            Self::MissingInstallerKey => "MissingInstallerKey",
            Self::FailedToConvertToAccountHash => "FailedToConvertToAccountHash",
            Self::InvalidBurner => "InvalidBurner",
            Self::PreviouslyBurntToken => "PreviouslyBurntToken",
            Self::MissingAllowMinting => "MissingAllowMinting",
            Self::InvalidAllowMinting => "InvalidAllowMinting",
            Self::MissingNumberOfMintedTokens => "MissingNumberOfMintedTokens",
            Self::InvalidNumberOfMintedTokens => "InvalidNumberOfMintedTokens",
            Self::MissingTokenMetaData => "MissingTokenMetaData",
            Self::InvalidTokenMetaData => "InvalidTokenMetaData",
            Self::MissingApprovedAccountHash => "MissingApprovedAccountHash",
            Self::InvalidApprovedAccountHash => "InvalidApprovedAccountHash",
            Self::MissingApprovedTokensDictionary => "MissingApprovedTokensDictionary",
            Self::TokenAlreadyApproved => "TokenAlreadyApproved",
            Self::MissingApproveAll => "MissingApproveAll",
            Self::InvalidApproveAll => "InvalidApproveAll",
            Self::MissingOperator => "MissingOperator",
            Self::InvalidOperator => "InvalidOperator",
            Self::Phantom => "Phantom",
            Self::ContractAlreadyInitialized => "ContractAlreadyInitialized",
            Self::MintingIsPaused => "MintingIsPaused",
            Self::FailureToParseAccountHash => "FailureToParseAccountHash",
            Self::VacantValueInDictionary => "VacantValueInDictionary",
            Self::MissingOwnershipMode => "MissingOwnershipMode",
            Self::InvalidOwnershipMode => "InvalidOwnershipMode",
            Self::InvalidTokenMinter => "InvalidTokenMinter",
            Self::MissingOwnedTokens => "MissingOwnedTokens",
            Self::InvalidAccountKeyInDictionary => "InvalidAccountKeyInDictionary",
            Self::MissingJsonSchema => "MissingJsonSchema",
            Self::InvalidJsonSchema => "InvalidJsonSchema",
            Self::InvalidKey => "InvalidKey",
            Self::InvalidOwnedTokens => "InvalidOwnedTokens",
            Self::MissingTokenURI => "MissingTokenURI",
            Self::InvalidTokenURI => "InvalidTokenURI",
            Self::MissingNftKind => "MissingNftKind",
            Self::InvalidNftKind => "InvalidNftKind",
            Self::MissingHolderMode => "MissingHolderMode",
            Self::InvalidHolderMode => "InvalidHolderMode",
            Self::MissingWhitelistMode => "MissingWhitelistMode",
            Self::InvalidWhitelistMode => "InvalidWhitelistMode",
            Self::MissingContractWhiteList => "MissingContractWhiteList",
            Self::InvalidContractWhitelist => "InvalidContractWhitelist",
            Self::UnlistedContractHash => "UnlistedContractHash",
            Self::InvalidContract => "InvalidContract",
            Self::EmptyContractWhitelist => "EmptyContractWhitelist",
            Self::MissingReceiptName => "MissingReceiptName",
            Self::InvalidReceiptName => "InvalidReceiptName",
            Self::InvalidJsonMetadata => "InvalidJsonMetadata",
            Self::InvalidJsonFormat => "InvalidJsonFormat",
            Self::FailedToParseCep99Metadata => "FailedToParseCep99Metadata",
            Self::FailedToParse721Metadata => "FailedToParse721Metadata",
            Self::FailedToParseCustomMetadata => "FailedToParseCustomMetadata",
            Self::InvalidCEP99Metadata => "InvalidCEP99Metadata",
            Self::FailedToJsonifyCEP99Metadata => "FailedToJsonifyCEP99Metadata",
            Self::InvalidNFT721Metadata => "InvalidNFT721Metadata",
            Self::FailedToJsonifyNFT721Metadata => "FailedToJsonifyNFT721Metadata",
            Self::InvalidCustomMetadata => "InvalidCustomMetadata",
            Self::MissingNFTMetadataKind => "MissingNFTMetadataKind",
            Self::InvalidNFTMetadataKind => "InvalidNFTMetadataKind",
            Self::MissingIdentifierMode => "MissingIdentifierMode",
            Self::InvalidIdentifierMode => "InvalidIdentifierMode",
            Self::FailedToParseTokenId => "FailedToParseTokenId",
            Self::MissingMetadataMutability => "MissingMetadataMutability",
            Self::InvalidMetadataMutability => "InvalidMetadataMutability",
            Self::FailedToJsonifyCustomMetadata => "FailedToJsonifyCustomMetadata",
            Self::ForbiddenMetadataUpdate => "ForbiddenMetadataUpdate",
            Self::MissingBurnMode => "MissingBurnMode",
            Self::InvalidBurnMode => "InvalidBurnMode",
            Self::MissingHashByIndex => "MissingHashByIndex",
            Self::InvalidHashByIndex => "InvalidHashByIndex",
            Self::MissingIndexByHash => "MissingIndexByHash",
            Self::InvalidIndexByHash => "InvalidIndexByHash",
            Self::MissingPageTableURef => "MissingPageTableURef",
            Self::InvalidPageTableURef => "InvalidPageTableURef",
            Self::MissingPageLimit => "MissingPageLimit",
            Self::InvalidPageLimit => "InvalidPageLimit",
            Self::InvalidPageNumber => "InvalidPageNumber",
            Self::InvalidPageIndex => "InvalidPageIndex",
            Self::MissingUnmatchedHashCount => "MissingUnmatchedHashCount",
            Self::InvalidUnmatchedHashCount => "InvalidUnmatchedHashCount",
            Self::MissingPackageHashForUpgrade => "MissingPackageHashForUpgrade",
            Self::MissingPageUref => "MissingPageUref",
            Self::InvalidPageUref => "InvalidPageUref",
            Self::CannotUpgradeWithZeroSupply => "CannotUpgradeWithZeroSupply",
            Self::CannotInstallWithZeroSupply => "CannotInstallWithZeroSupply",
            Self::MissingMigrationFlag => "MissingMigrationFlag",
            Self::InvalidMigrationFlag => "InvalidMigrationFlag",
            Self::ContractAlreadyMigrated => "ContractAlreadyMigrated",
            Self::UnregisteredOwnerInMint => "UnregisteredOwnerInMint",
            Self::UnregisteredOwnerInTransfer => "UnregisteredOwnerInTransfer",
            Self::MissingReportingMode => "MissingReportingMode",
            Self::InvalidReportingMode => "InvalidReportingMode",
            Self::MissingPage => "MissingPage",
            Self::UnregisteredOwnerFromMigration => "UnregisteredOwnerFromMigration",
            Self::ExceededMaxTotalSupply => "ExceededMaxTotalSupply",
            Self::MissingCEP78PackageHash => "MissingCEP78PackageHash",
            Self::InvalidCEP78InvalidHash => "InvalidCEP78InvalidHash",
            Self::InvalidPackageHashName => "InvalidPackageHashName",
            Self::InvalidAccessKeyName => "InvalidAccessKeyName",
            Self::InvalidCheckForUpgrade => "InvalidCheckForUpgrade",
            Self::InvalidNamedKeyConvention => "InvalidNamedKeyConvention",
            Self::OwnerReverseLookupModeNotTransferable => "OwnerReverseLookupModeNotTransferable",
            Self::InvalidAdditionalRequiredMetadata => "InvalidAdditionalRequiredMetadata",
            Self::InvalidOptionalMetadata => "InvalidOptionalMetadata",
            Self::MissingOptionalNFTMetadataKind => "MissingOptionalNFTMetadataKind",
            Self::InvalidOptionalNFTMetadataKind => "InvalidOptionalNFTMetadataKind",
            Self::MissingAdditionalNFTMetadataKind => "MissingAdditionalNFTMetadataKind",
            Self::InvalidAdditionalNFTMetadataKind => "InvalidAdditionalNFTMetadataKind",
            Self::InvalidRequirement => "InvalidRequirement",
            Self::MissingEventsMode => "MissingEventsMode",
            Self::InvalidEventsMode => "InvalidEventsMode",
            Self::CannotUpgradeToMoreSupply => "CannotUpgradeToMoreSupply",
            Self::MissingOperatorDict => "MissingOperatorDict",
            Self::MissingApprovedDict => "MissingApprovedDict",
            Self::MissingSpenderAccountHash => "MissingSpenderAccountHash",
            Self::InvalidSpenderAccountHash => "InvalidSpenderAccountHash",
            Self::MissingOwnerTokenIdentifierKey => "MissingOwnerTokenIdentifierKey",
            Self::InvalidTransferFilterContract => "InvalidTransferFilterContract",
            Self::MissingTransferFilterContract => "MissingTransferFilterContract",
            Self::TransferFilterContractNeedsTransferableMode => {
                "TransferFilterContractNeedsTransferableMode"
            }
            Self::TransferFilterContractDenied => "TransferFilterContractDenied",
            Self::MissingACLWhiteList => "MissingACLWhiteList",
            Self::InvalidACLWhitelist => "InvalidACLWhitelist",
            Self::EmptyACLWhitelist => "EmptyACLWhitelist",
            Self::InvalidACLPackageMode => "InvalidACLPackageMode",
            Self::MissingACLPackageMode => "MissingACLPackageMode",
            Self::InvalidPackageOperatorMode => "InvalidPackageOperatorMode",
            Self::MissingPackageOperatorMode => "MissingPackageOperatorMode",
            Self::InvalidOperatorBurnMode => "InvalidOperatorBurnMode",
            Self::MissingOperatorBurnMode => "MissingOperatorBurnMode",
            Self::InvalidIdentifier => "InvalidIdentifier",
            Self::DuplicateIdentifier => "DuplicateIdentifier",
            Self::CannotInsertArg => "CannotInsertArg",
            Self::InvalidPackageHash => "InvalidPackageHash",
            Self::InvalidUrefMigrationKey => "InvalidUrefMigrationKey",
            Self::InvalidRloKey => "InvalidRloKey",
            Self::UnlistedEntity => "UnlistedEntity",
            Self::FailedToConvertToEntityHash => "FailedToConvertToEntityHash",
            Self::InvalidMinterEntity => "InvalidMinterEntity",
            Self::FailedToConvertEventToJson => "FailedToConvertEventToJson",
            Self::MissingVersionContractKey => "MissingVersionContractKey",
            Self::InvalidVersionContractKey => "InvalidVersionContractKey",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_invalid_token_identifier() {
        assert_eq!(
            CEP78Error::from_user_code(28),
            Some(CEP78Error::InvalidTokenIdentifier)
        );
        assert_eq!(
            CEP78Error::InvalidTokenIdentifier.as_str(),
            "InvalidTokenIdentifier"
        );
    }

    #[test]
    fn maps_full_range_ends() {
        assert_eq!(
            CEP78Error::from_user_code(1),
            Some(CEP78Error::InvalidAccount)
        );
        assert_eq!(
            CEP78Error::from_user_code(180),
            Some(CEP78Error::InvalidVersionContractKey)
        );
        assert!(CEP78Error::from_user_code(0).is_none());
        assert!(CEP78Error::from_user_code(181).is_none());
    }
}
