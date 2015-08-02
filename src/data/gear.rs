use std::collections::HashMap;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Gear {
    costume: HashMap<String, String>,
    equipped: HashMap<String, String>,
    owned: HashMap<String, bool>,
}

