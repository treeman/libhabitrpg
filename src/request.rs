use hyper::Client;
use std::io::Read;

use id::Id;

header! { (XApiKey, "x-api-key") => [String] }
header! { (XApiUser, "x-api-user") => [String] }

/// A GET request with authentication headers attached.
/// TODO error handling!
pub fn get(url: &str, id: &Id) -> String {
    let client = Client::new();
    let mut res = client.get(url)
        .header(XApiKey(id.api_token.clone()))
        .header(XApiUser(id.user_id.clone()))
        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

