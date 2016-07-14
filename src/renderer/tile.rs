use colour::ansi::AnsiColour;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub character: char,
    pub colour: AnsiColour,
}

impl Tile {
    pub fn new(character: char, colour: AnsiColour) -> Tile {
        Tile { character: character, colour: colour }
    }
}
