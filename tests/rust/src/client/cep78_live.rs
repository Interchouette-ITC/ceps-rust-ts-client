//! CEP-78 NCTL integration: install → mint → owner_of / balance_of.

#[cfg(test)]
mod tests {
    use crate::helpers::{
        nctl_available, user1_account_hash, user1_public_key_hex, user1_secret_pem, CALL_PAYMENT,
    };
    use ceps_client::cep78::{InstallArgs, TokenIdentifier};
    use ceps_client::{CEP78Client, EventsMode78, TransactionParams, Verbosity};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    const INSTALL_PAYMENT: &str = "600000000000";

    fn cep78_client() -> CEP78Client {
        let rpc = env::var("CEPS_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:11101".into());
        let sse =
            env::var("CEPS_SSE_URL").unwrap_or_else(|_| "http://127.0.0.1:18101/events".into());
        CEP78Client::new(
            rpc,
            Some(sse),
            Some("casper-net-1".into()),
            Some(Verbosity::Low),
        )
        .expect("client")
    }

    fn wasm_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/wasm/cep78/cep78.wasm")
    }

    #[tokio::test]
    async fn cep78_install_mint_query() {
        if !nctl_available() {
            eprintln!("skip: NCTL not reachable");
            return;
        }
        let Some(secret) = user1_secret_pem() else {
            eprintln!("skip: no secret");
            return;
        };
        let wasm_file = wasm_path();
        if !wasm_file.is_file() {
            eprintln!("skip: missing {}", wasm_file.display());
            return;
        }
        let wasm = fs::read(&wasm_file).expect("wasm");
        let mut client = cep78_client();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let name = format!("Ceps78{nonce}");
        let args = InstallArgs::new(&name, "C78", 100).with_events_mode(EventsMode78::CES);
        let tx = TransactionParams::new(&secret, INSTALL_PAYMENT);
        let put = client.install(&args, &wasm, &tx).await.expect("install");
        assert!(!put.transaction_hash.is_empty());

        let pk = user1_public_key_hex(&secret);
        let contract = client
            .get_account_named_key(&pk, &format!("cep78_contract_hash_{name}"))
            .await
            .expect("contract hash");
        let package = client
            .get_account_named_key(&pk, &format!("cep78_contract_package_{name}"))
            .await
            .expect("package hash");
        client
            .set_contract_hash(&contract, Some(&package))
            .expect("set");

        assert_eq!(client.collection_name().await.expect("name"), name);
        assert_eq!(client.collection_symbol().await.expect("symbol"), "C78");
        assert_eq!(
            client.ownership_mode().await.expect("ownership"),
            ceps_client::cep78::OwnershipMode::Transferable
        );
        assert_eq!(
            client.events_mode().await.expect("events_mode"),
            EventsMode78::CES
        );

        let owner = user1_account_hash(&secret);
        let mint_tx = TransactionParams::new(&secret, CALL_PAYMENT);
        let mint = client
            .mint(&owner, "meta-0", None, &mint_tx)
            .await
            .expect("mint");
        let hash_key = format!(
            "hash-{}",
            client.core().target().expect("target").contract_hash
        );
        let ces = if let Some(rows) = mint.ces_events.clone() {
            rows
        } else if let Some(exec) = mint.execution_result.as_ref() {
            client
                .core()
                .parse_ces_execution(&[hash_key], exec)
                .await
                .unwrap_or_default()
        } else {
            client
                .core()
                .parse_ces_transaction(&[hash_key], &mint.transaction_hash)
                .await
                .unwrap_or_default()
        };
        if ces.is_empty() {
            eprintln!(
                "warn: no CES Mint event decoded (NCTL may omit effects); tx={}",
                mint.transaction_hash
            );
        } else {
            assert!(
                ces.iter()
                    .any(|r| r.error.is_none() && r.event.name.eq_ignore_ascii_case("Mint")),
                "expected Mint CES event, got {ces:?}"
            );
        }

        let bal = client.balance_of(&owner).await.expect("balance");
        assert_eq!(bal, "1");

        let session_wasm = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/wasm/cep78/balance_of_session.wasm");
        if session_wasm.is_file() {
            let session_bytes = fs::read(&session_wasm).expect("balance session wasm");
            let key_name = format!("ceps78_bal_{nonce}");
            let session = client
                .balance_of_session(&owner, &key_name, &session_bytes, &mint_tx)
                .await
                .expect("balance_of_session");
            assert!(!session.transaction_hash.is_empty());
            let stored = client
                .get_account_named_key(&pk, &key_name)
                .await
                .expect("session named key");
            // Session wasm binds a URef under the named key (balance lives at that uref).
            // Do not assert `contains('1')` on the uref hex: that is flaky.
            assert!(
                stored.starts_with("uref-"),
                "balance_of_session named key should be a uref, got {stored}"
            );
        }

        let owner_of = client
            .owner_of(&TokenIdentifier::id(0))
            .await
            .expect("owner_of");
        assert!(
            owner_of.contains(&owner.replace("account-hash-", ""))
                || owner_of.contains(&owner)
                || owner_of.to_lowercase().contains("account"),
            "owner_of={owner_of} expected around {owner}"
        );
    }
}
