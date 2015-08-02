use rustc_serialize::{ Encodable, Decoder, Encoder };

use data::{
    Date,
    Achievements,
    Habit,
    Daily,
    Todo,
    Reward,
    Stats,
    Tag,
    Items,
    Profile,
};

// TODO custom Show
#[derive(Debug, RustcEncodable, RustcDecodable)]
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
    pub items: Items,
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
    pub fn print_char(&self) {
        println!("{}, level {} {}", self.profile.name, self.stats.lvl, self.stats.class)
    }

    pub fn print_char_stats(&self) {
        println!("  {}/{:?} hp", self.stats.hp, self.stats.maxHealth);
        println!("  {:?}/{:?} mp", self.stats.mp, self.stats.maxMP);
        println!("  {}/{:?} xp", self.stats.exp, self.stats.toNextLevel);
    }

    // Filter out separators, starting with #, which I use as delimiters in habitrpg website.
    pub fn dailys<'a>(&'a self) -> Vec<&'a Daily> {
        self.dailys.iter().filter(|t: &&Daily| {
            !&t.text.starts_with("#")
        }).collect()
    }

    pub fn habits<'a>(&'a self) -> Vec<&'a Habit> {
        self.habits.iter().filter(|t: &&Habit| {
            !&t.text.starts_with("#")
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

