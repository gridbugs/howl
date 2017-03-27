use std::collections::HashSet;

use grid::{Grid, StaticGrid, DefaultGrid};
use math::Coord;
use knowledge::GameKnowledge;
use dimension_constructor::TwoDimensionalCons;
use search::TraverseCost;

pub type SimpleNpcKnowledge = GameKnowledge<SimpleNpcKnowledgeLevel>;

#[derive(Serialize, Deserialize)]
pub struct SimpleNpcKnowledgeCell {
    pub last_updated: u64,
    pub solid: bool,
    pub acid: bool,
}

impl SimpleNpcKnowledgeCell {
    pub fn new() -> Self {
        SimpleNpcKnowledgeCell {
            last_updated: 0,
            solid: false,
            acid: false,
        }
    }

    pub fn solid(&self) -> bool { self.solid }
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
        } else if self.acid {
            Some(2.0)
        } else {
            Some(1.0)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleNpcKnowledgeLevel {
    pub grid: StaticGrid<SimpleNpcKnowledgeCell>,
    pub targets: HashSet<Coord>,
    pub latest_target: u64,
    pub default: SimpleNpcKnowledgeCell,
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

    pub fn any_target(&self) -> Option<Coord> {
        for target in self.targets.iter() {
            return Some(*target);
        }
        None
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
