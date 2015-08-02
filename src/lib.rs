#![allow(non_snake_case)]
#![allow(dead_code)]

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

extern crate rustc_serialize;
extern crate time;
#[macro_use]
extern crate hyper;
extern crate url;

pub use id::Id;
pub use data::*;
pub use json_helpers::*;

use request::get;

mod data;
mod json_helpers;
mod id;
mod request;

// TODO error handling!!

pub fn get_user_response(id: &Id) -> String {
    get("https://beta.habitrpg.com/api/v2/user", id)
}

pub fn get_party_response(id: &Id) -> String {
    get("https://beta.habitrpg.com/api/v2/groups/party", id)
}

pub fn get_user(id: &Id) -> User {
    json_helpers::from_str(&get_user_response(id))
}

pub fn get_party(id: &Id) -> Party {
    json_helpers::from_str(&get_party_response(id))
}

pub fn load_user(file: &str) -> User {
    json_helpers::from_file(file)
}

pub fn load_party(file: &str) -> Party {
    json_helpers::from_file(file)
}

