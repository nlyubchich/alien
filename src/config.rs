use std::fs::File;
use std::io::prelude::*;
use toml;

pub struct BotConfig {
    pub telegram_owner_id: i64,
    pub telegram_bot_token: String,
    pub transmission_rpc_url: String,
}

#[derive(Deserialize)]
struct TomlConfig {
    pub telegram_owner_id: i64,
    pub telegram_bot_token: String,
    pub transmission_rpc_url: String,
}

pub fn get_config() -> BotConfig {
    let mut file = File::open("config.toml").expect("There should be config.toml");
    let mut raw_content = String::new();
    file.read_to_string(&mut raw_content).unwrap();

    let parsed_config: TomlConfig = toml::from_str(raw_content.as_str()).unwrap();
    BotConfig {
        telegram_owner_id: parsed_config.telegram_owner_id,
        telegram_bot_token: parsed_config.telegram_bot_token,
        transmission_rpc_url: parsed_config.transmission_rpc_url.clone(),
    }
}
