use serde_json;
use std::ops::Mul;

use integrations::transmission::SendTransmissionRpc;

#[derive(Deserialize, Debug)]
struct TorrentItem {
    name: String,
    // Torrent._StatusStopped         = 0;
    // Torrent._StatusCheckWait       = 1;
    // Torrent._StatusCheck           = 2;
    // Torrent._StatusDownloadWait    = 3;
    // Torrent._StatusDownload        = 4;
    // Torrent._StatusSeedWait        = 5;
    // Torrent._StatusSeed            = 6;
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

pub fn get_torrent_list_action<T: SendTransmissionRpc>(client: &T) -> String {

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

    let status = client.send_request(json.to_string());

    let p: Response = serde_json::from_str(status.as_str()).unwrap();
    match p.result.as_ref() {
        "success" => {
            let mut result = String::from("Here are your torrents:");

            for torrent in p.arguments.unwrap().torrents {
                result += format!(
                    "\n{} - {} ({}%)",
                    torrent.name, torrent.status,
                    torrent.percent_done.mul(100f64)
                ).as_str();
            }

            result
        },
        _ => format!("Transmission didn't return \"success\" status: {:?}", p),
    }
}
