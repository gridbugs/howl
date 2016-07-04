// Error indicates value for a field is too high
#[derive(Debug)]
pub struct AnsiRangeError;

#[derive(Clone, Copy, Debug)]
pub enum NormalColour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Grey,
}

#[derive(Clone, Copy, Debug)]
pub enum BrightColour {
    DarkGrey,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

// In reality this is represented by a byte for red, green and blue for
// convenience. The maximum value for each field is 5.
#[derive(Clone, Copy, Debug)]
pub struct RgbColour {
    red: u8,
    green: u8,
    blue: u8,
}
const RGB_COLOUR_MAX_FIELD: u8 = 5;

impl RgbColour {
    fn new(red: u8, green: u8, blue: u8) -> Result<RgbColour, AnsiRangeError> {
        if red > RGB_COLOUR_MAX_FIELD ||
           green > RGB_COLOUR_MAX_FIELD ||
           blue > RGB_COLOUR_MAX_FIELD
        {
            Err(AnsiRangeError)
        } else {
            Ok(RgbColour { red: red, green: green, blue: blue })
        }
    }
}

// This can hold values from 0 to 23.
#[derive(Clone, Copy, Debug)]
pub struct GreyscaleColour(u8);
const GREYSCALE_COLOUR_MAX_FIELD: u8 = 23;

impl GreyscaleColour {
    fn new(value: u8) -> Result<GreyscaleColour, AnsiRangeError> {
        if value > GREYSCALE_COLOUR_MAX_FIELD {
            Err(AnsiRangeError)
        } else {
            Ok(GreyscaleColour(value))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AnsiColour {
    Normal(NormalColour),
    Bright(BrightColour),
    Rgb(RgbColour),
    Greyscale(GreyscaleColour),
}

// Normal colours
pub const BLACK: AnsiColour = AnsiColour::Normal(NormalColour::Black);
pub const RED: AnsiColour = AnsiColour::Normal(NormalColour::Red);
pub const GREEN: AnsiColour = AnsiColour::Normal(NormalColour::Green);
pub const YELLOW: AnsiColour = AnsiColour::Normal(NormalColour::Yellow);
pub const BLUE: AnsiColour = AnsiColour::Normal(NormalColour::Blue);
pub const MAGENTA: AnsiColour = AnsiColour::Normal(NormalColour::Magenta);
pub const CYAN: AnsiColour = AnsiColour::Normal(NormalColour::Cyan);
pub const GREY: AnsiColour = AnsiColour::Normal(NormalColour::Grey);

// Bright colours
pub const DARK_GREY: AnsiColour = AnsiColour::Bright(BrightColour::DarkGrey);
pub const BRIGHT_RED: AnsiColour = AnsiColour::Bright(BrightColour::Red);
pub const BRIGHT_GREEN: AnsiColour = AnsiColour::Bright(BrightColour::Green);
pub const BRIGHT_YELLOW: AnsiColour = AnsiColour::Bright(BrightColour::Yellow);
pub const BRIGHT_BLUE: AnsiColour = AnsiColour::Bright(BrightColour::Blue);
pub const BRIGHT_MAGENTA: AnsiColour = AnsiColour::Bright(BrightColour::Magenta);
pub const BRIGHT_CYAN: AnsiColour = AnsiColour::Bright(BrightColour::Cyan);
pub const WHITE: AnsiColour = AnsiColour::Bright(BrightColour::White);

impl AnsiColour {
    pub fn new_rgb(red: u8, green: u8, blue: u8) ->
        Result<AnsiColour,  AnsiRangeError>
    {
        RgbColour::new(red, green, blue).map(|colour| {
            AnsiColour::Rgb(colour)
        })
    }

    pub fn new_greyscale(value: u8) -> Result<AnsiColour, AnsiRangeError> {
        GreyscaleColour::new(value).map(|colour| {
            AnsiColour::Greyscale(colour)
        })
    }

    pub fn code(&self) -> u8 {
        match *self {
            AnsiColour::Normal(colour) => {
                match colour {
                    NormalColour::Black     => 0x00,
                    NormalColour::Red       => 0x01,
                    NormalColour::Green     => 0x02,
                    NormalColour::Yellow    => 0x03,
                    NormalColour::Blue      => 0x04,
                    NormalColour::Magenta   => 0x05,
                    NormalColour::Cyan      => 0x06,
                    NormalColour::Grey      => 0x07,
                }
            },
            AnsiColour::Bright(colour) => {
                match colour {
                    BrightColour::DarkGrey  => 0x08,
                    BrightColour::Red       => 0x09,
                    BrightColour::Green     => 0x0a,
                    BrightColour::Yellow    => 0x0b,
                    BrightColour::Blue      => 0x0c,
                    BrightColour::Magenta   => 0x0d,
                    BrightColour::Cyan      => 0x0e,
                    BrightColour::White     => 0x0f,
                }
            },
            AnsiColour::Rgb(RgbColour{red, green, blue}) => {
                16 + 36 * red + 6 * green + blue
            },
            AnsiColour::Greyscale(GreyscaleColour(value)) => {
                0xe8 + value
            },
        }
    }
}
