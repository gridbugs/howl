pub const NUM_FRONTENDS: usize = 1;
pub const FRONTENDS: [Frontend; NUM_FRONTENDS] = [Frontend::Ansi];
pub const FRONTEND_STRINGS: [&'static str; NUM_FRONTENDS] = ["ansi"];

#[derive(Debug)]
pub enum Frontend {
    Ansi,
    Sdl,
}

impl Frontend {
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "ansi" => Some(Frontend::Ansi),
            "sdl" => Some(Frontend::Sdl),
            _ => None,
        }
    }
}
