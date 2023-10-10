use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/config.toml";

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("config can be loaded");
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server_port: u16,
    pub server_address: String,
    pub storage_dir: String,
    pub max_cache_size_mb: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()?;

        settings.try_deserialize()
    }
}
