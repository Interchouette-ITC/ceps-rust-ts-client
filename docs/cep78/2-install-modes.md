# Install modes

Required install args: `collection_name`, `collection_symbol`, `total_token_supply`, `ownership_mode`, `nft_metadata_kind`, `identifier_mode`, `metadata_mutability`.

| Enum | Values (`u8`) |
| --- | --- |
| `OwnershipMode` | Minter=0, Assigned=1, Transferable=2 |
| `NftKind` | Physical=0, Digital=1, Virtual=2 |
| `HolderMode` | Accounts=0, Contracts=1, Mixed=2 |
| `NftMetadataKind` | Cep78=0, Nft721=1, Raw=2, CustomValidated=3 |
| `IdentifierMode` | Ordinal=0, Hash=1 |
| `MetadataMutability` | Immutable=0, Mutable=1 |
| `MintingMode` | Installer=0, Public=1, Acl=2 |
| `BurnMode` | Burnable=0, NonBurnable=1 |
| `WhitelistMode` | Unlocked=0, Locked=1 |
| `OwnerReverseLookupMode` | NoLookup=0, Complete=1, TransfersOnly=2 |
| `NamedKeyConventionMode` | DerivedFromCollectionName=0, V1_0Standard=1, V1_0Custom=2 |
| `EventsMode78` | NoEvents=0, Cep47=1, Ces=2, Native=3, NativeBytes=4 |

Default `InstallArgs::new` picks Transferable + Raw + Ordinal + Immutable + Digital + Accounts + Burnable + NoLookup.

Named keys (DerivedFromCollectionName): `cep78_contract_hash_{name}`, `cep78_contract_package_{name}`, access + version siblings.

Upgrade uses the same installer WASM with `collection_name` and optional supply / events / package flags.
