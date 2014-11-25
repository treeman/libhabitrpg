use http::client::RequestWriter;
use http::method::Get;
use url::Url;

use id::Id;
use json_helpers;
use data::{ User, Party };

/// TODO error handling!

pub fn get_user(id: &Id) -> User {
    let response = get("https://beta.habitrpg.com/api/v2/user", id);
    json_helpers::from_str(response[])
}

pub fn get_party(id: &Id) -> Party {
    let response = get("https://beta.habitrpg.com/api/v2/user", id);
    json_helpers::from_str(response[])
}

/// A GET request with authentication headers attached.
pub fn get(url: &str, id: &Id) -> String {
    let url = match Url::parse(url) {
        Ok(x) => x,
        Err(e) => panic!("Invalid URL: {}", e),
    };
    let mut request: RequestWriter = RequestWriter::new(Get, url).unwrap();
    request.headers.extensions.insert(String::from_str("x-api-key"), id.api_token.clone());
    request.headers.extensions.insert(String::from_str("x-api-user"), id.user_id.clone());

    //println!("[1mHeaders:[0m");
    //for header in request.headers.iter() {
        //println!(" - {}: {}", header.header_name(), header.header_value());
    //}

    let mut response = match request.read_response() {
        Ok(x) => x,
        Err(_) => panic!("No response!"),
    };
    //println!("[1mStatus:[0m {}", response.status);
    //println!("[1mHeaders:[0m");
    //for header in response.headers.iter() {
        //println!(" - {}: {}", header.header_name(), header.header_value());
    //}
    //println!("[1mBody:[0m");
    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(err) => panic!("Reading response failed: {}", err),
    };
    String::from_utf8_lossy(body[]).into_string()
}

