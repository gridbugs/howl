#[derive(Clone, Copy, Debug)]
pub struct Rgba32 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Rgba32 {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Rgba32 {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
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

    pub fn alpha(self) -> u8 {
        self.alpha
    }
}
