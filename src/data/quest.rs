
// TODO fix
#[deriving(Show, Encodable, Decodable)]
pub struct Quest {
    pub active: bool,
    // completed: May be null, or a completed quest eg "gryphon"
    pub key: String, // May also be null
    pub progress: QuestProgress,
}

#[deriving(Show, Encodable, Decodable)]
pub struct QuestProgress {
    // collect
    pub hp: f32,
}

