use game::entity::{
    EntityTable,
    EntityId
};
use game::knowledge::DrawableCell;

use grid::{
    Grid,
    IterGrid,
    Coord,
};
use terminal::window_manager::WindowRef;
use colour::ansi;
use renderer::{
    ComplexTile,
    SimpleTile,
};
use geometry::Direction;

struct RendererCell {
    wall: bool,
}

impl Default for RendererCell {
    fn default() -> Self {
        RendererCell {
            wall: false,
        }
    }
}

pub struct WindowKnowledgeRenderer<'a> {
    window: WindowRef<'a>,
}

fn cell_has_wall(cell: &DrawableCell) -> bool {
    if let Some(ComplexTile::Wall { front: _, back: _ }) = cell.foreground.value() {
        true
    } else if let Some(ComplexTile::Wall { front: _, back: _ }) = cell.background.value() {
        true
    } else {
        false
    }
}

impl<'a> WindowKnowledgeRenderer<'a> {
    pub fn new(window: WindowRef<'a>) -> Self {
        WindowKnowledgeRenderer {
            window: window,
        }
    }

    fn get_simple_tile<G>(&self, tile: ComplexTile, coord: Coord, grid: &G) -> SimpleTile
        where G: Grid<Item=DrawableCell>
    {
        match tile {
            ComplexTile::Simple(s) => s,
            ComplexTile::Wall { front, back } => {
                if let Some(cell) = grid.get_nei(coord, Direction::South) {
                    if cell_has_wall(cell) {
                        back
                    } else {
                        front
                    }
                } else {
                    front
                }
            },
        }
    }

    pub fn render(&self,
                  entities: &EntityTable,
                  entity_id: EntityId,
                  turn_count: u64)
    {
        let entity = entities.get(entity_id);
        let level_id = entity.on_level().unwrap();
        let knowledge = entity.drawable_knowledge().unwrap();
        let grid = knowledge.grid(level_id).unwrap();

        for (coord, cell) in izip!(
            grid.coord_iter(),
            grid.iter())
        {
            let window_cell = self.window.get_cell(coord.x, coord.y);

            let mut bg = ansi::DARK_GREY;
            let mut fg = ansi::DARK_GREY;
            let mut ch = ' ';

            if let Some(bg_tile) = cell.background.value() {
                let tile = self.get_simple_tile(bg_tile, coord, grid);
                if let Some(c) = tile.background_colour() {
                    bg = c;
                }
                if let Some(c) = tile.character() {
                    ch = c;
                }
            }

            if let Some(fg_tile) = cell.foreground.value() {
                let tile = self.get_simple_tile(fg_tile, coord, grid);
                if let Some(c) = tile.foreground_colour() {
                    fg = c;
                }
                if let Some(c) = tile.character() {
                    ch = c;
                }
            }

            if cell.last_turn != turn_count {
                fg = ansi::BLACK;
                bg = ansi::DARK_GREY;
            }

            window_cell.set(ch, fg, bg);
        }

        self.window.flush();
    }
}
