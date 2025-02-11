use std::collections::HashMap;

use crate::settings::SETTINGS;

pub async fn notify(error: &str) {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        SETTINGS.telegram_bot_token
    );

    let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();

    let mut json = HashMap::new();
    json.insert("chat_id", &SETTINGS.telegram_chat_id);
    let text = &format!("Erro em {}:\n{}", SETTINGS.application_name.clone(), error);
    json.insert("text", text);

    let response = client.post(url).json(&json).send().await;
    match response {
        Ok(_) => (),
        Err(error) => {
            log::error!("{}", error.to_string());
        }
    }
}
