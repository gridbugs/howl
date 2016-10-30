use grid::Coord;
use geometry::Direction;

#[derive(Debug)]
pub struct PathNode {
    pub coord: Coord,
    pub in_direction: Direction,
}

pub struct Path {
    pub start: Coord,
    pub nodes: Vec<PathNode>,
    pub cost: f64,
}

impl PathNode {
    pub fn new(coord: Coord, in_direction: Direction) -> Self {
        PathNode {
            coord: coord,
            in_direction: in_direction,
        }
    }
}

impl Path {
    pub fn new() -> Self {
        Path {
            start: Coord::new(0, 0),
            nodes: Vec::new(),
            cost: 0.0,
        }
    }
}
