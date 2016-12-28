use game::*;
use grid::{Grid, StaticGrid, DefaultGrid};
use util::{BestMap, TwoDimensionalCons};
use math::Coord;
use frontends::ansi::ComplexTile;

pub type AnsiDrawableKnowledge = GameKnowledge<AnsiDrawableKnowledgeLevel>;

pub struct AnsiDrawableKnowledgeCell {
    last_updated: u64,
    foreground: BestMap<isize, ComplexTile>,
    background: BestMap<isize, ComplexTile>,
    moon: bool,
}

impl AnsiDrawableKnowledgeCell {
    fn new() -> Self {
        AnsiDrawableKnowledgeCell {
            last_updated: 0,
            foreground: BestMap::new(),
            background: BestMap::new(),
            moon: false,
        }
    }

    pub fn foreground(&self) -> Option<ComplexTile> {
        self.foreground.value()
    }

    pub fn background(&self) -> Option<ComplexTile> {
        self.background.value()
    }

    pub fn moon(&self) -> bool {
        self.moon
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }

    pub fn update(&mut self, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {

        let mut changed = false;

        if self.last_updated <= world_cell.last_updated() {

            self.moon = world_cell.moon();
            self.foreground.clear();
            self.background.clear();
            for entity in action_env.ecs.entity_iter(world_cell.entity_id_iter()) {
                entity.tile_depth().map(|depth| {
                    entity.ansi_tile().map(|tile| {
                        self.foreground.insert(depth, tile);
                        if tile.opaque_bg() {
                            self.background.insert(depth, tile);
                        }
                    });
                });
            }

            changed = true;
        }

        self.last_updated = action_env.id;

        changed
    }
}

impl Default for AnsiDrawableKnowledgeCell {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AnsiDrawableKnowledgeLevel {
    grid: StaticGrid<AnsiDrawableKnowledgeCell>,
    default: AnsiDrawableKnowledgeCell,
}

impl AnsiDrawableKnowledgeLevel {
    pub fn get_with_default(&self, coord: Coord) -> &AnsiDrawableKnowledgeCell {
        self.grid.get(coord).unwrap_or_else(|| &self.default)
    }
}

impl LevelKnowledge for AnsiDrawableKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, action_env: ActionEnv) -> bool {
        if let Some(knowledge_cell) = self.grid.get_mut(coord) {
            knowledge_cell.update(world_cell, accuracy, action_env)
        } else {
            false
        }
    }
}

impl TwoDimensionalCons for AnsiDrawableKnowledgeLevel {
    fn new(width: usize, height: usize) -> Self {
        AnsiDrawableKnowledgeLevel {
            grid: StaticGrid::new_default(width, height),
            default: AnsiDrawableKnowledgeCell::new(),
        }
    }
}
