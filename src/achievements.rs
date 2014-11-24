use std::collections::HashMap;

#[deriving(Show, Encodable, Decodable)]
pub struct Achievements {
    pub beastMaster: bool,
    //challenges: Vec<?>,
    pub perfect: uint,
    pub quests: HashMap<String, uint>,
    pub streak: uint,
    pub ultimateGear: bool,
}

