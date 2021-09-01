use std::{env, thread};
use std::future::Future;
use std::time::Duration;

use futures::executor::block_on;
use indy_api_types::domain::wallet::{Config, Credentials, KeyDerivationMethod};

use serde_json::Value;
use indy::domain::crypto::did::DidValue;
use indy::Locator;
use indy::domain::anoncreds::schema::SchemaId;
use indy::domain::cache::GetCacheOptions;

pub static DEFAULT_WALLET_KEY: &str = "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY";

pub async fn run_async_vdrtools_fork(storage_credentials: Option<Value>, storage_config: Option<Value>, storage_type: Option<String>) {
    info!("Getting locator instance");
    let l = Locator::instance();
    let did = DidValue::new("NEy7K3xVTdopbHhfQCtb4M", None);

    l.ledger_controller.build_get_nym_request(Some(did.clone()), did.clone());
    info!("run_async going to open pool");
    let pool_handle = l.pool_controller.open(String::from("pool1"), None).await.unwrap();
    info!("run_async pool opened, handle={}", pool_handle);

    let wallet_cfg = Config {
        id: "foobar".to_string(),
        storage_type,
        storage_config: storage_config.clone(),
        cache: None,
    };
    let wallet_credentials = Credentials {
        key: DEFAULT_WALLET_KEY.to_string(),
        rekey: None,
        storage_credentials,
        key_derivation_method: KeyDerivationMethod::RAW,
        rekey_derivation_method: KeyDerivationMethod::RAW,
    };
    l.wallet_controller.create(wallet_cfg.clone(), wallet_credentials.clone()).await.unwrap(); // might error if wallet already exists
    info!("run_async wallet created");

    let wallet_handle = l.wallet_controller.open(wallet_cfg.clone(), wallet_credentials.clone()).await.unwrap();
    info!("run_async wallet opened, = {:?}", wallet_handle);

    let schema_id = SchemaId(String::from("StetyxAvHZVH6HLDQ4ym4G:2:MedicalLicense+medical_license:3.0.0"));
    let opts = GetCacheOptions {
        no_cache: None,
        no_update: None,
        no_store: None,
        min_fresh: None,
    };
    let res = l.cache_controller.get_schema(pool_handle, wallet_handle, did.clone(), schema_id, opts).await;
    info!("res = {:?}", res);

    l.pool_controller.close(pool_handle).await.unwrap();
}
