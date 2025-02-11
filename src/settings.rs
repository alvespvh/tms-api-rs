use std::sync::LazyLock;

use config::Config;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub application_name: String,
    pub database_url: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    pub log_level: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Settings {
        dotenv().ok();
        let settings = Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap();

        settings.try_deserialize::<Settings>().unwrap()
    }
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(Settings::new);
