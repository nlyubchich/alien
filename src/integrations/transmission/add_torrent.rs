use serde_json;
use integrations::transmission::SendTransmissionRpc;

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
        if self.torrent_added.is_some() { status = Statuses::TorrentAdded }
        if self.torrent_duplicate.is_some() { status = Statuses::TorrentDuplicate }
        status
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    result: String,
    arguments: Arguments,
}

pub fn add_torrent_action<T: SendTransmissionRpc>(client: &T, msg: &str) -> String {
    let json = json!({
      "method": "torrent-add",
      "arguments": {
          "paused": false,
          "filename": msg
      }
    });

    let res = client.send_request(json.to_string());
    let p: Response = serde_json::from_str(res.as_str()).unwrap();

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
