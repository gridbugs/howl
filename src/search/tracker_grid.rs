use search::{Path, PathNode};
use grid::{Coord, StaticGrid, DefaultGrid};
use geometry::Direction;

const INITIAL_GRID_WIDTH: usize = 100;
const INITIAL_GRID_HEIGHT: usize = 60;

#[derive(Clone, Copy)]
struct SeenCell {
    seen_seq: u64,
    visited_seq: u64,
    cost: f64,
    parent: Option<Coord>,
    direction: Option<Direction>,
}

impl Default for SeenCell {
    fn default() -> Self {
        SeenCell {
            seen_seq: 0,
            visited_seq: 0,
            cost: 0.0,
            parent: None,
            direction: None,
        }
    }
}

pub struct TrackerGrid {
    grid: StaticGrid<SeenCell>,
    seq: u64,
    explored: u64,
}

impl TrackerGrid {
    pub fn new() -> Self {
        TrackerGrid {
            grid: StaticGrid::new_default(INITIAL_GRID_WIDTH, INITIAL_GRID_HEIGHT),
            seq: 0,
            explored: 0,
        }
    }

    pub fn clear(&mut self) {
        self.seq += 1;
        self.explored = 0;
    }

    fn see(&mut self, coord: Coord) {
        let mut cell = &mut self.grid[coord];
        cell.seen_seq = self.seq;
        cell.parent = None;
    }

    pub fn see_with_cost(&mut self, coord: Coord, cost: f64) -> bool {
        if self.is_visited(coord) {
            return false;
        }

        if let Some(existing_cost) = self.cost(coord) {
            if cost < existing_cost {
                self.grid[coord].cost = cost;
                true
            } else {
                false
            }
        } else {
            self.see(coord);
            self.grid[coord].cost = cost;
            true
        }
    }

    pub fn see_with_parent(&mut self,
                           coord: Coord,
                           cost: f64,
                           parent_coord: Coord,
                           direction: Direction)
                           -> bool {
        if self.see_with_cost(coord, cost) {
            self.grid[coord].parent = Some(parent_coord);
            self.grid[coord].direction = Some(direction);
            true
        } else {
            false
        }
    }

    fn is_seen(&self, coord: Coord) -> bool {
        self.grid[coord].seen_seq == self.seq
    }

    fn cost(&self, coord: Coord) -> Option<f64> {
        if self.is_seen(coord) {
            Some(self.grid[coord].cost)
        } else {
            None
        }
    }

    pub fn visit(&mut self, coord: Coord) {
        self.grid[coord].visited_seq = self.seq;
        self.explored += 1;
    }

    pub fn is_visited(&self, coord: Coord) -> bool {
        self.grid[coord].visited_seq == self.seq
    }

    pub fn populate_path(&self, start: Coord, path: &mut Path) {
        path.nodes.clear();
        path.start = start;
        path.cost = self.grid[start].cost;

        let mut coord = start;

        loop {
            if !self.is_visited(coord) {
                panic!("cell not visited");
            }

            if let Some(parent) = self.grid[coord].parent {
                let direction = self.grid[coord].direction.unwrap();
                path.nodes.push(PathNode::new(coord, direction));
                coord = parent;
            } else {
                path.nodes.reverse();
                break;
            }
        }
    }
}
