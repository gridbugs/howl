use std::mem;
use std::cmp;

use math::Coord;
use grid::Grid;
use bidirectional_list::BidirectionalList;

pub struct DynamicGrid<T: Default> {
    elements: BidirectionalList<BidirectionalList<T>>,
    limits_min: Coord,
    limits_max: Coord,
}

impl<T: Default> DynamicGrid<T> {
    pub fn new() -> Self {
        DynamicGrid {
            elements: BidirectionalList::new(),
            limits_min: Coord::new(0, 0),
            limits_max: Coord::new(0, 0),
        }
    }

    pub fn get_mut_with_default(&mut self, coord: Coord) -> &mut T {
        self.limits_max.x = cmp::max(self.limits_max.x, coord.x);
        self.limits_max.y = cmp::max(self.limits_max.y, coord.y);
        self.limits_min.x = cmp::min(self.limits_min.x, coord.x);
        self.limits_min.y = cmp::min(self.limits_min.y, coord.y);
        self.elements.get_mut_with_default(coord.y).get_mut_with_default(coord.x)
    }

    pub fn get_with_default(&self, coord: Coord) -> &T {
        self.elements.get_with_default(coord.y).get_with_default(coord.x)
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.limits_min = Coord::new(0, 0);
        self.limits_max = Coord::new(0, 0);
    }
}

impl<T: Default> Grid for DynamicGrid<T> {
    type Item = T;

    fn swap(&mut self, other: &mut Self) {
        mem::swap(self, other);
    }

    fn get(&self, coord: Coord) -> Option<&Self::Item> {
        self.elements.get(coord.y).and_then(|v| v.get(coord.x))
    }

    fn get_mut(&mut self, coord: Coord) -> Option<&mut Self::Item> {
        self.elements.get_mut(coord.y).and_then(|v| v.get_mut(coord.x))
    }

    fn get_checked(&self, coord: Coord) -> &Self::Item {
        self.elements.get_checked(coord.y).get_checked(coord.x)
    }

    fn get_checked_mut(&mut self, coord: Coord) -> &mut Self::Item {
        self.elements.get_checked_mut(coord.y).get_checked_mut(coord.x)
    }

    unsafe fn get_unchecked(&self, coord: Coord) -> &Self::Item {
        self.elements.get_unchecked(coord.y).get_unchecked(coord.x)
    }

    unsafe fn get_unchecked_mut(&mut self, coord: Coord) -> &mut Self::Item {
        self.elements.get_unchecked_mut(coord.y).get_unchecked_mut(coord.x)
    }

    fn limits_min(&self) -> Coord {
        self.limits_min
    }

    fn limits_max(&self) -> Coord {
        self.limits_max
    }
}
