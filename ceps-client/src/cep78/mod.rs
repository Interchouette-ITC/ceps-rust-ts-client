//! CEP-78 enhanced NFT client.

mod error;
mod keys;
mod modes;
mod types;

pub use error::CEP78Error;
pub use keys::{key_hex_body, operator_dictionary_key, prefixed_key};
pub use modes::{
    BurnMode, HolderMode, IdentifierMode, MetadataMutability, MintingMode, NamedKeyConventionMode,
    NftKind, NftMetadataKind, OwnerReverseLookupMode, OwnershipMode, WhitelistMode,
};
pub use types::{InstallArgs, SetVariablesArgs, TokenIdentifier, UpgradeArgs};

use crate::core::CEPClient;
use crate::core::{json_args, key_arg, string_arg, JsonArg};
use crate::error::{CEPError, CEPKind, Result};
use crate::schema::arg;
use crate::schema::cep78_fields as sch;
use crate::types::{CallResult, EventsMode78, TransactionParams};
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use serde_json::{json, Value};

/// Client for CEP-78 enhanced NFT contracts.
pub struct CEP78Client {
    core: CEPClient,
}

impl CEP78Client {
    /// Create a CEP-78 client.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let core =
            CEPClient::new(rpc_url, sse_url, chain_name, verbosity)?.with_cep_kind(CEPKind::CEP78);
        Ok(Self { core })
    }

    /// Borrow the shared core.
    pub fn core(&self) -> &CEPClient {
        &self.core
    }

    /// Mutable core access.
    pub fn core_mut(&mut self) -> &mut CEPClient {
        &mut self.core
    }

    /// RPC URL.
    pub fn rpc_url(&self) -> &str {
        self.core.rpc_url()
    }

    /// SSE URL when set.
    pub fn sse_url(&self) -> Option<&str> {
        self.core.sse_url()
    }

    /// Chain name.
    pub fn chain_name(&self) -> &str {
        self.core.chain_name()
    }

    /// Verbosity.
    pub fn verbosity(&self) -> Verbosity {
        self.core.verbosity()
    }

    /// Set RPC URL.
    pub fn set_rpc_url(&mut self, rpc_url: impl Into<String>) -> Result<()> {
        self.core.set_rpc_url(rpc_url)
    }

    /// Set SSE URL.
    pub fn set_sse_url(&mut self, sse_url: impl Into<String>) -> Result<()> {
        self.core.set_sse_url(sse_url)
    }

    /// Set chain name.
    pub fn set_chain_name(&mut self, chain_name: impl Into<String>) {
        self.core.set_chain_name(chain_name);
    }

    /// Set verbosity.
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.core.set_verbosity(verbosity);
    }

    /// Bind contract and optional package hash.
    pub fn set_contract_hash(
        &mut self,
        contract_hash: impl AsRef<str>,
        package_hash: Option<impl AsRef<str>>,
    ) -> Result<()> {
        self.core.set_contract_hash(contract_hash, package_hash)
    }

    /// Read a named key from an account (public key hex or `account-hash-…`).
    ///
    /// Typical post-install: `cep78_contract_hash_{name}` / `cep78_contract_package_{name}`.
    pub async fn get_account_named_key(
        &self,
        account_identifier: &str,
        named_key: &str,
    ) -> Result<String> {
        self.core
            .get_account_named_key(account_identifier, named_key)
            .await
    }

    /// Install a CEP-78 contract.
    pub async fn install(
        &self,
        args: &InstallArgs,
        wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        self.core
            .install_wasm(wasm, tx, &install_args_json(args)?)
            .await
    }

    /// Upgrade an existing CEP-78 package (same installer WASM).
    pub async fn upgrade(
        &self,
        args: &UpgradeArgs,
        wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![arg(
            &sch::ep::UPGRADE_COLLECTION_NAME,
            Value::String(args.collection_name.clone()),
        )];
        if let Some(supply) = args.total_token_supply {
            v.push(arg(&sch::ep::UPGRADE_TOTAL_TOKEN_SUPPLY, json!(supply)));
        }
        if let Some(mode) = args.events_mode {
            v.push(arg(&sch::ep::UPGRADE_EVENTS_MODE, json!(u8::from(mode))));
        }
        if let Some(b) = args.acl_package_mode {
            v.push(arg(&sch::ep::UPGRADE_ACL_PACKAGE_MODE, json!(b)));
        }
        if let Some(b) = args.package_operator_mode {
            v.push(arg(&sch::ep::UPGRADE_PACKAGE_OPERATOR_MODE, json!(b)));
        }
        if let Some(b) = args.operator_burn_mode {
            v.push(arg(&sch::ep::UPGRADE_OPERATOR_BURN_MODE, json!(b)));
        }
        self.core.install_wasm(wasm, tx, &json_args(&v)).await
    }

    /// Mint via entrypoint.
    pub async fn mint(
        &self,
        token_owner: &str,
        token_meta_data: &str,
        token_hash: Option<&str>,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            arg(
                &sch::ep::MINT_TOKEN_OWNER,
                Value::String(prefixed_key(token_owner)?),
            ),
            arg(
                &sch::ep::MINT_TOKEN_META_DATA,
                Value::String(token_meta_data.to_string()),
            ),
        ];
        if let Some(hash) = token_hash {
            v.push(arg(
                &sch::ep::MINT_TOKEN_HASH,
                Value::String(hash.to_string()),
            ));
        }
        self.core.call_entrypoint("mint", tx, &json_args(&v)).await
    }

    /// Mint via `mint_session.wasm` (registers owner + writes receipts).
    pub async fn mint_session(
        &self,
        token_owner: &str,
        token_meta_data: &str,
        token_hash: Option<&str>,
        session_wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            key_arg("token_owner", &prefixed_key(token_owner)?),
            string_arg("token_meta_data", token_meta_data),
            key_arg("nft_contract_hash", &self.contract_hash_key()?),
        ];
        if let Some(hash) = token_hash {
            v.push(string_arg("token_hash", hash));
        }
        self.core
            .call_session(session_wasm, tx, &json_args(&v))
            .await
    }

    /// Burn a token.
    pub async fn burn(
        &self,
        token: &TokenIdentifier,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = token_args(token)?;
        self.core.call_entrypoint("burn", tx, &json_args(&v)).await
    }

    /// Transfer a token.
    pub async fn transfer(
        &self,
        source: &str,
        target: &str,
        token: &TokenIdentifier,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            arg(
                &sch::ep::TRANSFER_SOURCE,
                Value::String(prefixed_key(source)?),
            ),
            arg(
                &sch::ep::TRANSFER_TARGET,
                Value::String(prefixed_key(target)?),
            ),
        ];
        v.extend(token_args(token)?);
        self.core
            .call_entrypoint("transfer", tx, &json_args(&v))
            .await
    }

    /// Transfer via `transfer_session.wasm`.
    pub async fn transfer_session(
        &self,
        source: &str,
        target: &str,
        token: &TokenIdentifier,
        session_wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            key_arg("source_key", &prefixed_key(source)?),
            key_arg("target_key", &prefixed_key(target)?),
            key_arg("nft_contract_hash", &self.contract_hash_key()?),
        ];
        v.extend(token_args(token)?);
        self.core
            .call_session(session_wasm, tx, &json_args(&v))
            .await
    }

    /// Register an owner for reverse-lookup pages.
    pub async fn register_owner(
        &self,
        token_owner: &str,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![arg(
            &sch::ep::REGISTER_OWNER_TOKEN_OWNER,
            Value::String(prefixed_key(token_owner)?),
        )];
        self.core
            .call_entrypoint("register_owner", tx, &json_args(&v))
            .await
    }

    /// Approve an operator for one token.
    pub async fn approve(
        &self,
        operator: &str,
        token: &TokenIdentifier,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![arg(
            &sch::ep::APPROVE_OPERATOR,
            Value::String(prefixed_key(operator)?),
        )];
        v.extend(token_args(token)?);
        self.core
            .call_entrypoint("approve", tx, &json_args(&v))
            .await
    }

    /// Revoke approval for one token.
    pub async fn revoke(
        &self,
        operator: &str,
        token: &TokenIdentifier,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![arg(
            &sch::ep::APPROVE_OPERATOR,
            Value::String(prefixed_key(operator)?),
        )];
        v.extend(token_args(token)?);
        self.core
            .call_entrypoint("revoke", tx, &json_args(&v))
            .await
    }

    /// Set approval for all.
    pub async fn set_approval_for_all(
        &self,
        operator: &str,
        approve_all: bool,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            arg(
                &sch::ep::SET_APPROVAL_OPERATOR,
                Value::String(prefixed_key(operator)?),
            ),
            arg(&sch::ep::SET_APPROVAL_APPROVE_ALL, json!(approve_all)),
        ];
        self.core
            .call_entrypoint("set_approval_for_all", tx, &json_args(&v))
            .await
    }

    /// Update token metadata (mutable collections only).
    pub async fn set_token_metadata(
        &self,
        token_meta_data: &str,
        token: &TokenIdentifier,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![arg(
            &sch::ep::SET_META_DATA,
            Value::String(token_meta_data.to_string()),
        )];
        v.extend(token_args(token)?);
        self.core
            .call_entrypoint("set_token_metadata", tx, &json_args(&v))
            .await
    }

    /// Update collection variables.
    pub async fn set_variables(
        &self,
        args: &SetVariablesArgs,
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v: Vec<JsonArg> = Vec::new();
        if let Some(b) = args.allow_minting {
            v.push(arg(&sch::ep::SV_ALLOW_MINTING, json!(b)));
        }
        if let Some(list) = &args.acl_whitelist {
            let keys: Result<Vec<_>> = list.iter().map(|k| prefixed_key(k)).collect();
            v.push(arg(&sch::ep::SV_ACL_WHITELIST, json!(keys?)));
        }
        if let Some(b) = args.acl_package_mode {
            v.push(arg(&sch::ep::SV_ACL_PACKAGE_MODE, json!(b)));
        }
        if let Some(b) = args.package_operator_mode {
            v.push(arg(&sch::ep::SV_PACKAGE_OPERATOR_MODE, json!(b)));
        }
        if let Some(b) = args.operator_burn_mode {
            v.push(arg(&sch::ep::SV_OPERATOR_BURN_MODE, json!(b)));
        }
        if v.is_empty() {
            return Err(CEPError::MissingArgument(
                "set_variables requires at least one field".into(),
            ));
        }
        self.core
            .call_entrypoint("set_variables", tx, &json_args(&v))
            .await
    }

    /// Refresh receipt pages via `updated_receipts.wasm` (package hash as `nft_contract_hash`).
    pub async fn updated_receipts(
        &self,
        session_wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let package = self
            .core
            .require_target()?
            .package_hash
            .as_ref()
            .ok_or_else(|| CEPError::MissingArgument("package hash required".into()))?;
        let v = vec![key_arg("nft_contract_hash", &format!("hash-{package}"))];
        self.core
            .call_session(session_wasm, tx, &json_args(&v))
            .await
    }

    /// Collection name.
    pub async fn collection_name(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["collection_name"]).await?)
    }

    /// Collection symbol.
    pub async fn collection_symbol(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["collection_symbol"]).await?)
    }

    /// Total token supply.
    pub async fn total_token_supply(&self) -> Result<u64> {
        decode_u64_cl(
            self.core
                .query_contract_key(&["total_token_supply"])
                .await?,
        )
    }

    /// Number of minted tokens.
    pub async fn number_of_minted_tokens(&self) -> Result<u64> {
        decode_u64_cl(
            self.core
                .query_contract_key(&["number_of_minted_tokens"])
                .await?,
        )
    }

    /// Events mode.
    pub async fn events_mode(&self) -> Result<EventsMode78> {
        let v = decode_u8_cl(self.core.query_contract_key(&["events_mode"]).await?)?;
        EventsMode78::from_u8(v).ok_or_else(|| CEPError::Decode(format!("unknown events_mode {v}")))
    }

    /// Whether minting is currently allowed.
    pub async fn allow_minting(&self) -> Result<bool> {
        decode_bool_cl(self.core.query_contract_key(&["allow_minting"]).await?)
    }

    /// Minting mode.
    pub async fn minting_mode(&self) -> Result<MintingMode> {
        mode_u8(self, "minting_mode", MintingMode::from_u8).await
    }

    /// Whitelist mode.
    pub async fn whitelist_mode(&self) -> Result<WhitelistMode> {
        mode_u8(self, "whitelist_mode", WhitelistMode::from_u8).await
    }

    /// Owner reverse-lookup / reporting mode (`reporting_mode` named key).
    pub async fn reporting_mode(&self) -> Result<OwnerReverseLookupMode> {
        mode_u8(self, "reporting_mode", OwnerReverseLookupMode::from_u8).await
    }

    /// Burn mode.
    pub async fn burn_mode(&self) -> Result<BurnMode> {
        mode_u8(self, "burn_mode", BurnMode::from_u8).await
    }

    /// Whether operators may burn.
    pub async fn operator_burn_mode(&self) -> Result<bool> {
        decode_bool_cl(
            self.core
                .query_contract_key(&["operator_burn_mode"])
                .await?,
        )
    }

    /// Holder mode.
    pub async fn holder_mode(&self) -> Result<HolderMode> {
        mode_u8(self, "holder_mode", HolderMode::from_u8).await
    }

    /// Identifier mode.
    pub async fn identifier_mode(&self) -> Result<IdentifierMode> {
        mode_u8(self, "identifier_mode", IdentifierMode::from_u8).await
    }

    /// Metadata mutability.
    pub async fn metadata_mutability(&self) -> Result<MetadataMutability> {
        mode_u8(self, "metadata_mutability", MetadataMutability::from_u8).await
    }

    /// NFT kind.
    pub async fn nft_kind(&self) -> Result<NftKind> {
        mode_u8(self, "nft_kind", NftKind::from_u8).await
    }

    /// NFT metadata kind.
    pub async fn nft_metadata_kind(&self) -> Result<NftMetadataKind> {
        mode_u8(self, "nft_metadata_kind", NftMetadataKind::from_u8).await
    }

    /// Ownership mode.
    pub async fn ownership_mode(&self) -> Result<OwnershipMode> {
        mode_u8(self, "ownership_mode", OwnershipMode::from_u8).await
    }

    /// Whether package-level operators are enabled.
    pub async fn package_operator_mode(&self) -> Result<bool> {
        decode_bool_cl(
            self.core
                .query_contract_key(&["package_operator_mode"])
                .await?,
        )
    }

    /// Whether ACL whitelist entries may be packages.
    pub async fn acl_package_mode(&self) -> Result<bool> {
        decode_bool_cl(self.core.query_contract_key(&["acl_package_mode"]).await?)
    }

    /// JSON schema string (custom validated metadata installs).
    pub async fn json_schema(&self) -> Result<String> {
        decode_string_cl(self.core.query_contract_key(&["json_schema"]).await?)
    }

    /// Whether `entity` appears in the ACL whitelist dictionary.
    pub async fn is_acl_whitelisted(&self, entity: &str) -> Result<bool> {
        let item = key_hex_body(entity)?;
        match self.core.query_dictionary("acl_whitelist", &item).await {
            Ok(raw) => decode_bool_cl(raw),
            Err(CEPError::EmptyQuery(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Persist owner under caller named key via `owner_of_session.wasm`.
    pub async fn owner_of_session(
        &self,
        token: &TokenIdentifier,
        key_name: &str,
        session_wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            key_arg("nft_contract_hash", &self.contract_hash_key()?),
            string_arg("key_name", key_name),
        ];
        v.extend(token_args(token)?);
        self.core
            .call_session(session_wasm, tx, &json_args(&v))
            .await
    }

    /// Persist balance under caller named key via `balance_of_session.wasm`.
    pub async fn balance_of_session(
        &self,
        token_owner: &str,
        key_name: &str,
        session_wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("nft_contract_hash", &self.contract_hash_key()?),
            key_arg("token_owner", &prefixed_key(token_owner)?),
            string_arg("key_name", key_name),
        ];
        self.core
            .call_session(session_wasm, tx, &json_args(&v))
            .await
    }

    /// Persist approval under caller named key via `get_approved_session.wasm`.
    pub async fn get_approved_session(
        &self,
        token: &TokenIdentifier,
        key_name: &str,
        session_wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let mut v = vec![
            key_arg("nft_contract_hash", &self.contract_hash_key()?),
            string_arg("key_name", key_name),
        ];
        v.extend(token_args(token)?);
        self.core
            .call_session(session_wasm, tx, &json_args(&v))
            .await
    }

    /// Persist operator flag under caller named key via `is_approved_for_all_session.wasm`.
    pub async fn is_approved_for_all_session(
        &self,
        token_owner: &str,
        operator: &str,
        key_name: &str,
        session_wasm: &[u8],
        tx: &TransactionParams,
    ) -> Result<CallResult> {
        let v = vec![
            key_arg("nft_contract_hash", &self.contract_hash_key()?),
            key_arg("token_owner", &prefixed_key(token_owner)?),
            key_arg("operator", &prefixed_key(operator)?),
            string_arg("key_name", key_name),
        ];
        self.core
            .call_session(session_wasm, tx, &json_args(&v))
            .await
    }

    /// Owner of a token (dictionary query).
    pub async fn owner_of(&self, token: &TokenIdentifier) -> Result<String> {
        let item = token_item_key(token);
        decode_key_cl(self.core.query_dictionary("token_owners", &item).await?)
    }

    /// Balance of an owner (dictionary query). Returns `"0"` when missing.
    pub async fn balance_of(&self, owner: &str) -> Result<String> {
        let item = key_hex_body(owner)?;
        match self.core.query_dictionary("balances", &item).await {
            Ok(raw) => decode_u64_string(raw),
            Err(CEPError::EmptyQuery(_)) => Ok("0".into()),
            Err(e) => Err(e),
        }
    }

    /// Approved operator for a token, if any.
    pub async fn get_approved(&self, token: &TokenIdentifier) -> Result<Option<String>> {
        let item = token_item_key(token);
        match self.core.query_dictionary("approved", &item).await {
            Ok(raw) => Ok(Some(decode_key_cl(raw)?)),
            Err(CEPError::EmptyQuery(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Whether `operator` is approved for all of `owner`.
    pub async fn is_approved_for_all(&self, owner: &str, operator: &str) -> Result<bool> {
        let item = operator_dictionary_key(owner, operator)?;
        match self.core.query_dictionary("operators", &item).await {
            Ok(raw) => decode_bool_cl(raw),
            Err(CEPError::EmptyQuery(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Token metadata for the given kind dictionary.
    pub async fn metadata(&self, token: &TokenIdentifier, kind: NftMetadataKind) -> Result<String> {
        let dict = match kind {
            NftMetadataKind::CEP78 => "metadata_cep78",
            NftMetadataKind::Nft721 => "metadata_nft721",
            NftMetadataKind::Raw => "metadata_raw",
            NftMetadataKind::CustomValidated => "metadata_custom_validated",
        };
        decode_string_cl(
            self.core
                .query_dictionary(dict, &token_item_key(token))
                .await?,
        )
    }

    fn contract_hash_key(&self) -> Result<String> {
        Ok(format!(
            "hash-{}",
            self.core.require_target()?.contract_hash
        ))
    }
}

fn install_args_json(args: &InstallArgs) -> Result<String> {
    let mut v = vec![
        arg(
            &sch::install::COLLECTION_NAME,
            Value::String(args.collection_name.clone()),
        ),
        arg(
            &sch::install::COLLECTION_SYMBOL,
            Value::String(args.collection_symbol.clone()),
        ),
        arg(
            &sch::install::TOTAL_TOKEN_SUPPLY,
            json!(args.total_token_supply),
        ),
        arg(
            &sch::install::OWNERSHIP_MODE,
            json!(u8::from(args.ownership_mode)),
        ),
        arg(
            &sch::install::NFT_METADATA_KIND,
            json!(u8::from(args.nft_metadata_kind)),
        ),
        arg(
            &sch::install::IDENTIFIER_MODE,
            json!(u8::from(args.identifier_mode)),
        ),
        arg(
            &sch::install::METADATA_MUTABILITY,
            json!(u8::from(args.metadata_mutability)),
        ),
    ];
    if let Some(kind) = args.nft_kind {
        v.push(arg(&sch::install::NFT_KIND, json!(u8::from(kind))));
    }
    if let Some(schema) = &args.json_schema {
        v.push(arg(
            &sch::install::JSON_SCHEMA,
            Value::String(schema.clone()),
        ));
    }
    if let Some(mode) = args.minting_mode {
        v.push(arg(&sch::install::MINTING_MODE, json!(u8::from(mode))));
    }
    if let Some(b) = args.allow_minting {
        v.push(arg(&sch::install::ALLOW_MINTING, json!(b)));
    }
    if let Some(b) = args.operator_burn_mode {
        v.push(arg(&sch::install::OPERATOR_BURN_MODE, json!(b)));
    }
    if let Some(b) = args.package_operator_mode {
        v.push(arg(&sch::install::PACKAGE_OPERATOR_MODE, json!(b)));
    }
    if let Some(mode) = args.whitelist_mode {
        v.push(arg(&sch::install::WHITELIST_MODE, json!(u8::from(mode))));
    }
    if let Some(mode) = args.holder_mode {
        v.push(arg(&sch::install::HOLDER_MODE, json!(u8::from(mode))));
    }
    if let Some(b) = args.acl_package_mode {
        v.push(arg(&sch::install::ACL_PACKAGE_MODE, json!(b)));
    }
    if let Some(list) = &args.acl_whitelist {
        let keys: Result<Vec<_>> = list.iter().map(|k| prefixed_key(k)).collect();
        v.push(arg(&sch::install::ACL_WHITELIST, json!(keys?)));
    }
    if let Some(mode) = args.burn_mode {
        v.push(arg(&sch::install::BURN_MODE, json!(u8::from(mode))));
    }
    if let Some(mode) = args.owner_reverse_lookup_mode {
        v.push(arg(
            &sch::install::OWNER_REVERSE_LOOKUP_MODE,
            json!(u8::from(mode)),
        ));
    }
    if let Some(mode) = args.named_key_convention {
        v.push(arg(
            &sch::install::NAMED_KEY_CONVENTION,
            json!(u8::from(mode)),
        ));
    }
    if let Some(name) = &args.access_key_name {
        v.push(arg(
            &sch::install::ACCESS_KEY_NAME,
            Value::String(name.clone()),
        ));
    }
    if let Some(name) = &args.hash_key_name {
        v.push(arg(
            &sch::install::HASH_KEY_NAME,
            Value::String(name.clone()),
        ));
    }
    if let Some(mode) = args.events_mode {
        v.push(arg(&sch::install::EVENTS_MODE, json!(u8::from(mode))));
    }
    if let Some(filter) = &args.transfer_filter_contract {
        v.push(arg(
            &sch::install::TRANSFER_FILTER_CONTRACT,
            Value::String(prefixed_key(filter)?),
        ));
    }
    if matches!(
        args.named_key_convention,
        Some(NamedKeyConventionMode::V1_0Custom)
    ) && (args.access_key_name.is_none() || args.hash_key_name.is_none())
    {
        return Err(CEPError::MissingArgument(
            "V1_0Custom requires access_key_name and hash_key_name".into(),
        ));
    }
    Ok(json_args(&v))
}

fn token_args(token: &TokenIdentifier) -> Result<Vec<JsonArg>> {
    Ok(match token {
        TokenIdentifier::Id(id) => vec![arg(&sch::ep::TOKEN_ID, json!(*id))],
        TokenIdentifier::Hash(hash) => {
            vec![arg(&sch::ep::TOKEN_HASH, Value::String(hash.clone()))]
        }
    })
}

fn token_item_key(token: &TokenIdentifier) -> String {
    match token {
        TokenIdentifier::Id(id) => id.to_string(),
        TokenIdentifier::Hash(hash) => hash.clone(),
    }
}

fn decode_string_cl(value: Value) -> Result<String> {
    if let Some(s) = value
        .pointer("/stored_value/CLValue/parsed")
        .and_then(|v| v.as_str())
    {
        return Ok(s.to_string());
    }
    if let Some(s) = value.pointer("/CLValue/parsed").and_then(|v| v.as_str()) {
        return Ok(s.to_string());
    }
    Err(CEPError::Decode(format!("expected string, got {value}")))
}

fn decode_u8_cl(value: Value) -> Result<u8> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Number(n) => n
            .as_u64()
            .map(|v| v as u8)
            .ok_or_else(|| CEPError::Decode(format!("expected u8, got {n}"))),
        Value::String(s) => s
            .parse()
            .map_err(|e| CEPError::Decode(format!("expected u8: {e}"))),
        other => Err(CEPError::Decode(format!("expected u8, got {other}"))),
    }
}

fn decode_u64_cl(value: Value) -> Result<u64> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| CEPError::Decode(format!("expected u64, got {n}"))),
        Value::String(s) => s
            .parse()
            .map_err(|e| CEPError::Decode(format!("expected u64: {e}"))),
        other => Err(CEPError::Decode(format!("expected u64, got {other}"))),
    }
}

fn decode_u64_string(value: Value) -> Result<String> {
    Ok(decode_u64_cl(value)?.to_string())
}

fn decode_bool_cl(value: Value) -> Result<bool> {
    let parsed = value
        .pointer("/stored_value/CLValue/parsed")
        .or_else(|| value.pointer("/CLValue/parsed"))
        .cloned()
        .unwrap_or(value);
    match parsed {
        Value::Bool(b) => Ok(b),
        Value::Number(n) => Ok(n.as_u64().unwrap_or(0) != 0),
        Value::String(s) => match s.to_ascii_lowercase().as_str() {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            other => Err(CEPError::Decode(format!(
                "expected bool string, got {other}"
            ))),
        },
        other => Err(CEPError::Decode(format!("expected bool, got {other}"))),
    }
}

async fn mode_u8<T, F>(client: &CEP78Client, named_key: &str, map: F) -> Result<T>
where
    F: FnOnce(u8) -> Option<T>,
{
    let v = decode_u8_cl(client.core.query_contract_key(&[named_key]).await?)?;
    map(v).ok_or_else(|| CEPError::Decode(format!("unknown {named_key} {v}")))
}

fn decode_key_cl(value: Value) -> Result<String> {
    if let Some(s) = value
        .pointer("/stored_value/CLValue/parsed")
        .and_then(|v| v.as_str())
    {
        return Ok(s.to_string());
    }
    if let Some(obj) = value.pointer("/stored_value/CLValue/parsed") {
        if let Some(s) = obj.as_str() {
            return Ok(s.to_string());
        }
        return Ok(obj.to_string());
    }
    if let Some(s) = value.pointer("/CLValue/parsed").and_then(|v| v.as_str()) {
        return Ok(s.to_string());
    }
    Err(CEPError::Decode(format!("expected Key, got {value}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_client() {
        let client =
            CEP78Client::new("http://127.0.0.1:11101", None, None, Some(Verbosity::High)).unwrap();
        assert_eq!(client.rpc_url(), "http://127.0.0.1:11101/rpc");
        assert!(client.sse_url().is_none());
    }

    #[test]
    fn install_json_includes_required() {
        let args = InstallArgs::new("Col", "COL", 100).with_events_mode(EventsMode78::CES);
        let s = install_args_json(&args).unwrap();
        assert!(s.contains("collection_name"));
        assert!(s.contains("events_mode"));
        crate::schema::assert_install_json_matches_schema(crate::schema::CepId::Cep78, &s);
    }

    #[test]
    fn token_id_args() {
        let v = token_args(&TokenIdentifier::id(7)).unwrap();
        let s = json_args(&v);
        assert!(s.contains("token_id"));
        assert!(s.contains('7'));
    }

    #[tokio::test]
    async fn make_only_install_returns_transaction_json() {
        let client = CEP78Client::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let tx = TransactionParams::for_make("1000000000").with_initiator_addr(
            "010101010101010101010101010101010101010101010101010101010101010101",
        );
        let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let args = InstallArgs::new("Col", "COL", 50);
        let result = client
            .install(&args, &wasm, &tx)
            .await
            .expect("make-only install");
        assert!(result.put_result.is_null());
        assert!(result.transaction.is_some());
        assert!(!result.transaction_hash.is_empty());
    }

    #[tokio::test]
    async fn make_only_mint_returns_transaction_json() {
        let mut client = CEP78Client::new("http://127.0.0.1:11101", None, None, None).unwrap();
        client
            .set_contract_hash(
                "cfa781f5eb69c3eee952c2944ce9670a049f88c5e46b83fb5881ebe13fb98e6d",
                None::<&str>,
            )
            .unwrap();
        let tx = TransactionParams::for_make("1000000000").with_initiator_addr(
            "010101010101010101010101010101010101010101010101010101010101010101",
        );
        let result = client
            .mint(
                "account-hash-b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f",
                "meta",
                None,
                &tx,
            )
            .await
            .expect("make-only mint");
        assert!(result.put_result.is_null());
        assert!(result.transaction.expect("json").is_object());
    }
}
