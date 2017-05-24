extern crate telegram_bot;
#[macro_use(Deserialize)]
extern crate serde_derive;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_json;
extern crate toml;

mod handle;
mod integrations;
mod config;

use config::{get_config};
use telegram_bot::{Api,ListeningMethod,MessageType,ListeningAction};
use integrations::transmission::{do_add_torrent_action};
use handle::{check_message,Actions};



fn main() {

    let config = get_config();
    let api = Api::from_token(config.telegram_bot_token.as_str()).unwrap();
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    listener.listen(|u| {
        // If the received update contains a message...
        if let Some(m) = u.message {
            // if the message was a text message:
            if let MessageType::Text(text) = m.msg {
                if m.from.id != config.telegram_owner_id {
                    try!(api.send_message(
                        m.chat.id(),
                        String::from("Lol, nope"),
                        None, None, None, None)
                    );
                } else {
                    let message_type = check_message(&text);
                    let response_message = match message_type {
                        Some(Actions::AddTorrent) => do_add_torrent_action(&config, &text),
                        None => String::from("Sorry, I don't understand"),
                    };

                    try!(api.send_message(
                        m.chat.id(),
                        response_message,
                        None, None, None, None)
                    );
                }
            }
        }

        Ok(ListeningAction::Continue)
    }).unwrap();
}
