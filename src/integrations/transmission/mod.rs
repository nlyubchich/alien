use std::io::Read;
use hyper::Client;
use hyper::header::Headers;

mod add_torrent;
mod list_torrents;
mod common;

use config::BotConfig;
use self::add_torrent::add_torrent_action;
use self::list_torrents::get_torrent_list_action;
use self::common::XTransmissionSessionId;
use handle::BotIntegration;
// use

pub fn get_transmission_session_id(client: &Client, transmission_url: &str) -> String {
    let json = json!({
      "method": "session-get",
    });
    let res = client
        .post(transmission_url)
        .body(&json.to_string())
        .send()
        .unwrap();
    let headers = &res.headers;
    headers.get::<XTransmissionSessionId>()
        .expect("Could not get X-Transmission-Session-Id")
        .to_string()
}

pub enum Actions {
    AddTorrent,
    ListTorrents,
}

// pub fn process_request(message) -> String {
//     if let telegram::MessageType::Text(text) = message.clone().msg {
//         let message_type = check_message(&text);
//         return match message_type {
//             Some(Actions::AddTorrent) => add_torrent_action(&config, &text),
//             Some(Actions::ListTorrents) => get_torrent_list_action(&config, &text),
//             None => String::from("Sorry, I don't understand"),
//         };
//     }
//     String::from("Sorry, I don't understand")
// }

pub struct ApiClient {
    http_client: Client,
    session_id: String,
    api_endpoint: String,
}

pub trait SendTransmissionRpc {
    fn send_request(&self, payload: String) -> String;
}

impl ApiClient {
    pub fn new(client: Client, api_endpoint: String) -> ApiClient {
        ApiClient {
            session_id: get_transmission_session_id(
                &client, api_endpoint.clone().as_str()
            ),
            http_client: client,
            api_endpoint: api_endpoint.clone(),
        }
    }
}
impl SendTransmissionRpc for ApiClient {
    fn send_request(&self, payload: String) -> String {
        let mut headers = Headers::new();
        headers.set(XTransmissionSessionId(self.session_id.clone()));
        let mut res = self.http_client
            .post(self.api_endpoint.as_str())
            .headers(headers)
            .body(&payload)
            .send()
            .unwrap();

        let mut status = String::new();
        res.read_to_string(&mut status).unwrap();
        status
    }
}

pub struct Transmission {
    api_client: ApiClient,
}

impl Transmission {
    fn get_message_type(&self, message: &str) -> Option<Actions> {
        match message {
            message if message.starts_with("magnet:") => Some(Actions::AddTorrent),
            message if message == "list torrents" => Some(Actions::ListTorrents),
            _ => None,
        }
    }
}


impl BotIntegration<Transmission> for Transmission {
    fn new(config: &BotConfig) -> Transmission {
        let client = Client::new();
        Transmission {
            api_client: ApiClient::new(
                client, config.transmission_rpc_url.clone()
            )
        }
    }

    fn dispatch(&self, message: String) -> String {
        let client = &self.api_client;
        match self.get_message_type(message.as_str()) {
            Some(Actions::AddTorrent) => add_torrent_action(client, &message),
            Some(Actions::ListTorrents) => get_torrent_list_action(client),
            None => String::from("Sorry, I don't understand"),
        }
    }
}
