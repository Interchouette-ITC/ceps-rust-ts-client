//! Shared transport, install, call, query, and wait helpers.

mod args;
mod call;
mod install;
mod query;
mod urls;

pub use args::{
    bool_arg, json_arg, json_args, key_arg, key_list_arg, option_byte_list_arg, string_arg,
    string_pair_list_arg, u256_arg, u256_list_arg, u64_arg, u8_arg, JsonArg,
};
pub use urls::{normalize_rpc_url, normalize_sse_url};

use crate::error::{CEPError, CEPKind, Result};
use crate::types::{CallResult, ContractTarget, TransactionParams};
use casper_rust_wasm_sdk::types::cl::bytes::Bytes;
use casper_rust_wasm_sdk::types::hash::addressable_entity_hash::AddressableEntityHash;
use casper_rust_wasm_sdk::types::hash::package_hash::PackageHash;
use casper_rust_wasm_sdk::types::hash::transaction_hash::TransactionHash;
use casper_rust_wasm_sdk::types::transaction::Transaction;
use casper_rust_wasm_sdk::types::transaction_params::transaction_builder_params::TransactionBuilderParams;
use casper_rust_wasm_sdk::types::transaction_params::transaction_str_params::TransactionStrParams;
use casper_rust_wasm_sdk::types::verbosity::Verbosity;
use casper_rust_wasm_sdk::SDK;
use casper_rust_wasm_sdk::SSE::{CESEvent, CESParseResult, CESParser, EventName, RawEvent};
use serde_json::Value;

/// Default NCTL RPC endpoint.
pub const DEFAULT_RPC_URL: &str = "http://127.0.0.1:11101";
/// Default NCTL SSE events endpoint.
pub const DEFAULT_SSE_URL: &str = "http://127.0.0.1:18101/events";
/// Default NCTL chain name.
pub const DEFAULT_CHAIN_NAME: &str = "casper-net-1";
/// Default wait timeout (matches JS clients): 120 seconds.
pub const DEFAULT_WAIT_TIMEOUT_MS: u64 = 120_000;

/// Shared CEP client: endpoints, SDK handle, and contract targeting.
pub struct CEPClient {
    sdk: SDK,
    rpc_url: String,
    sse_url: Option<String>,
    chain_name: String,
    verbosity: Verbosity,
    target: Option<ContractTarget>,
    /// Use VmCasperV2 session runtime when `Some(true)`.
    runtime_v2: Option<bool>,
    cep_kind: Option<CEPKind>,
}

impl CEPClient {
    /// Create a shared CEP client.
    ///
    /// `rpc_url` is required. Empty `sse_url` is treated as unset.
    pub fn new(
        rpc_url: impl Into<String>,
        sse_url: Option<String>,
        chain_name: Option<String>,
        verbosity: Option<Verbosity>,
    ) -> Result<Self> {
        let rpc_url = normalize_rpc_url(&rpc_url.into())?;
        let sse_url = match sse_url {
            Some(s) if !s.trim().is_empty() => Some(normalize_sse_url(&s)?),
            _ => None,
        };
        let verbosity = verbosity.unwrap_or(Verbosity::Low);
        let sdk = SDK::new(Some(rpc_url.clone()), None, Some(verbosity));
        Ok(Self {
            sdk,
            rpc_url,
            sse_url,
            chain_name: chain_name.unwrap_or_else(|| DEFAULT_CHAIN_NAME.to_string()),
            verbosity,
            target: None,
            runtime_v2: Some(false),
            cep_kind: None,
        })
    }

    /// Tag this core with a CEP kind (used when mapping user errors).
    pub fn with_cep_kind(mut self, kind: CEPKind) -> Self {
        self.cep_kind = Some(kind);
        self
    }

    /// Set session runtime to VmCasperV2 (`true`) or VmCasperV1 (`false`).
    pub fn set_runtime_v2(&mut self, runtime_v2: bool) {
        self.runtime_v2 = Some(runtime_v2);
    }

    /// Borrow the underlying SDK (crate-internal).
    pub(crate) fn sdk(&self) -> &SDK {
        &self.sdk
    }

    /// RPC URL (always ends with `/rpc`).
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// SSE URL when configured.
    pub fn sse_url(&self) -> Option<&str> {
        self.sse_url.as_deref()
    }

    /// Chain name used for transactions.
    pub fn chain_name(&self) -> &str {
        &self.chain_name
    }

    /// Current verbosity.
    pub fn verbosity(&self) -> Verbosity {
        self.verbosity
    }

    /// Bound contract identity after [`Self::set_contract_hash`], if any.
    ///
    /// Holds the contract (entity) hash and optional package hash used for
    /// entrypoint calls, queries, and CES helpers.
    pub fn target(&self) -> Option<&ContractTarget> {
        self.target.as_ref()
    }

    /// Update RPC URL.
    pub fn set_rpc_url(&mut self, rpc_url: impl Into<String>) -> Result<()> {
        let rpc_url = normalize_rpc_url(&rpc_url.into())?;
        self.sdk
            .set_rpc_address(Some(rpc_url.clone()))
            .map_err(CEPError::Other)?;
        self.rpc_url = rpc_url;
        Ok(())
    }

    /// Update SSE URL.
    pub fn set_sse_url(&mut self, sse_url: impl Into<String>) -> Result<()> {
        self.sse_url = Some(normalize_sse_url(&sse_url.into())?);
        Ok(())
    }

    /// Update chain name.
    pub fn set_chain_name(&mut self, chain_name: impl Into<String>) {
        self.chain_name = chain_name.into();
    }

    /// Update verbosity on the core and SDK handle.
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
        let _ = self.sdk.set_verbosity(Some(verbosity));
    }

    /// Bind contract (and optional package) hashes.
    pub fn set_contract_hash(
        &mut self,
        contract_hash: impl AsRef<str>,
        package_hash: Option<impl AsRef<str>>,
    ) -> Result<()> {
        self.target = Some(ContractTarget::new(contract_hash, package_hash)?);
        Ok(())
    }

    /// Require a bound contract target.
    pub(crate) fn require_target(&self) -> Result<&ContractTarget> {
        self.target.as_ref().ok_or(CEPError::ContractHashMissing)
    }

    /// Build [`TransactionStrParams`] from transaction options and JSON session args.
    pub(crate) fn build_tx_params(
        &self,
        tx: &TransactionParams,
        args_json: &str,
    ) -> Result<TransactionStrParams> {
        tx.validate().map_err(CEPError::InvalidArgument)?;
        let params = TransactionStrParams::default();
        let chain = tx.chain_name.as_deref().unwrap_or(self.chain_name.as_str());
        params.set_chain_name(chain);
        params.set_payment_amount(&tx.payment_amount);
        if let Some(pem) = tx
            .secret_key_pem
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            params.set_secret_key(pem);
        }
        if let Some(addr) = tx
            .initiator_addr
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            params.set_initiator_addr(addr);
        }
        if !args_json.is_empty() {
            params.set_session_args_json(args_json);
        }
        Ok(params)
    }

    /// Make a Transaction without putting it; serialize via SDK `to_json_string`.
    pub(crate) fn make_only_result(
        &self,
        builder: TransactionBuilderParams,
        params: TransactionStrParams,
    ) -> Result<CallResult> {
        let transaction = self.sdk().make_transaction(builder, params)?;
        let hash = transaction.hash().to_string();
        let json_str = transaction
            .to_json_string()
            .map_err(|e| CEPError::Other(format!("serialize transaction: {e}")))?;
        let value: Value = serde_json::from_str(&json_str)
            .map_err(|e| CEPError::Other(format!("parse transaction JSON: {e}")))?;
        Ok(CallResult::from_make(hash, value))
    }

    /// Put an already-signed Transaction JSON (for example from make-only after external signing).
    ///
    /// When `wait` is true, waits on SSE and attaches execution (and CES when a contract is
    /// bound), matching the normal CEP put path.
    pub async fn put_transaction(
        &self,
        transaction: &Value,
        wait: bool,
        wait_timeout_ms: Option<u64>,
    ) -> Result<CallResult> {
        let json_str = serde_json::to_string(transaction)
            .map_err(|e| CEPError::Other(format!("serialize transaction JSON: {e}")))?;
        let tx = Transaction::from_json_string(&json_str)
            .map_err(|e| CEPError::Other(format!("parse transaction JSON: {e}")))?;
        let put = self
            .sdk
            .put_transaction(tx, Some(self.verbosity), Some(self.rpc_url.clone()))
            .await
            .map_err(|e| CEPError::Other(e.to_string()))?;
        let tx_hash = TransactionHash::from(put.result.transaction_hash).to_string();
        let put_json = serde_json::to_value(&put.result)
            .map_err(|e| CEPError::Other(format!("serialize put result: {e}")))?;
        let result = CallResult::new(tx_hash, put_json);
        if !wait {
            return Ok(result);
        }
        let wait_params = TransactionParams {
            secret_key_pem: None,
            payment_amount: String::new(),
            chain_name: None,
            wait: true,
            wait_timeout_ms,
            put: true,
            initiator_addr: None,
        };
        self.maybe_wait(&wait_params, result).await
    }

    /// Install a session WASM (install or upgrade path).
    pub(crate) async fn install_wasm(
        &self,
        wasm: &[u8],
        tx: &TransactionParams,
        args_json: &str,
    ) -> Result<CallResult> {
        install::install_wasm(self, wasm, tx, args_json).await
    }

    /// Call a contract entrypoint by package hash (preferred) or entity hash.
    pub(crate) async fn call_entrypoint(
        &self,
        entry_point: &str,
        tx: &TransactionParams,
        args_json: &str,
    ) -> Result<CallResult> {
        call::call_entrypoint(self, entry_point, tx, args_json).await
    }

    /// Call a companion session WASM (CEP-78 session helpers).
    pub(crate) async fn call_session(
        &self,
        wasm: &[u8],
        tx: &TransactionParams,
        args_json: &str,
    ) -> Result<CallResult> {
        install::call_session(self, wasm, tx, args_json).await
    }

    /// Query a named key under the bound contract (`hash-{contract}`).
    pub(crate) async fn query_contract_key(&self, path: &[&str]) -> Result<Value> {
        query::query_contract_key(self, path).await
    }

    /// Query a dictionary item under a named dictionary on the bound contract.
    pub(crate) async fn query_dictionary(
        &self,
        dictionary_name: &str,
        item_key: &str,
    ) -> Result<Value> {
        query::query_dictionary(self, dictionary_name, item_key).await
    }

    /// Read a named key from an account (by public key hex or account-hash-…).
    pub(crate) async fn get_account_named_key(
        &self,
        account_identifier: &str,
        named_key: &str,
    ) -> Result<String> {
        query::get_account_named_key(self, account_identifier, named_key).await
    }

    /// Wait for a transaction hash on the configured SSE endpoint.
    ///
    /// Use after a put with wait disabled, or when you already have a transaction hash.
    /// Prefer [`TransactionParams::wait`] on CEP mutate/install for the normal put path.
    pub async fn wait_transaction(
        &self,
        transaction_hash: &str,
        timeout_ms: Option<u64>,
    ) -> Result<Value> {
        let sse = self
            .sse_url
            .as_deref()
            .ok_or_else(|| CEPError::WaitFailed("SSE URL is not configured".into()))?;
        let timeout = timeout_ms.or(Some(DEFAULT_WAIT_TIMEOUT_MS));
        let event = self
            .sdk
            .wait_transaction(sse, transaction_hash, timeout)
            .await
            .map_err(CEPError::WaitFailed)?;
        serde_json::to_value(event).map_err(|e| CEPError::Other(e.to_string()))
    }

    /// Build an SDK CES parser for one or more contract hashes (schemas from chain).
    pub(crate) async fn ces_parser_create(
        &self,
        contract_hashes: &[String],
        state_root_hash: Option<&str>,
    ) -> Result<CESParser> {
        self.sdk
            .CES_parser(contract_hashes, state_root_hash, Some(self.rpc_url.clone()))
            .await
            .map_err(CEPError::Other)
    }

    /// Parse CES events from an execution-result JSON value using schemas for `contract_hashes`.
    ///
    /// Use when you already have execution JSON and want CES rows outside auto-attach on
    /// [`CallResult::ces_events`].
    pub async fn parse_ces_execution(
        &self,
        contract_hashes: &[String],
        execution_result: &Value,
    ) -> Result<Vec<CESParseResult>> {
        let parser = self.ces_parser_create(contract_hashes, None).await?;
        let body = extract_execution_for_ces(execution_result);
        parser
            .parse_execution_result(&body)
            .map_err(CEPError::Other)
    }

    /// Fetch a transaction and parse CES events for `contract_hashes`.
    ///
    /// Prefer this over lower-level SDK CES parser APIs when decoding CEP events by hash.
    pub async fn parse_ces_transaction(
        &self,
        contract_hashes: &[String],
        transaction_hash: &str,
    ) -> Result<Vec<CESParseResult>> {
        let tx_hash = TransactionHash::new(transaction_hash)
            .map_err(|e| CEPError::InvalidHash(format!("transaction hash: {e}")))?;
        let mut json = None;
        if let Ok(get_tx) = self
            .sdk
            .get_transaction(
                tx_hash,
                Some(false),
                Some(self.verbosity),
                Some(self.rpc_url.clone()),
            )
            .await
        {
            if let Ok(v) = serde_json::to_value(&get_tx.result) {
                json = Some(v);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if json.as_ref().is_none_or(|j| !json_has_execution(j)) {
            if let Ok(v) = fetch_transaction_json_raw(&self.rpc_url, transaction_hash).await {
                json = Some(v);
            }
        }
        let json = json.ok_or_else(|| {
            CEPError::Other(format!(
                "could not load execution JSON for transaction {transaction_hash}"
            ))
        })?;
        self.parse_ces_execution(contract_hashes, &json).await
    }

    /// Bounded SSE collect for node event kinds (not CES contract event names).
    pub(crate) async fn sse_collect(
        &self,
        event_names: &[EventName],
        max_events: usize,
        timeout_ms: u64,
        start_from: Option<u64>,
    ) -> Result<Vec<RawEvent>> {
        let sse_url = self
            .sse_url
            .as_deref()
            .ok_or_else(|| CEPError::WaitFailed("SSE URL is not configured".into()))?;
        let client = self.sdk.SSE_client(sse_url);
        client
            .collect(event_names, max_events, timeout_ms, start_from)
            .await
            .map_err(CEPError::Other)
    }

    /// Collect `TransactionProcessed` SSE frames, then decode CES for the bound contract.
    ///
    /// Filters decoded rows to `ces_event_names` when non-empty (CES contract event names such as
    /// `Mint`). Requires [`Self::set_contract_hash`] and a configured SSE URL.
    pub async fn collect_ces_events(
        &self,
        ces_event_names: &[&str],
        max_transactions: usize,
        timeout_ms: u64,
    ) -> Result<Vec<CESEvent>> {
        let target = self.require_target()?;
        let hash_key = format!("hash-{}", target.contract_hash);
        let raws = self
            .sse_collect(
                &[EventName::TransactionProcessed],
                max_transactions,
                timeout_ms,
                None,
            )
            .await?;
        let parser = self.ces_parser_create(&[hash_key], None).await?;
        let mut out = Vec::new();
        for raw in raws {
            let parsed = match parser.parse_transaction_processed_json(&raw.data) {
                Ok(v) => v,
                Err(_) => continue,
            };
            for row in parsed {
                if row.error.is_some() {
                    continue;
                }
                if !ces_event_names.is_empty()
                    && !ces_event_names
                        .iter()
                        .any(|n| n.eq_ignore_ascii_case(&row.event.name))
                {
                    continue;
                }
                out.push(row.event);
            }
        }
        Ok(out)
    }

    pub(crate) fn builder_for_entrypoint(
        &self,
        entry_point: &str,
    ) -> Result<TransactionBuilderParams> {
        let target = self.require_target()?;
        if let Some(package_hex) = &target.package_hash {
            let package = PackageHash::new(package_hex)
                .map_err(|e| CEPError::InvalidHash(format!("package hash: {e}")))?;
            Ok(TransactionBuilderParams::new_package(
                package,
                entry_point,
                None,
            ))
        } else {
            let entity = AddressableEntityHash::new(&target.contract_hash)
                .map_err(|e| CEPError::InvalidHash(format!("contract hash: {e}")))?;
            Ok(TransactionBuilderParams::new_invocable_entity(
                entity,
                entry_point,
            ))
        }
    }

    pub(crate) fn bytes_from_slice(wasm: &[u8]) -> Bytes {
        Bytes::from(wasm.to_vec())
    }

    pub(crate) async fn maybe_wait(
        &self,
        tx: &TransactionParams,
        mut result: CallResult,
    ) -> Result<CallResult> {
        if !tx.put || !tx.wait {
            return Ok(result);
        }
        let _event = self
            .wait_transaction(&result.transaction_hash, tx.wait_timeout_ms)
            .await?;
        let event = _event;
        let tx_hash = TransactionHash::new(&result.transaction_hash)
            .map_err(|e| CEPError::InvalidHash(format!("transaction hash: {e}")))?;

        // After SSE reports processed: try to attach execution JSON when the node
        // can return it. Some large successful installs fail NCTL deserialize.
        let mut last_json = None;
        // Prefer execution embedded in the SSE TransactionProcessed payload.
        if json_has_execution(&event) {
            last_json = Some(event.clone());
        }
        for _ in 0..8 {
            if let Ok(get_tx) = self
                .sdk
                .get_transaction(
                    tx_hash.clone(),
                    Some(false),
                    Some(self.verbosity),
                    Some(self.rpc_url.clone()),
                )
                .await
            {
                if let Ok(json) = serde_json::to_value(&get_tx.result) {
                    last_json = Some(json);
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            if last_json.as_ref().is_none_or(|j| !json_has_execution(j)) {
                if let Ok(json) =
                    fetch_transaction_json_raw(&self.rpc_url, &result.transaction_hash).await
                {
                    last_json = Some(json);
                }
            }

            if let Some(json) = &last_json {
                if let Some(err) = extract_execution_error(json) {
                    return Err(CEPError::from_execution_message(err, self.cep_kind));
                }
                if json_has_execution(json) {
                    break;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }
        if let Some(json) = last_json {
            if let Some(err) = extract_execution_error(&json) {
                return Err(CEPError::from_execution_message(err, self.cep_kind));
            }
            result = result.with_execution(json);
            if let Some(target) = &self.target {
                let keys = [
                    format!("hash-{}", target.contract_hash),
                    format!("entity-contract-{}", target.contract_hash),
                ];
                let exec = result.execution_result.as_ref().unwrap();
                let parse_body = extract_execution_for_ces(exec);
                let exec_str = exec.to_string();
                for key in &keys {
                    let Ok(parser) = self
                        .ces_parser_create(std::slice::from_ref(key), None)
                        .await
                    else {
                        continue;
                    };
                    let rows = parser
                        .parse_transaction_processed_json(&exec_str)
                        .or_else(|_| parser.parse_execution_result(&parse_body));
                    if let Ok(rows) = rows {
                        if !rows.is_empty() {
                            result = result.with_ces_events(rows);
                            break;
                        }
                        if result.ces_events.is_none() {
                            result = result.with_ces_events(rows);
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    pub(crate) fn runtime_v2(&self) -> Option<bool> {
        self.runtime_v2
    }
}

/// Prefer nested execution_result / execution_info bodies for CES parse.
fn extract_execution_for_ces(json: &Value) -> Value {
    if let Some(v) = json.get("execution_result").cloned() {
        return v;
    }
    if let Some(v) = json.pointer("/execution_info/execution_result").cloned() {
        return v;
    }
    if let Some(v) = json.pointer("/result/execution_result").cloned() {
        return v;
    }
    json.clone()
}

fn json_has_execution(json: &Value) -> bool {
    json.get("execution_info").is_some()
        || json.pointer("/execution_result").is_some()
        || json.to_string().contains("\"execution_result\"")
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_transaction_json_raw(
    rpc_url: &str,
    transaction_hash: &str,
) -> std::result::Result<Value, String> {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "info_get_transaction",
        "params": {
            "transaction_hash": { "Version1": transaction_hash },
            "finalized_approvals": false
        }
    });
    let client = reqwest::Client::new();
    let resp = client
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("raw info_get_transaction: {e}"))?;
    let value: Value = resp
        .json()
        .await
        .map_err(|e| format!("raw info_get_transaction body: {e}"))?;
    if let Some(err) = value.get("error") {
        return Err(format!("raw info_get_transaction rpc error: {err}"));
    }
    value
        .get("result")
        .cloned()
        .ok_or_else(|| "raw info_get_transaction missing result".to_string())
}

fn extract_execution_error(json: &Value) -> Option<String> {
    let candidates = [
        json.pointer("/execution_info/execution_result/Version2/error_message"),
        json.pointer("/execution_info/execution_result/Version1/error_message"),
        json.pointer("/execution_info/execution_result/Failure/error_message"),
        json.pointer("/execution_result/Version2/error_message"),
        json.pointer("/execution_result/Version1/error_message"),
        json.pointer("/execution_result/Failure/error_message"),
        json.pointer("/execution_info/execution_result/error_message"),
        json.pointer("/execution_info/execution_result/failure/error_message"),
        json.pointer("/execution_result/failure/error_message"),
    ];
    for c in candidates.into_iter().flatten() {
        if let Some(s) = c.as_str() {
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    let text = json.to_string();
    if let Some(idx) = text.find("User error:") {
        let slice: String = text[idx..].chars().take(80).collect();
        return Some(slice);
    }
    // Ignore `"Failure":null` / `"Failure": null` success shapes from SSE / get_tx JSON.
    let has_real_failure = text.contains("\"Failure\":{")
        || text.contains("\"Failure\" : {")
        || text.contains("\"failure\":{");
    if has_real_failure {
        if let Some(idx) = text.find("error_message") {
            let slice: String = text[idx..].chars().take(120).collect();
            return Some(format!("execution Failure: {slice}"));
        }
        return Some("execution Failure (see transaction result)".into());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_new_normalizes_urls() {
        let core = CEPClient::new("http://127.0.0.1:11101", None, None, None).unwrap();
        assert_eq!(core.rpc_url(), "http://127.0.0.1:11101/rpc");
        assert!(core.sse_url().is_none());
    }

    #[test]
    fn core_rejects_empty_rpc() {
        assert!(CEPClient::new("", None, None, None).is_err());
    }

    #[test]
    fn extracts_version2_error_message() {
        let json = serde_json::json!({
            "execution_info": {
                "execution_result": {
                    "Version2": {
                        "error_message": "ApiError::EarlyEndOfStream [17]"
                    }
                }
            }
        });
        assert_eq!(
            extract_execution_error(&json).as_deref(),
            Some("ApiError::EarlyEndOfStream [17]")
        );
    }

    #[test]
    fn ignores_null_version2_error_message() {
        let json = serde_json::json!({
            "execution_info": {
                "execution_result": {
                    "Version2": {
                        "error_message": null
                    }
                }
            }
        });
        assert!(extract_execution_error(&json).is_none());
    }

    fn sample_pem_and_pk() -> (String, String) {
        let sk = casper_types::SecretKey::generate_ed25519().expect("generate key");
        let pem = sk.to_pem().expect("to_pem");
        let pk = casper_rust_wasm_sdk::helpers::public_key_from_secret_key(&pem).expect("pk");
        (pem, pk)
    }

    #[test]
    fn build_tx_params_put_requires_secret() {
        let core = CEPClient::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let mut tx = TransactionParams::new("pem", "1000000000");
        tx.secret_key_pem = None;
        let err = core.build_tx_params(&tx, "[]").unwrap_err();
        assert!(err.to_string().contains("secret_key_pem"));
    }

    #[test]
    fn build_tx_params_unsigned_make_needs_initiator() {
        let core = CEPClient::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let tx = TransactionParams::for_make("1000000000");
        assert!(core.build_tx_params(&tx, "[]").is_err());
        let tx = tx.with_initiator_addr(
            "010101010101010101010101010101010101010101010101010101010101010101",
        );
        // validation passes; SDK make may still reject a fake initiator later
        assert!(core.build_tx_params(&tx, "[]").is_ok());
    }

    #[tokio::test]
    async fn make_only_install_returns_transaction_json_without_put() {
        let (pem, _pk) = sample_pem_and_pk();
        let core = CEPClient::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let tx = TransactionParams::new(&pem, "1000000000").make_only();
        // Minimal empty Wasm module header (make does not execute it).
        let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let result = core
            .install_wasm(&wasm, &tx, "[]")
            .await
            .expect("make-only install");
        assert!(result.put_result.is_null());
        assert!(!result.transaction_hash.is_empty());
        assert!(result.execution_result.is_none());
        assert!(result.ces_events.as_ref().is_none_or(|v| v.is_empty()));
        let body = result.transaction.expect("transaction json");
        assert!(body.is_object(), "transaction must be a JSON object");
        let hash_in_body = body
            .get("hash")
            .or_else(|| body.pointer("/TransactionV1/hash"))
            .or_else(|| body.pointer("/transaction/hash"));
        assert!(
            hash_in_body.is_some() || body.to_string().contains(&result.transaction_hash),
            "transaction JSON should carry the hash ({})",
            result.transaction_hash
        );
    }

    #[tokio::test]
    async fn make_only_call_with_initiator_addr() {
        let (_pem, pk) = sample_pem_and_pk();
        let mut core = CEPClient::new("http://127.0.0.1:11101", None, None, None).unwrap();
        core.set_contract_hash(
            "cfa781f5eb69c3eee952c2944ce9670a049f88c5e46b83fb5881ebe13fb98e6d",
            None::<&str>,
        )
        .unwrap();
        let tx = TransactionParams::for_make("1000000000").with_initiator_addr(&pk);
        let result = core
            .call_entrypoint("mint", &tx, "[]")
            .await
            .expect("make-only call");
        assert!(result.put_result.is_null());
        assert!(result.execution_result.is_none());
        assert!(!result.transaction_hash.is_empty());
        let body = result.transaction.expect("transaction json");
        assert!(body.is_object());
    }

    #[tokio::test]
    async fn put_transaction_rejects_invalid_json() {
        let core = CEPClient::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let bad = serde_json::json!({"not": "a_transaction"});
        let err = core
            .put_transaction(&bad, false, None)
            .await
            .expect_err("invalid transaction JSON must fail");
        assert!(
            err.to_string().contains("parse transaction JSON")
                || err.to_string().to_lowercase().contains("parse")
                || err.to_string().to_lowercase().contains("deserialize")
                || err.to_string().to_lowercase().contains("invalid"),
            "unexpected error: {err}"
        );
    }

    #[tokio::test]
    async fn make_only_rejects_put_without_secret() {
        let core = CEPClient::new("http://127.0.0.1:11101", None, None, None).unwrap();
        let tx = TransactionParams::for_make("1000000000");
        // for_make already put=false; force invalid put without secret via validate path
        let mut bad = TransactionParams::new("x", "1");
        bad.secret_key_pem = None;
        assert!(bad.validate().is_err());
        let unsigned = TransactionParams::for_make("1");
        assert!(unsigned.validate().is_err());
        let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let err = core.install_wasm(&wasm, &tx, "[]").await.unwrap_err();
        assert!(
            err.to_string().contains("initiator_addr")
                || err.to_string().contains("secret_key")
                || err.to_string().contains("make-only"),
            "unexpected error: {err}"
        );
    }
}
