use std::collections::HashMap;

use data::Gear;
use data::Date;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LastDrop {
    count: usize,
    date: Date,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Items {
    currentMount: Option<String>, // Sometimes "" ??
    currentPet: Option<String>,
    eggs: HashMap<String, usize>,
    food: HashMap<String, usize>,
    gear: Gear,
    lastDrop: LastDrop,
    hatchingPotions: HashMap<String, usize>,
    mounts: HashMap<String, bool>,
    pets: HashMap<String, isize>,
    quests: HashMap<String, isize>,
}

