use game::{LevelKnowledge, SpatialHashCell, Turn};
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
    fn update_cell(&mut self, coord: Coord, _world_cell: &SpatialHashCell, _accuracy: f64, _turn: Turn) {
        let _knowledge_cell = self.grid.get_mut_with_default(coord);
    }
}
