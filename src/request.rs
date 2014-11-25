use http::client::RequestWriter;
use http::method::Get;
use url::Url;

use id::Id;

/// TODO error handling!

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

