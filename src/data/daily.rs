use std::fmt::{ Show, Formatter, Error };

use data::date::Date;
use data::repeat::Repeat;
use data::print;

#[deriving(Encodable, Decodable)]
pub struct Daily {
    pub text: String,
    //attribute: String, // "str" Some value...
    pub priority: f32,
    pub value: f32,
    pub notes: String,
    pub dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
    pub id: String,
    pub streak: uint,
    // checklist
    // collapseChecklist
    pub repeat: Repeat,
    pub completed: bool,
    //history: Vec<String>, // TODO
}

impl Daily {
    pub fn due_today(&self) -> bool {
        self.repeat.due_today()
    }
}

impl Show for Daily {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", print::remove_emoji(self.text[]))
    }
}

