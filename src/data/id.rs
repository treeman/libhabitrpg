use std::fmt::{ Show, Formatter, Result };

use json_helpers;

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
        json_helpers::from_file(loc)
    }
}

