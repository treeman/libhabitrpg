use std::fmt::{ Show, Formatter, Result };
use std::io::{ File, Open, Read };
use serialize::{ json, Decodable };

// API_TOKEN and USER_ID for habitrpg identifiers.
#[deriving(Decodable)]
pub struct Id  {
    pub api_token: String,
    pub user_id: String,
}

impl Show for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "api_token: {} user_id: {}", self.api_token, self.user_id)
    }
}

impl Id {
    pub fn from_file(loc: &str) -> Id {
        let path = Path::new(loc);
        let mut file = match File::open_mode(&path, Open, Read) {
            Ok(f) => f,
            Err(e) => panic!("file error: {}", e)
        };

        let contents: String = match file.read_to_string() {
            Ok(f) => f,
            Err(e) => panic!("file error: {}", e)
        };

        let json_object = match json::from_str(contents[]) {
            Ok(v) => v,
            Err(e) => panic!("json parse error: {}", e)
        };
        let mut decoder = json::Decoder::new(json_object);

        match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => panic!("Decoding error: {}", e)
        }
    }
}

