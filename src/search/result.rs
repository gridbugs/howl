use grid::Coord;
use geometry::Direction;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Path {
    pub coords: Vec<PathNode>,
    pub cost: f64,
    pub explored: u64,
}

impl Path {
    pub fn new(coords: Vec<PathNode>, cost: f64, explored: u64) -> Self {
        Path {
            coords: coords,
            cost: cost,
            explored: explored,
        }
    }
}
