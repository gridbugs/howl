use game::{Level, LevelId, EntityId, EntityWrapper, EntityStore};
use game::knowledge::{KnowledgeCell, DrawableCell, DrawableExtra};

use grid::{Grid, IterGrid, Coord};
use terminal::{Window, style};
use colour::ansi;
use tile::{ComplexTile, SimpleTile};
use geometry::Direction;

const MOONLIGHT_COLOUR: ansi::AnsiColour = ansi::MAGENTA;

struct RendererCell {
    wall: bool,
}

impl Default for RendererCell {
    fn default() -> Self {
        RendererCell { wall: false }
    }
}

pub struct WindowKnowledgeRenderer<'a> {
    window: Window<'a>,
}

fn cell_has_wall(cell: &DrawableExtra) -> bool {
    if let Some(ComplexTile::Wall { front: _, back: _ }) = cell.foreground.value() {
        true
    } else if let Some(ComplexTile::Wall { front: _, back: _ }) = cell.background.value() {
        true
    } else {
        false
    }
}

impl<'a> WindowKnowledgeRenderer<'a> {
    pub fn new(window: Window<'a>) -> Self {
        WindowKnowledgeRenderer { window: window }
    }

    fn get_simple_tile<G>(&self, tile: ComplexTile, coord: Coord, grid: &G) -> SimpleTile
        where G: Grid<Item = DrawableCell>
    {
        match tile {
            ComplexTile::Simple(s) => s,
            ComplexTile::Wall { front, back } => {
                if let Some(cell) = grid.get_nei(coord, Direction::South) {
                    if cell_has_wall(cell.extra()) {
                        back
                    } else {
                        front
                    }
                } else {
                    front
                }
            }
        }
    }

    pub fn render(&mut self,
                  entities: &Level,
                  entity_id: EntityId,
                  level_id: LevelId,
                  turn_count: u64) {
        let entity = entities.get(entity_id).unwrap();
        let knowledge = entity.drawable_knowledge().unwrap();
        let grid = knowledge.grid(level_id).unwrap();

        for (coord, cell_common) in izip!(grid.coord_iter(), grid.iter()) {
            let cell = cell_common.extra();

            let mut bg = ansi::DARK_GREY;
            let mut fg = ansi::DARK_GREY;
            let mut ch = ' ';
            let mut style = style::NONE;

            if let Some(bg_tile) = cell.background.value() {
                let tile = self.get_simple_tile(bg_tile, coord, grid);
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

            if let Some(fg_tile) = cell.foreground.value() {
                let tile = self.get_simple_tile(fg_tile, coord, grid);
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

            if cell.moonlight {
                bg = MOONLIGHT_COLOUR;
            }

            if cell_common.last_updated_turn() != turn_count {
                fg = ansi::BLACK;
                bg = ansi::DARK_GREY;
            }

            self.window.get_cell(coord.x, coord.y).set(ch, fg, bg, style);
        }

        self.window.flush();
    }
}
