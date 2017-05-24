use config::{BotConfig};
use std::io::Read;
use hyper::Client;
use hyper::header::Headers;

header! { (XTransmissionSessionId, "X-Transmission-Session-Id") => [String] }


pub fn get_transmission_session_id(transmission_url: &str) -> String {
    let client = Client::new();
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

pub fn do_add_torrent_action(config: &BotConfig, msg: &str) -> String {

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
    status
}
