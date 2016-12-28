use std::collections::HashSet;

use game::*;
use grid::{Grid, StaticGrid, DefaultGrid};
use math::Coord;
use search::TraverseCost;
use util::TwoDimensionalCons;

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

    fn update(&mut self, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {
        let mut changed = false;

        if self.last_updated <= world_cell.last_updated() {
            changed = true;

            self.solid = world_cell.solid();
        }

        self.last_updated = action_env.id;

        changed
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
    grid: StaticGrid<SimpleNpcKnowledgeCell>,
    targets: HashSet<Coord>,
    latest_target: u64,
    default: SimpleNpcKnowledgeCell,
}

impl SimpleNpcKnowledgeLevel {
    pub fn get_with_default(&self, coord: Coord) -> &SimpleNpcKnowledgeCell {
        self.grid.get(coord).unwrap_or_else(|| &self.default)
    }

    pub fn grid(&self) -> &StaticGrid<SimpleNpcKnowledgeCell> {
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
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, action_env: ActionEnv) -> bool {
        if let Some(knowledge_cell) = self.grid.get_mut(coord) {
            if knowledge_cell.update(world_cell, accuracy, action_env) {
                if world_cell.pc() {
                    if action_env.id == self.latest_target {
                        self.targets.insert(coord);
                    } else if action_env.id > self.latest_target {
                        self.targets.clear();
                        self.targets.insert(coord);
                        self.latest_target = action_env.id;
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl TwoDimensionalCons for SimpleNpcKnowledgeLevel {
    fn new(width: usize, height: usize) -> Self {
        SimpleNpcKnowledgeLevel {
            grid: StaticGrid::new_default(width, height),
            targets: HashSet::new(),
            latest_target: 0,
            default: SimpleNpcKnowledgeCell::new(),
        }
    }
}
