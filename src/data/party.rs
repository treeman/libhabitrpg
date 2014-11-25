use json_helpers;

use data::{
    Quest,
    Achievements,
    Stats,
    Items,
    Profile,
};

// TODO should parse from other place?
#[deriving(Show, Encodable, Decodable)]
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
    pub memberCount: uint,
    pub members: Vec<PartyMember>,
    pub name: String,
}

impl Party {
    pub fn from_file(loc: &str) -> Party {
        json_helpers::from_file(loc)
    }
}

#[deriving(Show, Encodable, Decodable)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go() {
        let party = Party::from_file("data/party.json");

        //println!("{}", party);
        for x in party.members.iter() {
            println!("{}", x.items);
        }
        assert!(false);
    }
}
