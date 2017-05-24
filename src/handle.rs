pub enum Actions {
    AddTorrent
}

// fn process_request(config, msg) -> String {
//     ""
// }

pub fn check_message(msg: &str) -> Option<Actions> {
    if msg.starts_with("magnet:") {
        return Some(Actions::AddTorrent);
    }
    None
}
