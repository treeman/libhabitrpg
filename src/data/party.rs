use data::{
    Quest,
    Achievements,
    Stats,
    Items,
    Profile,
};

// TODO should parse from other place?
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Party {
    // chat
    //    id
    //    likes
    //    text
    //    timestamp
    //    uuid
    // ordering
    pub quest: Quest, // TODO should be optional!
    // leader
    pub memberCount: usize,
    pub members: Vec<PartyMember>,
    pub name: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct PartyMember {
    //
    achievements: Achievements,
    // auth
    //   timestamps
    //      created
    //      loggedin
    items: Items,
    // party ?
    // preferences..
    profile: Profile,
    stats: Stats,
    // auth
    //    timestamps
    //        created
    //        loggedin
}

