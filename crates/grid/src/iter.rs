use math::Coord;
use math::{direction_iter, DirectionIter};

use grid::Grid;

pub struct NeiCoordIter {
    coord: Coord,
    dir_iter: DirectionIter,
}

impl NeiCoordIter {
    pub fn new(coord: Coord) -> Self {
        NeiCoordIter {
            coord: coord,
            dir_iter: direction_iter(),
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
