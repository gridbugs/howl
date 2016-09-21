use grid::Coord;
use geometry::Direction;

#[derive(Debug, Clone)]
pub struct PathNode {
    pub coord: Coord,
    pub direction: Direction,
}

impl PathNode {
    pub fn new(coord: Coord, direction: Direction) -> Self {
        PathNode {
            coord: coord,
            direction: direction,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub coords: Vec<PathNode>,
    pub cost: f64,
    pub explored: u64,
    pub index: usize,
}

impl Path {
    pub fn new(coords: Vec<PathNode>, cost: f64, explored: u64) -> Self {
        Path {
            coords: coords,
            cost: cost,
            explored: explored,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Option<&PathNode> {
        if self.index < self.coords.len() {
            let ret = &self.coords[self.index];
            self.index += 1;
            Some(ret)
        } else {
            None
        }
    }
}
