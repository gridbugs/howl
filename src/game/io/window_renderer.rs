use game::entity::{EntityTable, EntityId};
use game::entity::Component::*;
use game::entity::ComponentType as Type;

use terminal::window_manager::WindowRef;
use colour::ansi;
use grid::StaticGrid;

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
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(' ', ansi::WHITE, ansi::BLACK, -1, -1)
    }
}

pub fn render<'a>(window: WindowRef<'a>, entities: &EntityTable, level_id: EntityId) {
    if let Some(&LevelData(ref level)) = entities.get(level_id).get(Type::LevelData) {
        let mut grid = StaticGrid::<Cell>::new_default(level.width as isize, level.height as isize);

        for entity in level.entities(entities) {
            if let Some(&Position(ref v)) = entity.get(Type::Position) {
                let cell = &mut grid[v];
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

        for (coord, cell) in izip!(grid.coord_iter(), grid.iter()) {
            let window_cell = window.get_cell(coord.x, coord.y);
            window_cell.set(cell.ch, cell.fg, cell.bg);
        }
    }

    window.flush();

}
