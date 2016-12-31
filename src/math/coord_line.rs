use std::slice;
use math::Coord;

pub struct CoordLineIter<'a>(slice::Iter<'a, Coord>);

impl<'a> Iterator for CoordLineIter<'a> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|c| *c)
    }
}

#[derive(Debug)]
pub struct CoordLine {
    coords: Vec<Coord>,
}

impl CoordLine {
    pub fn new() -> Self {
        CoordLine {
            coords: vec![Coord::new(0, 0)],
        }
    }

    pub fn start(&self) -> Coord {
        *self.coords.first().expect("empty line")
    }

    pub fn end(&self) -> Coord {
        *self.coords.last().expect("empty line")
    }

    pub fn clear(&mut self, coord: Coord) {
        self.coords.clear();
        self.coords.push(coord);
    }

    pub fn extend(&mut self, coord: Coord) {
        self.coords.push(coord);
    }

    pub fn len(&self) -> usize {
        self.coords.len()
    }

    pub fn get(&self, idx: usize) -> Option<Coord> {
        self.coords.get(idx).map(|c| *c)
    }

    pub fn iter(&self) -> CoordLineIter {
        CoordLineIter(self.coords.iter())
    }
}
