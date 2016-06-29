use renderer::colour::Rgb24;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    character: char,
    colour: Rgb24,
}

impl Tile {
    pub fn new(character: char, colour: Rgb24) -> Tile {
        Tile { character: character, colour: colour }
    }

    pub fn new_rgb(character: char, red: u8, green: u8, blue: u8) -> Tile {
        Tile::new(character, Rgb24::new(red, green, blue))
    }
}
