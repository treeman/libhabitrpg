use std::fmt::{ Debug, Formatter, Result };

// API_TOKEN and USER_ID for habitrpg identifiers.
#[derive(RustcDecodable, Eq, PartialEq)]
pub struct Id  {
    pub api_token: String,
    pub user_id: String,
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "api_token: {} user_id: {}", self.api_token, self.user_id)
    }
}

//#[test]
//fn id() {
    //use json_helpers::*;
    //let s = r#"{ "api_token": "token", "user_id": "id" }"#;
    //let id: Id = json_helpers::from_str(s);
    //assert_eq!(id, Id { api_token: "token".to_string(), user_id: "id".to_string() });
//}

