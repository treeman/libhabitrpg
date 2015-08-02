use std::fmt::{ Display, Formatter, Error };

use data::date::Date;
use data::print;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Habit {
    pub text: String,
    //attribute: String, // "str" wut?
    pub priority: f32,
    pub value: f32,
    pub notes: String,
    pub dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
    pub id: String,
    pub down: bool,
    pub up: bool,
    //history: Vec<String>, // TODO
}

impl Display for Habit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", print::remove_emoji(&self.text))
    }
}

