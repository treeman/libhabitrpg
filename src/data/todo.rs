use std::fmt::{ Show, Formatter, Error };

use data::date::Date;
use data::print;

#[deriving(Encodable, Decodable)]
pub struct Todo {
    pub text: String,
    //attribute: String, // "str" wut?
    pub priority: f32,
    pub value: f32,
    pub notes: String,
    pub dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
    pub id: String,
    // checklist
    // collapseChecklist
    pub completed: bool,
    // archived: bool,
    // dateCompleted: Date,
    // challenge?
}

impl Show for Todo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = print::remove_emoji(self.text[]).to_string();
        if self.completed {
            s.push_str("(Done)");
        };
        write!(f, "{}", s)
    }
}

