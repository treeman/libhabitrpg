use serialize::{ Encodable, Decoder, Encoder };

use data::date::Date;
use data::achievements::Achievements;
use data::habit::Habit;
use data::daily::Daily;
use data::todo::Todo;
use data::reward::Reward;
use data::stats::Stats;
use data::tag::Tag;
//use data::id::Id;

use json_helpers;

#[deriving(Show, Encodable, Decodable)]
pub struct Items {
    pub currentMount: String,
    pub currentPet: String,
}

#[deriving(Show, Encodable, Decodable)]
pub struct Profile {
    pub name: String,
}

// TODO custom Show
#[deriving(Show, Encodable, Decodable)]
pub struct User {
    pub achievements: Achievements,

    // Made private so we can filter away unwanted things.
    habits: Vec<Habit>,
    dailys: Vec<Daily>,
    todos: Vec<Todo>,
    rewards: Vec<Reward>,

    // filters?
    //flags: Flags, // skip?
    // history exp/todo
    pub id: String,
    // invitations
    //pub items: Items,
    pub lastCron: Date,
    // newMessages ?
    // pub party: Party, // Parsed from other place
    // preferences
    // profile (name...)
    pub profile: Profile,
    pub stats: Stats,
    pub tags: Vec<Tag>, // TODO dictionary
}

impl User {
    pub fn from_file(loc: &str) -> User {
        json_helpers::from_file(loc)
    }

    pub fn print_char(&self) {
        println!("{}, level {} {}", self.profile.name, self.stats.lvl, self.stats.class)
    }

    pub fn print_char_stats(&self) {
        println!("  {}/{} hp", self.stats.hp, self.stats.maxHealth);
        println!("  {}/{} mp", self.stats.mp, self.stats.maxMP);
        println!("  {}/{} xp", self.stats.exp, self.stats.toNextLevel);
    }

    // Filter out separators, starting with #, which I use as delimiters in habitrpg website.
    pub fn dailys<'a>(&'a self) -> Vec<&'a Daily> {
        self.dailys.iter().filter(|t: &&Daily| {
            !t.text.as_slice().starts_with("#")
        }).collect()
    }

    pub fn habits<'a>(&'a self) -> Vec<&'a Habit> {
        self.habits.iter().filter(|t: &&Habit| {
            !t.text.as_slice().starts_with("#")
        }).collect()
    }

    //// Fetch unfinished todos as opposed to all todos.
    pub fn unfinished_todos<'a>(&'a self) -> Vec<&'a Todo> {
        self.todos.iter().filter(|t: &&Todo| {
            !t.completed
        }).collect()
    }

    //pub fn print_todays_stats(&self) {
        //// TODO
    //}

    pub fn print_task_stats(&self) {
        println!("  {} habits", self.habits.len());
        println!("  {} dailys", self.dailys.len());
        println!("  {} todos", self.unfinished_todos().len());
    }

    pub fn print(&self) {
        self.print_char();
        self.print_char_stats();
        self.print_task_stats();
    }
}

#[test]
fn tst() {
    let user = User::from_file("data/user.json");
    user.print();
    assert!(false);
}

