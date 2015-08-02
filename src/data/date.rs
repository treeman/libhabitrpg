use std::fmt::{ Display, Formatter, Error };
use rustc_serialize::{ Encodable, Decodable, Decoder, Encoder };
use std::result::Result;

// For now just parse 2014-06-27T18:22:05.834Z
// which is simply hardcoded. Might want to use datetime library when it has been made.
#[derive(Debug)]
pub struct Date {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
    sec: usize,
    ms: usize,
}

impl Date {
    // Not very stable, but works atm?
    pub fn from_str(s: &str) -> Option<Date> {
        let re = regex!(r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})\.(\d{3})Z");
        let caps = re.captures(s);
        let res = caps.and_then(|x| {
            Some(Date {
                // Yes this is ugly as crap. Is there a way to fix this? :)
                year: x.at(1).unwrap().parse().unwrap(),
                month: x.at(2).unwrap().parse().unwrap(),
                day: x.at(3).unwrap().parse().unwrap(),
                hour: x.at(4).unwrap().parse().unwrap(),
                min: x.at(5).unwrap().parse().unwrap(),
                sec: x.at(6).unwrap().parse().unwrap(),
                ms: x.at(7).unwrap().parse().unwrap(),
            })
        });
        res
    }

    pub fn to_string(&self) -> String {
        format!("{}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}.{:0>3}Z",
                self.year, self.month, self.day,
                self.hour, self.min, self.sec, self.ms)
    }
}

impl Decodable for Date {
    fn decode<D: Decoder>(d: &mut D) -> Result<Date, D::Error> {
        d.read_str().and_then(|x| {
            match Date::from_str(&x) {
                Some(x) => Ok(x),
                None => panic!("Failed to parse Date from {}", x)
            }
        })
    }
}

impl Encodable for Date {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(&self.to_string())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", &self.to_string())
    }
}

