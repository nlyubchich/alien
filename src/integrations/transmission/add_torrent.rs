use std::io::Read;
use hyper::Client;
use hyper::header::Headers;
use serde_json;
use config::{BotConfig};

use integrations::transmission::common::XTransmissionSessionId;

enum Statuses {
    TorrentDuplicate,
    TorrentAdded,
    Unknown,
}

#[derive(Deserialize, Debug)]
struct CommonArgoment {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Arguments {
    #[serde(rename="torrent-duplicate")]
    torrent_duplicate: Option<CommonArgoment>,
    #[serde(rename="torrent-added")]
    torrent_added: Option<CommonArgoment>,
}

impl Arguments {
    pub fn get_argument_status(&self) -> Statuses {
        let mut status = Statuses::Unknown;
        match self.torrent_added {
            Some(_) => status = Statuses::TorrentAdded,
            None => (),
        }
        match self.torrent_duplicate {
            Some(_) => status = Statuses::TorrentDuplicate,
            None => (),
        }
        return status;
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    result: String,
    arguments: Arguments,
}

pub fn add_torrent_action(config: &BotConfig, msg: &str) -> String {

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

    let mut res = client
        .post(config.transmission_url.as_str())
        .headers(headers)
        .body(&json.to_string())
        .send()
        .unwrap();

    let mut status = String::new();
    res.read_to_string(&mut status).unwrap();
    let p: Response = serde_json::from_str(status.as_str()).unwrap();

    match p.result.as_ref() {
        "success" => {
            let args = p.arguments;
            match args.get_argument_status() {
                Statuses::TorrentDuplicate => format!("duplicated {}", args.torrent_duplicate.unwrap().name),
                Statuses::TorrentAdded => format!("added new {}", args.torrent_added.unwrap().name),
                Statuses::Unknown => String::from("Unknown result"),
            }
        },
        _ => format!("Transmission didn't return \"success\" status: {:?}", p),
    }
}
