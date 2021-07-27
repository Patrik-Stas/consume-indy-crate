extern crate indy;

use std::{env, thread};
use std::future::Future;
use std::time::Duration;

use futures::executor::block_on;
use indy_api_types::domain::wallet::{Config, Credentials, KeyDerivationMethod};

use self::indy::domain::anoncreds::schema::SchemaId;
use self::indy::domain::cache::GetCacheOptions;
use self::indy::domain::crypto::did::DidValue;
use self::indy::Locator;

pub static DEFAULT_WALLET_KEY: &str = "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY";

pub fn run_async() {
    println!("run_async started");
    let did = DidValue::new("NEy7K3xVTdopbHhfQCtb4M", None);
    let l = Locator::instance();
    l.ledger_controller.build_get_nym_request(Some(did.clone()), did.clone());
    println!("run_async going to open pool");
    let pool_handle = block_on(l.pool_controller.open(String::from("pool1"), None)).unwrap();
    println!("run_async pool opened, handle={}", pool_handle);

    let wallet_cfg = Config {
        id: "foobar".to_string(),
        storage_type: None,
        storage_config: None,
        cache: None,
    };
    let wallet_credentials = Credentials {
        key: DEFAULT_WALLET_KEY.to_string(),
        rekey: None,
        storage_credentials: None,
        key_derivation_method: KeyDerivationMethod::RAW,
        rekey_derivation_method: KeyDerivationMethod::RAW,
    };
    block_on(l.wallet_controller.create(wallet_cfg.clone(), wallet_credentials.clone())); // might error if wallet already exists
    println!("run_async wallet created");

    let wallet_handle = block_on(l.wallet_controller.open(wallet_cfg.clone(), wallet_credentials.clone())).unwrap();
    println!("run_async wallet opened, = {:?}", wallet_handle);

    let schema_id = SchemaId(String::from("StetyxAvHZVH6HLDQ4ym4G:2:MedicalLicense+medical_license:3.0.0"));
    let opts = GetCacheOptions {
        no_cache: None,
        no_update: None,
        no_store: None,
        min_fresh: None,
    };
    let res = block_on(l.cache_controller.get_schema(pool_handle, wallet_handle, did.clone(), schema_id, opts));
    println!("res = {:?}", res);

    block_on(l.pool_controller.close(pool_handle)).unwrap();
}
