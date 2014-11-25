#![feature(macro_rules)]
#![feature(phase)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(globs)]
#![feature(slicing_syntax)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate serialize;
extern crate time;
extern crate http;
extern crate url;

pub use id::Id;
pub use data::*;
pub use json_helpers::*;

use request::get;

mod data;
mod json_helpers;
mod id;
mod request;

pub fn get_user_response(id: &Id) -> String {
    get("https://beta.habitrpg.com/api/v2/user", id)
}

pub fn get_party_response(id: &Id) -> String {
    get("https://beta.habitrpg.com/api/v2/groups/party", id)
}

pub fn get_user(id: &Id) -> User {
    json_helpers::from_str(get_user_response(id)[])
}

pub fn get_party(id: &Id) -> Party {
    json_helpers::from_str(get_party_response(id)[])
}

pub fn load_user(file: &str) -> User {
    json_helpers::from_file(file)
}

pub fn load_party(file: &str) -> Party {
    json_helpers::from_file(file)
}

