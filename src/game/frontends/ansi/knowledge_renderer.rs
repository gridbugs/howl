use game::*;
use game::frontends::ansi::resolve_tile;
use frontends::ansi::{self, ComplexTile, SimpleTile};
use math::Coord;

const MOON_COLOUR: ansi::AnsiColour = ansi::colours::MAGENTA;

pub struct AnsiKnowledgeRenderer {
    window: ansi::Window,
    buffer: TileBuffer,
    scroll: bool,
}

impl AnsiKnowledgeRenderer {
    pub fn new(window: ansi::Window, scroll: bool) -> Self {
        let width = window.width();
        let height = window.height();

        AnsiKnowledgeRenderer {
            window: window,
            buffer: TileBuffer::new(width, height),
            scroll: scroll,
        }
    }

    fn simple_tile(tile: ComplexTile, is_front: bool) -> SimpleTile {
        match tile {
            ComplexTile::Simple(s) => s,
            ComplexTile::Wall { front, back } => {
                if is_front {
                    front
                } else {
                    back
                }
            }
        }
    }
}

impl KnowledgeRenderer for AnsiKnowledgeRenderer {

    fn width(&self) -> usize {
        self.window.width()
    }

    fn height(&self) -> usize {
        self.window.height()
    }

    fn update(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, position: Coord) {
        let scroll_position = if self.scroll {
            Some(position)
        } else {
            None
        };

        self.buffer.update(knowledge, turn_id, scroll_position);
    }

    fn draw(&mut self) {
        for (coord, cell) in izip!(self.buffer.coord_iter(), self.buffer.iter()) {
            let mut bg = ansi::colours::DARK_GREY;
            let mut fg = ansi::colours::DARK_GREY;
            let mut ch = ' ';
            let mut style = ansi::styles::NONE;

            if let Some(bg_tile_type) = cell.background {
                let bg_tile = resolve_tile::resolve_tile(bg_tile_type);
                let tile = Self::simple_tile(bg_tile, cell.front);
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

            if let Some(fg_tile_type) = cell.foreground {
                let fg_tile = resolve_tile::resolve_tile(fg_tile_type);
                let tile = Self::simple_tile(fg_tile, cell.front);
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

            if cell.moon {
                bg = MOON_COLOUR;
            }

            if !cell.visible {
                fg = ansi::colours::BLACK;
                bg = ansi::colours::DARK_GREY;
            }

            self.window.get_cell(coord.x, coord.y).set(ch, fg, bg, style);
        }

        self.window.flush();
    }
}
