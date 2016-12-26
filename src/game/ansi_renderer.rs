use game::{AnsiDrawableKnowledgeLevel, AnsiDrawableKnowledgeCell};
use frontends::ansi::{self, ComplexTile, SimpleTile};
use math::Coord;
use direction::Direction;

const MOON_COLOUR: ansi::AnsiColour = ansi::colours::MAGENTA;

pub struct AnsiRenderer<'a> {
    window: ansi::Window<'a>,
    scroll: bool,
}

impl<'a> AnsiRenderer<'a> {
    pub fn new(window: ansi::Window<'a>, scroll: bool) -> Self {
        AnsiRenderer {
            window: window,
            scroll: scroll,
        }
    }

    fn width(&self) -> usize {
        self.window.width()
    }

    fn height(&self) -> usize {
        self.window.height()
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

    pub fn render(&mut self, knowledge: &AnsiDrawableKnowledgeLevel, turn_id: u64, position: Coord) {
        let width = self.width() as isize;
        let height = self.height() as isize;
        let offset = if self.scroll {
            position - Coord::new(width / 2, height / 2)
        } else {
            Coord::new(0, 0)
        };

        for i in 0..height {
            for j in 0..width {
                let window_coord = Coord::new(j, i);
                let world_coord = window_coord + offset;
                let cell = knowledge.get_with_default(world_coord);

                let mut bg = ansi::colours::DARK_GREY;
                let mut fg = ansi::colours::DARK_GREY;
                let mut ch = ' ';
                let mut style = ansi::styles::NONE;

                if let Some(bg_tile) = cell.background() {
                    let tile = Self::simple_tile(bg_tile, world_coord, knowledge);
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
                    let tile = Self::simple_tile(fg_tile, world_coord, knowledge);
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

                self.window.get_cell(window_coord.x, window_coord.y).set(ch, fg, bg, style);
            }
        }
        self.window.flush();
    }
}
