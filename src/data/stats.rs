
//#[deriving(Show, Encodable, Decodable)]
//pub struct Buffs {
    //constitution: usize,
    //intelligence: usize,
    //perception: usize,
    //strength: usize,
    //stealth: usize,
    //// snowball: bool ?
    //// streaks: bool ?
//}

// TODO custom Show
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Stats {
    //buffs: Buffs,
    pub class: String, // Or class...
    pub con: usize,
    pub int: usize,
    pub per: usize,
    pub str: usize,
    pub exp: f32,
    pub gp: f32,
    pub hp: f32,
    pub mp: f32,
    pub maxHealth: Option<usize>,
    pub maxMP: Option<usize>,
    pub lvl: usize,
    pub points: usize, // ??
    pub toNextLevel: Option<f32>, // usize?
    // training ?
}
