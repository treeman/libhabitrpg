use std::collections::HashMap;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Achievements {
    pub beastMaster: bool,
    //challenges: Vec<?>,
    pub perfect: usize,
    pub quests: HashMap<String, usize>,
    pub streak: usize,
    pub ultimateGear: bool,
}

