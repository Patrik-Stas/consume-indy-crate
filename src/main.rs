extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
#[macro_use]
extern crate failure;
// when using Rust FFI wrapper
// #[macro_use]
// pub extern crate indyrs as indy;
#[macro_use]
extern crate lazy_static;

// when using libindy = { git = "https://gitlab.com/PatrikStas/vdr-tools", rev = "1473cea" }
#[macro_use]
extern crate indy;

#[macro_use]
extern crate log;
extern crate pretty_env_logger as env_logger;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

use serde_json::Value;

use crate::libindy_async::run_async_vdrtools_fork;

#[cfg(feature = "ffi")]
use crate::libindy_ffi::run_libindy_ffi;
#[cfg(feature = "pg")]
use crate::pgwallet::wallet_plugin_loader::{init_wallet_plugin, PluginInitConfig};
use indy::utils::logger::LibindyDefaultLogger;


mod libindy_async;
mod utils;

#[cfg(feature = "pg")]
mod pgwallet;
#[cfg(feature = "ffi")]
mod libindy_ffi;

fn main() {
    let pattern = env::var("RUST_LOG").ok();
    LibindyDefaultLogger::init(pattern);
    info!("Indy logger initialized");
    // env_logger::try_init()
    //     .expect("Can't init env logger");

    // indy_set_default_logger(pattern.as_ref().map(String::as_str));
    // utils::logger::set_default_logger(pattern.as_ref().map(String::as_str)).unwrap();

    let storage_config = r#"
          {
            "read_host": "localhost",
            "write_host": "localhost",
            "port": 3306,
            "db_name": "indycratetest",
            "default_connection_limit": 50
          }"#;
    let storage_credentials = r#"
        {
            "user": "root",
            "pass": "mysecretpassword"
        }"#;
    // pg wallet plugin init ...
    // let init_config: PluginInitConfig = PluginInitConfig {
    //     storage_type: String::from("postgres_storage"),
    //     plugin_library_path: None,
    //     plugin_init_function: None,
    //     config: storage_config.into(),
    //     credentials: storage_credentials.into()
    // };
    // init_wallet_plugin(&init_config).unwrap();
    // info!("Init pgsql storage: Finished.");

    // run_libindy_ffi(storage_credentials, storage_config)

    let storage_credentials = Some(serde_json::from_str(storage_credentials).unwrap());
    let storage_config = Some(serde_json::from_str(storage_config).unwrap());
    run_async_vdrtools_fork(storage_credentials, storage_config, Some("mysql".into()));
}