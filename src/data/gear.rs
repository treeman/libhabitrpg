use std::collections::HashMap;

#[deriving(Show, Encodable, Decodable)]
pub struct Gear {
    costume: HashMap<String, String>,
    equipped: HashMap<String, String>,
    owned: HashMap<String, bool>,
}

