use telegram_bot as telegram;
use config::BotConfig;
use integrations::{IntegrationsEnum,IntegrationInstances};


pub trait BotIntegration<Integration> {
    fn new(config: &BotConfig) -> Integration;
    fn dispatch(&self, message: String) -> String;
}

pub fn process_update(
        config: &BotConfig,
        integrations: &IntegrationInstances,
        message: &telegram::Message
) -> String {
    if message.from.id != config.telegram_owner_id {
        return String::from("Sorry, you are not allowed to use this bot");
    }

    if let telegram::MessageType::Text(text) = message.clone().msg {
        return match check_message(&text) {
            Some(IntegrationsEnum::Transmission) => integrations.transmission.dispatch(text),
            None => String::from("Sorry, I cannot match to any integrations :("),
        };
    }
    String::from("Sorry, I cannot match to any integrations :(")
}

pub fn check_message(msg: &str) -> Option<IntegrationsEnum> {
    match msg {
        msg if msg.starts_with("magnet:") => Some(IntegrationsEnum::Transmission),
        msg if msg == "list torrents" => Some(IntegrationsEnum::Transmission),
        _ => None,
    }
}
