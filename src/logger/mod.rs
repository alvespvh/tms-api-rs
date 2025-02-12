use std::str::FromStr;

use convert_case::Casing;

use crate::prelude::SETTINGS;

pub mod telegram;

pub fn init() {
    let mut level = SETTINGS.log_level.clone().unwrap_or("INFO".to_string());
    level = level.to_uppercase();
    let level = log::LevelFilter::from_str(level.as_str()).unwrap_or(log::LevelFilter::Info);
    env_logger::builder()
        .filter_module(
            &env!("CARGO_PKG_NAME").to_case(convert_case::Case::Snake),
            level,
        )
        .init();
}

pub fn _info(text: &String) {
    log::info!("{}", text);
}

pub async fn error(text: &str) {
    log::error!("{}", text);
    let telegram_text = &text[..1000.min(text.len())];
    telegram::notify(telegram_text).await;
}
