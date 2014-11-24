use serialize::{ Decoder, Decodable, json };
use std::io::{ File, Open, Read };

pub fn from_file<D: Decodable<json::Decoder, json::DecoderError>>(loc: &str) -> D {
    let path = Path::new(loc);
    let mut file = match File::open_mode(&path, Open, Read) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e)
    };

    let contents: String = match file.read_to_string() {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e)
    };

    let json_object = match json::from_str(contents[]) {
        Ok(v) => v,
        Err(e) => panic!("json parse error: {}", e)
    };
    let mut decoder = json::Decoder::new(json_object);

    match Decodable::decode(&mut decoder) {
        Ok(v) => v,
        Err(e) => panic!("Decoding error: {}", e)
    }
}
