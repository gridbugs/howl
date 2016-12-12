use std::collections::HashSet;

use game::*;
use grid::DynamicGrid;
use math::Coord;
use search::TraverseCost;

pub type SimpleNpcKnowledge = GameKnowledge<SimpleNpcKnowledgeLevel>;

pub struct SimpleNpcKnowledgeCell {
    last_updated: u64,
    solid: bool,
}

impl SimpleNpcKnowledgeCell {
    fn new() -> Self {
        SimpleNpcKnowledgeCell {
            last_updated: 0,
            solid: false,
        }
    }
}

impl Default for SimpleNpcKnowledgeCell {
    fn default() -> Self {
        Self::new()
    }
}

impl TraverseCost for SimpleNpcKnowledgeCell {
    fn traverse_cost(&self) -> Option<f64> {
        if self.solid {
            None
        } else {
            Some(1.0)
        }
    }
}

pub struct SimpleNpcKnowledgeLevel {
    grid: DynamicGrid<SimpleNpcKnowledgeCell>,
    targets: HashSet<Coord>,
    latest_target: u64,
}

impl SimpleNpcKnowledgeLevel {
    pub fn new() -> Self {
        SimpleNpcKnowledgeLevel {
            grid: DynamicGrid::new(),
            targets: HashSet::new(),
            latest_target: 0,
        }
    }

    pub fn get_with_default(&self, coord: Coord) -> &SimpleNpcKnowledgeCell {
        self.grid.get_with_default(coord)
    }

    pub fn grid(&self) -> &DynamicGrid<SimpleNpcKnowledgeCell> {
        &self.grid
    }

    pub fn last_target_update(&self) -> u64 {
        self.latest_target
    }

    pub fn contains_target(&self, coord: Coord) -> bool {
        self.targets.contains(&coord)
    }
}

impl LevelKnowledge for SimpleNpcKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {
        let mut changed = false;
        let knowledge_cell = self.grid.get_mut_with_default(coord);
        if knowledge_cell.last_updated <= world_cell.last_updated() {
            changed = true;

            knowledge_cell.solid = world_cell.solid();

            if world_cell.pc() {
                if action_env.id == self.latest_target {
                    self.targets.insert(coord);
                } else if action_env.id > self.latest_target {
                    self.targets.clear();
                    self.targets.insert(coord);
                    self.latest_target = action_env.id;
                }
            }
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
