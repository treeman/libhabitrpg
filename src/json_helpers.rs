use rustc_serialize::{ Decodable, json };
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn from_file<D: Decodable>(path: &str) -> D {
    from_path(Path::new(path))
}

pub fn from_path<D: Decodable>(path: &Path) -> D {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e)
    };

    println!("{}: {:?}", path.display(), file);
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => panic!("file reading error: {}", e)
    }

    from_str(&contents)
}

pub fn from_str<D: Decodable>(s: &str) -> D {
    let json_object = match Json::from_str(s) {
        Ok(v) => v,
        Err(e) => panic!("json parse error: {}", e)
    };
    let mut decoder = json::Decoder::new(json_object);

    match Decodable::decode(&mut decoder) {
        Ok(v) => v,
        Err(e) => panic!("Decoding error: {}", e)
    }
}

#[test]
fn file_reading() {
    use Id;
    let id: Id = from_file("test/id.json");
    assert_eq!(id.api_token, "PASSWORD2");
    assert_eq!(id.user_id, "TERMINATOR_XXL_IMBAZ0R");
}

