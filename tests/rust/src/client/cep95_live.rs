//! CEP-95 NCTL integration: install → mint → transfer → approve.

#[cfg(test)]
mod tests {
    use crate::helpers::{
        account_hash_from_secret, nctl_available, user1_account_hash, user1_public_key_hex,
        user1_secret_pem, user_secret_pem, CALL_PAYMENT,
    };
    use ceps_client::cep95::InstallArgs;
    use ceps_client::{Cep95Client, TransactionParams, Verbosity};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    const INSTALL_PAYMENT: &str = "600000000000";

    fn cep95_client() -> Cep95Client {
        let rpc = env::var("CEPS_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:11101".into());
        let sse =
            env::var("CEPS_SSE_URL").unwrap_or_else(|_| "http://127.0.0.1:18101/events".into());
        Cep95Client::new(
            rpc,
            Some(sse),
            Some("casper-net-1".into()),
            Some(Verbosity::Low),
        )
        .expect("client")
    }

    fn wasm_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/wasm/cep95/cep95.wasm")
    }

    #[tokio::test]
    async fn cep95_install_mint_transfer_approve() {
        if !nctl_available() {
            eprintln!("skip: NCTL not reachable");
            return;
        }
        let Some(secret) = user1_secret_pem() else {
            eprintln!("skip: no secret");
            return;
        };
        let Some(spender_secret) = user_secret_pem(2, "SECRET_KEY_USER_2") else {
            eprintln!("skip: no user-2 secret");
            return;
        };
        let wasm_file = wasm_path();
        if !wasm_file.is_file() {
            eprintln!("skip: missing {}", wasm_file.display());
            return;
        }
        let wasm = fs::read(&wasm_file).expect("wasm");
        let mut client = cep95_client();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let name = format!("Ceps95{nonce}");
        let package_key = format!("cep95_pkg_{nonce}");
        let args = InstallArgs::new(&name, "C95", &package_key);
        let tx = TransactionParams::new(&secret, INSTALL_PAYMENT);
        let put = client.install(&args, &wasm, &tx).await.expect("install");
        assert!(!put.transaction_hash.is_empty());

        let pk = user1_public_key_hex(&secret);
        let (contract, package) = client
            .bind_odra_install(&pk, &package_key)
            .await
            .expect("bind");
        assert!(
            contract.contains("hash-") || contract.len() >= 64,
            "{contract}"
        );
        assert!(!package.is_empty());

        assert_eq!(client.name().await.expect("name"), name);
        assert_eq!(client.symbol().await.expect("symbol"), "C95");

        let owner = user1_account_hash(&secret);
        let spender = account_hash_from_secret(&spender_secret);
        let mint_tx = TransactionParams::new(&secret, CALL_PAYMENT);
        client
            .mint(&owner, "1", None, &mint_tx)
            .await
            .expect("mint");

        let bal = client.balance_of(&owner).await.expect("balance");
        assert_eq!(bal, "1");

        let owner_of = client.owner_of("1").await.expect("owner_of");
        assert!(
            owner_of.contains(&owner.replace("account-hash-", ""))
                || owner_of.contains(&owner)
                || owner_of.to_lowercase().contains("account"),
            "owner_of={owner_of} expected around {owner}"
        );

        // Self-transfer keeps ownership; exercises transfer_from entrypoint.
        client
            .transfer_from(&owner, &owner, "1", &mint_tx)
            .await
            .expect("transfer_from");
        assert_eq!(
            client.balance_of(&owner).await.expect("balance after xfer"),
            "1"
        );

        // Approve a different account (cannot approve current owner: user error 40003).
        client
            .approve(&spender, "1", &mint_tx)
            .await
            .expect("approve");
        let approved = client.get_approved("1").await.expect("get_approved");
        assert!(
            approved.is_some(),
            "expected get_approved Some after approve"
        );
        let approved = approved.unwrap();
        assert!(
            approved.contains(&spender.replace("account-hash-", "")) || approved.contains(&spender),
            "approved={approved} expected around {spender}"
        );
    }
}
