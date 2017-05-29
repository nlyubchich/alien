use std::io::Read;
use hyper::Client;
use hyper::header::Headers;
use serde_json;
use config::{BotConfig};
use std::ops::Mul;

use integrations::transmission::common::XTransmissionSessionId;

#[derive(Deserialize, Debug)]
struct TorrentItem {
    name: String,
    status: i64,
    #[serde(rename="percentDone")]
    percent_done: f64,
}

#[derive(Deserialize, Debug)]
struct Arguments {
    torrents: Vec<TorrentItem>,
}

#[derive(Deserialize, Debug)]
struct Response {
    result: String,
    arguments: Option<Arguments>,
}

pub fn get_torrent_list_action(config: &BotConfig, _: &str) -> String {
    let client = Client::new();

    let json = json!({
        "method":"torrent-get",
        "arguments": {
            "fields":[
                "id","name","error","errorString","eta","isFinished","isStalled",
                "leftUntilDone","metadataPercentComplete","peersConnected",
                "percentDone","rateDownload","rateUpload","recheckProgress",
                "seedRatioMode","seedRatioLimit","sizeWhenDone","status",
                "downloadDir","uploadedEver","uploadRatio","webseedsSendingToUs"
            ],
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
            let mut result = String::from("Here is your torrents:");

            for torrent in p.arguments.unwrap().torrents {
                result += format!("\n{} - {} ({}%)", torrent.name, torrent.status, torrent.percent_done.mul(100f64)).as_str();
            }

            result
        },
        _ => format!("Transmission didn't return \"success\" status: {:?}", p),
    }
}
