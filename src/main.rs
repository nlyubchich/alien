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
use telegram_bot::{Api,ListeningMethod,ListeningAction};
use handle::process_update;



fn main() {
    let config = get_config();
    let api = Api::from_token(config.telegram_bot_token.as_str()).unwrap();
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    listener.listen(|update| {
        if let Some(message) = update.message {
            let response = process_update(&config, &message);
            try!(api.send_message(
                message.chat.id(),
                response,
                None, None, None, None
            ));
        }
        Ok(ListeningAction::Continue)
    }).unwrap();
}
