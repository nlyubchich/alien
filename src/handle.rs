use telegram_bot as telegram;
use integrations::transmission::{do_add_torrent_action,get_torrent_list_action};
use config::BotConfig;

pub enum Actions {
    AddTorrent,
    ListTorrents,
}

pub fn process_update(config: &BotConfig, message: &telegram::Message) -> String {
    if message.from.id != config.telegram_owner_id {
        return String::from("Sorry, you are not allowed to use this bot");
    }

    if let telegram::MessageType::Text(text) = message.clone().msg {
        let message_type = check_message(&text);
        return match message_type {
            Some(Actions::AddTorrent) => do_add_torrent_action(&config, &text),
            Some(Actions::ListTorrents) => get_torrent_list_action(&config, &text),
            None => String::from("Sorry, I don't understand"),
        };
    }
    String::from("Sorry, I don't understand")
}

pub fn check_message(msg: &str) -> Option<Actions> {
    match msg {
        msg if msg.starts_with("magnet:") => Some(Actions::AddTorrent),
        msg if msg == "list torrents" => Some(Actions::ListTorrents),
        _ => None,
    }
}
