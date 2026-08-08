# Errors

CEP-18 user errors start at **60000**. Parsed from execution messages containing `User error: N`.

| Code | Name |
| --- | --- |
| 60000 | InvalidContext |
| 60001 | InsufficientBalance |
| 60002 | InsufficientAllowance |
| 60003 | Overflow |
| 60004 | PackageHashMissing |
| 60005 | PackageHashNotPackage |
| 60006 | InvalidEventsMode |
| 60007 | MissingEventsMode |
| 60008 | Phantom |
| 60009 | FailedToGetArgBytes |
| 60010 | InsufficientRights |
| 60011 | InvalidAdminList |
| 60012 | InvalidMinterList |
| 60013 | InvalidNoneList |
| 60014 | InvalidEnableMbFlag |
| 60015 | AlreadyInitialized |
| 60016 | MintBurnDisabled |
| 60017 | CannotTargetSelfUser |
| 60018 | InvalidBurnTarget |
| 60019–60038 | Upgrade / storage / uref failures |

Typed map: `ceps_client::cep18::Cep18Error::from_code(u16)`.
