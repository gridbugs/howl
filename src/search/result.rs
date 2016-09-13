use grid::Coord;

#[derive(Debug)]
pub struct Path {
    pub coords: Vec<Coord>,
    pub cost: f64,
    pub explored: u64,
}

impl Path {
    pub fn new(coords: Vec<Coord>, cost: f64, explored: u64) -> Self {
        Path {
            coords: coords,
            cost: cost,
            explored: explored,
        }
    }
}
