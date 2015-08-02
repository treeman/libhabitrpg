
// Could instead use a HashMap?
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Tag {
    pub id: String,
    pub name: String,
}
