use game::*;
use game::frontends::ansi::resolve_tile;
use frontends::ansi::{self, ComplexTile, SimpleTile, AnsiColour, Style};
use coord::Coord;

const MOON_COLOUR: ansi::AnsiColour = ansi::colours::MAGENTA;
const AIM_LINE_COLOUR: ansi::AnsiColour = ansi::colours::YELLOW;
const AIM_COLOUR_SHORT: ansi::AnsiColour = ansi::colours::GREEN;
const AIM_COLOUR_NORMAL: ansi::AnsiColour = ansi::colours::BLUE;
const AIM_COLOUR_LONG: ansi::AnsiColour = ansi::colours::RED;
const AIM_COLOUR_OUT_OF_RANGE: ansi::AnsiColour = ansi::colours::BLACK;

struct AnsiInfo {
    ch: char,
    fg: AnsiColour,
    bg: AnsiColour,
    style: Style,
}

impl Default for AnsiInfo {
    fn default() -> Self {
        AnsiInfo {
            bg: ansi::colours::DARK_GREY,
            fg: ansi::colours::DARK_GREY,
            ch: ' ',
            style: ansi::styles::NONE,
        }
    }
}

pub struct AnsiKnowledgeRenderer {
    window: ansi::Window,
    buffer: TileBuffer,
    scroll: bool,
    scroll_position: Coord,
}

impl AnsiKnowledgeRenderer {
    pub fn new(window: ansi::Window, scroll: bool) -> Self {
        let width = window.width();
        let height = window.height();

        AnsiKnowledgeRenderer {
            window: window,
            buffer: TileBuffer::new(width, height),
            scroll: scroll,
            scroll_position: Coord::new(0, 0),
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

    fn to_ansi_info(cell: &CellDrawInfo) -> AnsiInfo {
        let mut info = AnsiInfo::default();

        if let Some(bg_tile_type) = cell.background {
            let bg_tile = resolve_tile::resolve_tile(bg_tile_type);
            let tile = Self::simple_tile(bg_tile, cell.front);
            if let Some(c) = tile.background_colour() {
                info.bg = c;
            }
            if let Some(c) = tile.character() {
                info.ch = c;
            }
            if let Some(s) = tile.style() {
                info.style = s;
            }
        }

        if let Some(fg_tile_type) = cell.foreground {
            let fg_tile = resolve_tile::resolve_tile(fg_tile_type);
            let tile = Self::simple_tile(fg_tile, cell.front);
            if let Some(c) = tile.foreground_colour() {
                info.fg = c;
            }
            if let Some(c) = tile.character() {
                info.ch = c;
            }
            if let Some(s) = tile.style() {
                info.style = s;
            }
        }

        if cell.moon {
            info.bg = MOON_COLOUR;
        }

        if !cell.visible {
            info.fg = ansi::colours::BLACK;
            info.bg = ansi::colours::DARK_GREY;
        }

        info
    }

    fn draw_internal(&mut self) {
        for (coord, cell) in izip!(self.buffer.coord_iter(), self.buffer.iter()) {
            let info = Self::to_ansi_info(cell);
            self.window.get_cell(coord.x, coord.y).set(info.ch, info.fg, info.bg, info.style);
        }
    }

    fn world_to_screen(&self, coord: Coord) -> Coord {
        coord - self.scroll_position
    }

    fn draw_overlay_internal(&mut self, overlay: &RenderOverlay) {
        if let Some(ref aim_line) = overlay.aim_line {
            for coord in aim_line.iter() {
                let screen_coord = self.world_to_screen(coord);
                if let Some(cell) = self.buffer.get(screen_coord) {
                    let mut info = Self::to_ansi_info(cell);
                    info.bg = AIM_LINE_COLOUR;
                    self.window.get_cell(screen_coord.x, screen_coord.y).set(info.ch, info.fg, info.bg, info.style);
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

    fn world_offset(&self) -> Coord {
        self.scroll_position
    }

    fn update(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, position: Coord) {
        let scroll_position = if self.scroll {
            Some(position)
        } else {
            None
        };

        self.scroll_position = self.buffer.update(knowledge, turn_id, scroll_position);
    }

    fn draw(&mut self) {
        self.draw_internal();
        self.window.flush();
    }

    fn draw_with_overlay(&mut self, overlay: &RenderOverlay) {
        self.draw_internal();
        self.draw_overlay_internal(overlay);
        self.window.flush();
    }
}
