extern crate indy;

use std::{env, thread};
use std::future::Future;
use std::time::Duration;

use futures::executor::block_on;
use indy::commands::{build_executors, CommandExecutor};
use indy::commands::anoncreds::AnoncredsCommandExecutor;
use indy::commands::pool::PoolCommandExecutor;
use indy::domain::anoncreds::schema::SchemaId;
use indy::domain::cache::GetCacheOptions;
use indy::domain::crypto::did::DidValue;
use indy::domain::pool::{PoolConfig, PoolOpenConfig};
use indy::utils::logger::LibindyDefaultLogger;
use indy_api_types::{PoolHandle, WalletHandle};
use indy_api_types::domain::wallet::{Config, Credentials, KeyDerivationMethod};

pub static DEFAULT_WALLET_KEY: &str = "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY";

pub fn run_async() {
    println!("run_async started");
    let pattern = env::var("RUST_LOG").ok();
    LibindyDefaultLogger::init(pattern);
    let did = DidValue::new("NEy7K3xVTdopbHhfQCtb4M", None);
    let (
        anoncredsCommandExecutor,
        cryptoCommandExecutor,
        ledgerCommandExecutor,
        poolCommandExecutor,
        didCommandExecutor,
        walletCommandExecutor,
        pairwiseCommandExecutor,
        blobStorageCommandExecutor,
        nonSecretsCommandExecutor,
        paymentsCommandExecutor,
        cacheCommandExecutor,
        metricsCommandExecutor) = build_executors();

    println!("run_async going to open pool");
    let pool_handle = block_on(poolCommandExecutor.open(String::from("pool1"), None)).unwrap();
    println!("run_async pool opened, handle={}", pool_handle);

    let wallet_cfg = Config {
        id: "foobar".to_string(),
        storage_type: None,
        storage_config: None,
    };
    let wallet_credentials = Credentials {
        key: DEFAULT_WALLET_KEY.to_string(),
        rekey: None,
        storage_credentials: None,
        key_derivation_method: KeyDerivationMethod::RAW,
        rekey_derivation_method: KeyDerivationMethod::RAW,
    };
    block_on(walletCommandExecutor._create(&wallet_cfg, &wallet_credentials)); // might error if wallet already exists
    println!("run_async wallet created");

    let wallet_handle = block_on(walletCommandExecutor._open(&wallet_cfg, &wallet_credentials)).unwrap();
    println!("run_async wallet opened, = {:?}", wallet_handle);

    let schema_id = &SchemaId(String::from("StetyxAvHZVH6HLDQ4ym4G:2:MedicalLicense+medical_license:3.0.0"));
    let opts = GetCacheOptions {
        no_cache: None,
        no_update: None,
        no_store: None,
        min_fresh: None,
    };
    let res = block_on(cacheCommandExecutor.get_schema(pool_handle, wallet_handle, &did, schema_id, opts));
    println!("res = {:?}", res);

    block_on(poolCommandExecutor.close(pool_handle)).unwrap();
}
