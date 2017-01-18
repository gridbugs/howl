#[derive(Clone, Copy, Debug)]
pub struct Rgb24 {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb24 {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb24 {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn red(self) -> u8 {
        self.red
    }

    pub fn green(self) -> u8 {
        self.green
    }

    pub fn blue(self) -> u8 {
        self.blue
    }
}

pub mod colours {
    use super::*;

    pub const RED: Rgb24 = Rgb24 { red: 255, green: 0, blue: 0 };
    pub const GREEN: Rgb24 = Rgb24 { red: 0, green: 255, blue: 0 };
    pub const BLUE: Rgb24 = Rgb24 { red: 0, green: 0, blue: 255 };

    pub const PURPLE: Rgb24 = Rgb24 { red: 0x99, green: 0x00, blue: 0xcc };
}
