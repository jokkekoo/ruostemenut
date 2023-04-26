use serde_json;


pub fn make_request(uri: String) -> String {
    let resp = reqwest::blocking::get(uri)?
        .text();
    resp
}