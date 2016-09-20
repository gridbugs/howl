use grid::Coord;
use grid::Grid;

use geometry::direction;

pub struct CoordIter {
    min: Coord,
    max: Coord,
    current: Coord,
}

impl CoordIter {
    pub fn new(min: Coord, max: Coord) -> Self {
        CoordIter {
            min: min,
            max: max,
            current: min,
        }
    }
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y > self.max.y {
            return None;
        }

        let coord = self.current;

        self.current.x += 1;
        if self.current.x > self.max.x {
            self.current.x = self.min.x;
            self.current.y += 1;
        }

        Some(coord)
    }
}

pub struct NeiCoordIter {
    coord: Coord,
    dir_iter: direction::Iter,
}

impl NeiCoordIter {
    pub fn new(coord: Coord) -> Self {
        NeiCoordIter {
            coord: coord,
            dir_iter: direction::iter(),
        }
    }
}

impl Iterator for NeiCoordIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        self.dir_iter.next().map(|dir| self.coord + dir.vector())
    }
}

pub struct NeiIter<'a, G: Grid + 'a> {
    grid: &'a G,
    nei_coord_iter: NeiCoordIter,
}

impl<'a, G: Grid + 'a> NeiIter<'a, G> {
    pub fn new(grid: &'a G, coord: Coord) -> Self {
        NeiIter {
            grid: grid,
            nei_coord_iter: NeiCoordIter::new(coord),
        }
    }
}

impl<'a, G: Grid> Iterator for NeiIter<'a, G> {
    type Item = Option<&'a G::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nei_coord_iter.next().map(|coord| self.grid.get(coord))
    }
}

pub struct SomeNeiIter<'a, G: Grid + 'a>(NeiIter<'a, G>);

impl<'a, G: Grid + 'a> SomeNeiIter<'a, G> {
    pub fn new(grid: &'a G, coord: Coord) -> Self {
        SomeNeiIter(NeiIter::new(grid, coord))
    }
}

impl<'a, G: Grid> Iterator for SomeNeiIter<'a, G> {
    type Item = &'a G::Item;

    fn next(&mut self) -> Option<Self::Item> {

        while let Some(maybe_neighbour) = self.0.next() {
            if maybe_neighbour.is_some() {
                return maybe_neighbour;
            }
        }

        None
    }
}

pub struct SomeNeiCoordIter<'a, G: Grid + 'a> {
    grid: &'a G,
    nei_coord_iter: NeiCoordIter,
}

impl<'a, G: Grid + 'a> SomeNeiCoordIter<'a, G> {
    pub fn new(grid: &'a G, coord: Coord) -> Self {
        SomeNeiCoordIter {
            grid: grid,
            nei_coord_iter: NeiCoordIter::new(coord),
        }
    }
}

impl<'a, G: Grid> Iterator for SomeNeiCoordIter<'a, G> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(coord) = self.nei_coord_iter.next() {
            if self.grid.is_valid_coord(coord) {
                return Some(coord);
            }
        }

        None
    }
}
