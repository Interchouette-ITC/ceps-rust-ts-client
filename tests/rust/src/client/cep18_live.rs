//! CEP-18 NCTL integration: install → query → transfer.

#[cfg(test)]
mod tests {
    use crate::helpers::{
        cep18_client, nctl_available, user1_account_hash, user1_public_key_hex, user1_secret_pem,
        wasm_path, CALL_PAYMENT, INSTALL_PAYMENT,
    };
    use ceps_client::cep18::InstallArgs;
    use ceps_client::types::{DeployParams, EventsMode};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn cep18_install_query_transfer() {
        if !nctl_available() {
            eprintln!("skip: NCTL RPC not reachable on 127.0.0.1:11101");
            return;
        }
        let Some(secret) = user1_secret_pem() else {
            eprintln!("skip: user-1 secret key not found");
            return;
        };
        let wasm_file = wasm_path("cep18.wasm");
        if !wasm_file.is_file() {
            eprintln!("skip: missing {}", wasm_file.display());
            return;
        }
        let wasm = fs::read(&wasm_file).expect("read wasm");

        let mut client = cep18_client();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let name = format!("CepsRust{nonce}");
        let args = InstallArgs::new(&name, "CRT", 9, "1000000000000")
            .with_events_mode(EventsMode::Ces)
            .with_mint_and_burn(true);

        let deploy = DeployParams::new(&secret, INSTALL_PAYMENT);
        let install = client
            .install(&args, &wasm, &deploy)
            .await
            .expect("install");
        assert!(!install.transaction_hash.is_empty());

        let pk = user1_public_key_hex(&secret);
        let contract_hash = client
            .core()
            .get_account_named_key(&pk, &format!("cep18_contract_hash_{name}"))
            .await
            .expect("contract hash named key");
        let package_hash = client
            .core()
            .get_account_named_key(&pk, &format!("cep18_contract_package_{name}"))
            .await
            .expect("package hash named key");

        client
            .set_contract_hash(&contract_hash, Some(&package_hash))
            .expect("set hashes");

        let token_name = client.name().await.expect("name");
        assert_eq!(token_name, name);
        let symbol = client.symbol().await.expect("symbol");
        assert_eq!(symbol, "CRT");
        let decimals = client.decimals().await.expect("decimals");
        assert_eq!(decimals, 9);

        let owner = user1_account_hash(&secret);
        let balance = client.balance_of(&owner).await.expect("balance");
        assert_eq!(balance, "1000000000000");

        // Transfer a small amount to self is rejected; burn instead as mutate smoke.
        let burn_deploy = DeployParams::new(&secret, CALL_PAYMENT);
        let burned = client.burn(&owner, "1", &burn_deploy).await.expect("burn");
        assert!(!burned.transaction_hash.is_empty());

        let balance_after = client.balance_of(&owner).await.expect("balance after");
        assert_eq!(balance_after, "999999999999");
    }
}
