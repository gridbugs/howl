use game::{AnsiDrawableKnowledgeLevel, AnsiDrawableKnowledgeCell};
use frontends::ansi::{self, ComplexTile, SimpleTile};
use math::Coord;
use direction::Direction;

const MOON_COLOUR: ansi::AnsiColour = ansi::colours::MAGENTA;

pub struct AnsiRenderer<'a> {
    window: ansi::Window<'a>,
}

impl<'a> AnsiRenderer<'a> {
    pub fn new(window: ansi::Window<'a>) -> Self {
        AnsiRenderer {
            window: window,
        }
    }

    fn cell_has_wall(cell: &AnsiDrawableKnowledgeCell) -> bool {
        if let Some(ComplexTile::Wall {..}) = cell.foreground() {
            true
        } else if let Some(ComplexTile::Wall {..}) = cell.background() {
            true
        } else {
            false
        }
    }

    fn simple_tile(tile: ComplexTile, coord: Coord, knowledge: &AnsiDrawableKnowledgeLevel) -> SimpleTile {
        match tile {
            ComplexTile::Simple(s) => s,
            ComplexTile::Wall { front, back } => {
                let south_coord = coord + Direction::South.vector();
                let cell = knowledge.get_with_default(south_coord);
                if Self::cell_has_wall(cell) {
                    back
                } else {
                    front
                }
            }
        }
    }

    pub fn render(&mut self, knowledge: &AnsiDrawableKnowledgeLevel, turn_id: u64, top_left: Coord, width: usize, height: usize) {
        for i in 0..(height as isize + top_left.y) {
            for j in 0..(width as isize + top_left.x) {
                let coord = Coord::new(j, i);
                let cell = knowledge.get_with_default(coord);

                let mut bg = ansi::colours::DARK_GREY;
                let mut fg = ansi::colours::DARK_GREY;
                let mut ch = ' ';
                let mut style = ansi::styles::NONE;

                if let Some(bg_tile) = cell.background() {
                    let tile = Self::simple_tile(bg_tile, coord, knowledge);
                    if let Some(c) = tile.background_colour() {
                        bg = c;
                    }
                    if let Some(c) = tile.character() {
                        ch = c;
                    }
                    if let Some(s) = tile.style() {
                        style = s;
                    }
                }

                if let Some(fg_tile) = cell.foreground() {
                    let tile = Self::simple_tile(fg_tile, coord, knowledge);
                    if let Some(c) = tile.foreground_colour() {
                        fg = c;
                    }
                    if let Some(c) = tile.character() {
                        ch = c;
                    }
                    if let Some(s) = tile.style() {
                        style = s;
                    }
                }

                if cell.moon() {
                    bg = MOON_COLOUR;
                }

                if cell.last_updated() != turn_id {
                    fg = ansi::colours::BLACK;
                    bg = ansi::colours::DARK_GREY;
                }

                self.window.get_cell(coord.x, coord.y).set(ch, fg, bg, style);
            }
        }
        self.window.flush();
    }
}
