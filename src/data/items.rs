use std::collections::HashMap;

use data::Gear;
use data::Date;

#[deriving(Show, Encodable, Decodable)]
pub struct LastDrop {
    count: uint,
    date: Date,
}

#[deriving(Show, Encodable, Decodable)]
pub struct Items {
    currentMount: Option<String>, // Sometimes "" ??
    currentPet: Option<String>,
    eggs: HashMap<String, uint>,
    food: HashMap<String, uint>,
    gear: Gear,
    lastDrop: LastDrop,
    hatchingPotions: HashMap<String, uint>,
    mounts: HashMap<String, bool>,
    pets: HashMap<String, int>,
    quests: HashMap<String, int>,
}

