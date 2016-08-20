use game::entity::{
    EntityTable,
    EntityId,
};
use game::entity::Component::*;
use game::entity::ComponentType as Type;

use terminal::window_manager::WindowRef;
use colour::ansi;
use grid::{
    Grid,
    DefaultGrid,
    IterGrid,
    StaticGrid,
};

struct Cell {
    ch: char,
    fg: ansi::AnsiColour,
    bg: ansi::AnsiColour,
    bg_depth: isize,
    fg_depth: isize,
}

impl Cell {
    fn new(ch: char, fg: ansi::AnsiColour, bg: ansi::AnsiColour, fg_depth: isize, bg_depth: isize) -> Self {
        Cell {
            ch: ch,
            fg: fg,
            bg: bg,
            fg_depth: fg_depth,
            bg_depth: bg_depth,
        }
    }

    fn clear(&mut self) {
        self.fg_depth = -1;
        self.bg_depth = -1;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(' ', ansi::WHITE, ansi::DARK_GREY, -1, -1)
    }
}

pub struct WindowRenderer<'a> {
    window: WindowRef<'a>,
    grid: StaticGrid<Cell>,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(window: WindowRef<'a>) -> Self {
        WindowRenderer {
            window: window,
            grid: StaticGrid::new_default(window.width(), window.height()),
        }
    }

    pub fn render(&mut self, entities: &EntityTable, level_id: EntityId) {

        for cell in self.grid.iter_mut() {
            cell.clear();
        }

        if let Some(&LevelData(ref level)) = entities.get(level_id).get(Type::LevelData) {
            for entity in level.entities(entities) {
                if let Some(&Position(ref v)) = entity.get(Type::Position) {
                    let cell = &mut self.grid[v];
                    if let Some(&TileDepth(depth)) = entity.get(Type::TileDepth) {
                        if let Some(&SolidTile{ref tile, background: bg}) = entity.get(Type::SolidTile) {
                            if depth > cell.bg_depth {
                                cell.bg_depth = depth;
                                cell.bg = bg;
                            }
                            if depth > cell.fg_depth {
                                cell.fg_depth = depth;
                                cell.fg = tile.colour;
                                cell.ch = tile.character;
                            }
                        } else if let Some(&TransparentTile(ref tile)) = entity.get(Type::TransparentTile) {
                            if depth > cell.fg_depth {
                                cell.fg_depth = depth;
                                cell.fg = tile.colour;
                                cell.ch = tile.character;
                            }
                        }
                    }
                }
            }

            for (coord, cell) in izip!(self.grid.coord_iter(), self.grid.iter()) {
                let window_cell = self.window.get_cell(coord.x, coord.y);
                window_cell.set(cell.ch, cell.fg, cell.bg);
            }
        }

        self.window.flush();
    }
}
