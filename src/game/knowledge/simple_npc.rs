use game::*;
use grid::DynamicGrid;
use math::Coord;

pub type SimpleNpcKnowledge = GameKnowledge<SimpleNpcKnowledgeLevel>;

pub struct SimpleNpcKnowledgeCell {
    last_updated: u64,
    solid: bool,
    pc: bool,
}

impl SimpleNpcKnowledgeCell {
    fn new() -> Self {
        SimpleNpcKnowledgeCell {
            last_updated: 0,
            solid: false,
            pc: false,
        }
    }

    fn solid(&self) -> bool {
        self.solid
    }

    fn pc(&self) -> bool {
        self.pc
    }
}

impl Default for SimpleNpcKnowledgeCell {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SimpleNpcKnowledgeLevel {
    grid: DynamicGrid<SimpleNpcKnowledgeCell>,
}

impl SimpleNpcKnowledgeLevel {
    pub fn new() -> Self {
        SimpleNpcKnowledgeLevel {
            grid: DynamicGrid::new(),
        }
    }

    pub fn get_with_default(&self, coord: Coord) -> &SimpleNpcKnowledgeCell {
        self.grid.get_with_default(coord)
    }
}

impl LevelKnowledge for SimpleNpcKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {
        let mut changed = false;
        let knowledge_cell = self.grid.get_mut_with_default(coord);
        if knowledge_cell.last_updated <= world_cell.last_updated() {
            changed = true;

            knowledge_cell.solid = world_cell.solid();
            knowledge_cell.pc = world_cell.pc();
        }
        knowledge_cell.last_updated = action_env.id;

        changed
    }
}

impl Default for SimpleNpcKnowledgeLevel {
    fn default() -> Self {
        Self::new()
    }
}
