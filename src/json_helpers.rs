use serialize::{ Decoder, Decodable, json };
use std::io::{ File, Open, Read };

pub fn from_file<D: Decodable<json::Decoder, json::DecoderError>>(loc: &str) -> D {
    let path = Path::new(loc);
    from_path(&path)
}

pub fn from_path<D: Decodable<json::Decoder, json::DecoderError>>(path: &Path) -> D {
    let mut file = match File::open_mode(path, Open, Read) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e)
    };

    let contents: String = match file.read_to_string() {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e)
    };

    from_str(contents[])
}

pub fn from_str<D: Decodable<json::Decoder, json::DecoderError>>(s: &str) -> D {
    let json_object = match json::from_str(s[]) {
        Ok(v) => v,
        Err(e) => panic!("json parse error: {}", e)
    };
    let mut decoder = json::Decoder::new(json_object);

    match Decodable::decode(&mut decoder) {
        Ok(v) => v,
        Err(e) => panic!("Decoding error: {}", e)
    }
}

