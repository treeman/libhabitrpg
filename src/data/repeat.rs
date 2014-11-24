use time;

#[deriving(Show, Encodable, Decodable)]
pub struct Repeat  {
    pub su: bool,
    pub m: bool,
    pub t: bool,
    pub w: bool,
    pub th: bool,
    pub f: bool,
    pub s: bool,
}

impl Repeat {
    pub fn due_today(&self) -> bool {
        let t = time::now();
        match t.tm_wday {
            0 => self.su,
            1 => self.m,
            2 => self.t,
            3 => self.w,
            4 => self.th,
            5 => self.f,
            6 => self.s,
            _ => panic!("Tm.tm_wday errorenous: {}", t.tm_wday),
        }
    }
}

