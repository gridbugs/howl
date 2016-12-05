use game::{GameKnowledge, LevelKnowledge, SpatialHashCell, ActionEnv};
use grid::DynamicGrid;
use util::BestMap;
use math::Coord;
use frontends::ansi::ComplexTile;

pub type AnsiDrawableKnowledge = GameKnowledge<AnsiDrawableKnowledgeLevel>;

pub struct AnsiDrawableKnowledgeCell {
    last_updated: u64,
    foreground: BestMap<isize, ComplexTile>,
    background: BestMap<isize, ComplexTile>,
}

impl AnsiDrawableKnowledgeCell {
    fn new() -> Self {
        AnsiDrawableKnowledgeCell {
            last_updated: 0,
            foreground: BestMap::new(),
            background: BestMap::new(),
        }
    }

    pub fn foreground(&self) -> Option<ComplexTile> {
        self.foreground.value()
    }

    pub fn background(&self) -> Option<ComplexTile> {
        self.background.value()
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }
}

impl Default for AnsiDrawableKnowledgeCell {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AnsiDrawableKnowledgeLevel {
    grid: DynamicGrid<AnsiDrawableKnowledgeCell>,
}

impl AnsiDrawableKnowledgeLevel {
    pub fn new() -> Self {
        AnsiDrawableKnowledgeLevel {
            grid: DynamicGrid::new(),
        }
    }

    pub fn get_with_default(&self, coord: Coord) -> &AnsiDrawableKnowledgeCell {
        self.grid.get_with_default(coord)
    }
}

impl LevelKnowledge for AnsiDrawableKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {
        let mut changed = false;
        let knowledge_cell = self.grid.get_mut_with_default(coord);
        if knowledge_cell.last_updated <= world_cell.last_updated() {
            changed = true;

            knowledge_cell.foreground.clear();
            knowledge_cell.background.clear();
            for entity in action_env.ecs.entity_iter(world_cell.entity_id_iter()) {
                entity.tile_depth().map(|depth| {
                    entity.ansi_tile().map(|tile| {
                        knowledge_cell.foreground.insert(depth, tile);
                        if tile.opaque_bg() {
                            knowledge_cell.background.insert(depth, tile);
                        }
                    });
                });
            }
        }
        knowledge_cell.last_updated = action_env.id;

        changed
    }
}

impl Default for AnsiDrawableKnowledgeLevel {
    fn default() -> Self {
        Self::new()
    }
}