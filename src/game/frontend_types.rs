pub const NUM_FRONTENDS: usize = 1;
pub const FRONTENDS: [Frontend; NUM_FRONTENDS] = [Frontend::Sdl];
pub const FRONTEND_STRINGS: [&'static str; NUM_FRONTENDS] = ["sdl"];

#[derive(Debug)]
pub enum Frontend {
    Sdl,
}

impl Frontend {
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "sdl" => Some(Frontend::Sdl),
            _ => None,
        }
    }
}
