use envconfig::Envconfig;

lazy_static! {
    static ref APP_ENV_CONFIG: AppEnvConfig = AppEnvConfig::init().unwrap();
}

pub fn get_app_env_config() -> &'static AppEnvConfig {
    return &APP_ENV_CONFIG
}

#[derive(Envconfig, Debug)]
pub struct AppEnvConfig {
    #[envconfig(from = "NEW_AGENT_KDF", default = "RAW")]
    pub new_agent_kdf: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn should_construct_app_env_config() {
        env::remove_var("NEW_AGENT_KDF");
        let app_config = AppEnvConfig::init().unwrap();
        assert_eq!(app_config.new_agent_kdf, "RAW", "Default new_agent_kdf should be Raw");

        env::set_var("NEW_AGENT_KDF", "RAW");
        let app_config = AppEnvConfig::init().unwrap();
        assert_eq!(app_config.new_agent_kdf, "RAW", "Expected new_agent_kdf to be Raw.");
    }
}