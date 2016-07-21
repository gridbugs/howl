use ecs::entity_table::EntityTable;
use ecs::system_queue::SystemQueue;
use ecs::message::Message;
use ecs::entity::Component::*;
use ecs::entity::ComponentType as Type;
use ecs::message::Field::*;
use ecs::message::FieldType;

use terminal::window_manager::WindowRef;
use colour::ansi;
use grid::static_grid::StaticGrid;

use std::fmt;

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

pub struct WindowRenderer<'a> {
    window: WindowRef<'a>,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(window: WindowRef<'a>) -> Self {
        WindowRenderer {
            window: window,
        }
    }

    pub fn process_message(&mut self, message: &mut Message,
                           entities: &mut EntityTable, _: &SystemQueue)
    {
        if let Some(&RenderLevel { level: level_id }) = message.get(FieldType::RenderLevel) {

            if let Some(&Level(ref level)) = entities.get(level_id).get(Type::Level) {
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
                    let window_cell = self.window.get_cell(coord.x, coord.y);
                    window_cell.set(cell.ch, cell.fg, cell.bg);
                }
            }
        }
    }
}

impl<'a> fmt::Debug for WindowRenderer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WindowRenderer {{}}")
    }
}
