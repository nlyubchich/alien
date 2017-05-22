extern crate telegram_bot;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate hyper;

use std::io::Read;
use telegram_bot::{Api,ListeningMethod,MessageType,ListeningAction};
use hyper::Client;

use hyper::header::Headers;
header! { (XTransmissionSessionId, "X-Transmission-Session-Id") => [String] }

const TRANSMISSION_RPC_LINK: &str = "http://torrent.example.com/transmission/rpc";

enum Actions {
    AddTorrent
}

struct BotConfig {
    transmission_session_id: String,
    owner_id: i64
}


fn check_message(msg: &str) -> Option<Actions> {
    if msg.starts_with("magnet:") {
        return Some(Actions::AddTorrent);
    }
    None
}

fn do_add_torrent_action(config: &BotConfig, msg: &str) -> String {
    let client = Client::new();

    let json = json!({
      "method": "torrent-add",
      "arguments": {
          "paused": false,
          "filename": msg
      }
    });

    let mut headers = Headers::new();
    headers.set(XTransmissionSessionId(config.transmission_session_id.clone()));

    let payload: String = json.to_string();
    let mut res = client
        .post(TRANSMISSION_RPC_LINK)
        .headers(headers)
        .body(payload.as_str())
        .send()
        .unwrap();

    let mut status = String::new();
    res.read_to_string(&mut status).unwrap();
    status
}

fn get_transmission_session_id() -> String {
    let client = Client::new();
    let json = json!({
      "method": "session-get",
    });
    let res = client
        .post(TRANSMISSION_RPC_LINK)
        .body(json.to_string().as_str())
        .send()
        .unwrap();
    let headers = &res.headers;
    headers.get::<XTransmissionSessionId>()
        .expect("Could not get X-Transmission-Session-Id")
        .to_string()
}


fn main() {
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    let config = BotConfig {
        transmission_session_id: get_transmission_session_id(),
        owner_id: 275296204,  // TODO: get owner_id from some external config (maybe env?)
    };

    listener.listen(|u| {
        // If the received update contains a message...
        if let Some(m) = u.message {
            // if the message was a text message:
            if let MessageType::Text(text) = m.msg {
                if m.from.id != config.owner_id {
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
