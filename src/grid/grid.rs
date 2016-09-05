use grid::{
    Coord,
    CoordIter,
    NeiCoordIter,
    NeiIter,
    SomeNeiIter,
    SomeNeiCoordIter,
};

use geometry::Direction;

use std::marker::Sized;

pub trait Grid {

    type Item;

    fn swap(&mut self, other: &mut Self);

    fn get(&self, coord: Coord) -> Option<&Self::Item>;
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Self::Item>;

    fn get_unsafe(&self, coord: Coord) -> &Self::Item;
    fn get_mut_unsafe(&mut self, coord: Coord) -> &mut Self::Item;

    fn get_nei(&self, coord: Coord, direction: Direction) -> Option<&Self::Item> {
        self.get(coord + direction.vector())
    }

    fn limits_min(&self) -> Coord;
    fn limits_max(&self) -> Coord;

    fn x_min(&self) -> isize { self.limits_min().x }
    fn y_min(&self) -> isize { self.limits_min().y }
    fn x_max(&self) -> isize { self.limits_max().x }
    fn y_max(&self) -> isize { self.limits_max().y }

    fn width(&self) -> usize {
        (self.x_max() - self.x_min() + 1) as usize
    }

    fn height(&self) -> usize {
        (self.y_max() - self.y_min() + 1) as usize
    }

    fn is_valid_coord(&self, c: Coord) -> bool {
        c.x >= self.x_min() && c.y >= self.y_min() &&
            c.x <= self.x_max() && c.y <= self.y_max()
    }

    fn is_border_coord(&self, c: Coord) -> bool {
        c.x == self.x_min() || c.y == self.y_min() ||
            c.x == self.x_max() || c.y == self.y_max()
    }

    fn coord_iter(&self) -> CoordIter {
        CoordIter::new(self.limits_min(), self.limits_max())
    }

    fn nei_coord_iter(&self, coord: Coord) -> NeiCoordIter {
        NeiCoordIter::new(coord)
    }

    fn nei_iter<'a>(&'a self, coord: Coord) -> NeiIter<'a, Self>
        where Self: Sized
    {
        NeiIter::new(self, coord)
    }

    fn some_nei_iter<'a>(&'a self, coord: Coord) -> SomeNeiIter<'a, Self>
        where Self: Sized
    {
        SomeNeiIter::new(self, coord)
    }

    fn some_nei_coord_iter<'a>(&'a self, coord: Coord) -> SomeNeiCoordIter<'a, Self>
        where Self: Sized
    {
        SomeNeiCoordIter::new(self, coord)
    }
}

pub trait DefaultGrid : Grid
    where <Self as Grid>::Item: Default,
{
    fn new_default(width: usize, height: usize) -> Self;
    fn reset_all(&mut self);
}

pub trait IterGrid<'a> : Grid
    where <Self as Grid>::Item: 'a,
{
    type Iter: Iterator<Item=&'a Self::Item>;
    type IterMut: Iterator<Item=&'a mut Self::Item>;

    fn iter(&'a self) -> Self::Iter;
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

pub trait RowGrid<'a> : Grid
    where <Self as Grid>::Item: 'a,
{
    type RowIntoIter: IntoIterator<Item=&'a Self::Item> + 'a;
    type RowIter: Iterator<Item=Self::RowIntoIter>;

    type RowIntoIterMut: IntoIterator<Item=&'a mut Self::Item> + 'a;
    type RowIterMut: Iterator<Item=Self::RowIntoIterMut>;

    fn rows(&'a self) -> Self::RowIter;
    fn rows_mut(&'a mut self) -> Self::RowIterMut;
}
