use hyper::Client;

mod add_torrent;
mod list_torrents;
mod common;

pub use self::add_torrent::add_torrent_action;
pub use self::list_torrents::get_torrent_list_action;
use self::common::XTransmissionSessionId;


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
