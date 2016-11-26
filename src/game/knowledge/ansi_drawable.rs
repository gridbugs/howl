use game::{LevelKnowledge, SpatialHashCell};
use grid::DynamicGrid;
use util::BestMap;
use math::Coord;
use frontends::ansi::ComplexTile;

pub struct AnsiDrawableKnowledgeCell {
    foreground: BestMap<isize, ComplexTile>,
    background: BestMap<isize, ComplexTile>,
}

impl AnsiDrawableKnowledgeCell {
    fn new() -> Self {
        AnsiDrawableKnowledgeCell {
            foreground: BestMap::new(),
            background: BestMap::new(),
        }
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
}

impl LevelKnowledge for AnsiDrawableKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, turn: u64) {
    }
}
