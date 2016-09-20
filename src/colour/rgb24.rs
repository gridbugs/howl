#[derive(Clone, Copy, Debug)]
pub struct Rgb24 {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb24 {
    pub fn new(red: u8, green: u8, blue: u8) -> Rgb24 {
        Rgb24 {
            red: red,
            green: green,
            blue: blue,
        }
    }
}

pub const BLACK: Rgb24 = Rgb24 {
    red: 0,
    green: 0,
    blue: 0,
};
pub const WHITE: Rgb24 = Rgb24 {
    red: 255,
    green: 255,
    blue: 255,
};
pub const RED: Rgb24 = Rgb24 {
    red: 255,
    green: 0,
    blue: 0,
};
pub const GREEN: Rgb24 = Rgb24 {
    red: 0,
    green: 255,
    blue: 0,
};
pub const BLUE: Rgb24 = Rgb24 {
    red: 0,
    green: 0,
    blue: 255,
};
