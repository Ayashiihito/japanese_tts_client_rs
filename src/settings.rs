use config::{Config, ConfigError, File};
use serde::Deserialize;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("config can be loaded");
    }
    
    #[derive(Debug, Deserialize, Clone)]
    pub struct Settings {
        pub server_port: u16,
        pub server_address: String,
        pub storage_dir: String,
        pub max_cache_size: u64,
        pub cache_expiration: u64,
    }
    
    const CONFIG_FILE_PATH: &str = "./config/config.toml";
    
    impl Settings {
        pub fn new() -> Result<Self, ConfigError> {
            let mut settings = Config::new();
            settings.merge(File::with_name(CONFIG_FILE_PATH))?;
            settings.try_into()
        }
    }
