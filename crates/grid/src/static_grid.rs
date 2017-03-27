use std::mem;
use std::slice;

use math::Vector2;
use math::Coord;

use grid::{Grid, DefaultGrid, CopyGrid, IterGrid, RowGrid, CoordIterGrid};
use coord_cell::CoordCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticGrid<T> {
    pub width: usize,
    pub height: usize,
    limits: Vector2<isize>,
    size: usize,
    elements: Vec<T>,
}

pub struct StaticCoordIter {
    min: Coord,
    max: Coord,
    current: Coord,
}

impl StaticCoordIter {
    pub fn new(min: Coord, max: Coord) -> Self {
        StaticCoordIter {
            min: min,
            max: max,
            current: min,
        }
    }
}

impl Iterator for StaticCoordIter {
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

impl<T: CoordCell> StaticGrid<T> {
    pub fn new_coords(width: usize, height: usize, data: T::Data) -> StaticGrid<T> {
        let mut grid = StaticGrid::new_uninitialised(width, height);

        for _ in 0..grid.size {
            grid.elements.push(T::new(0, 0, data));
        }

        grid
    }
}

impl<T: Default> DefaultGrid for StaticGrid<T> {
    fn new_default(width: usize, height: usize) -> Self {
        let mut grid = StaticGrid::new_uninitialised(width, height);

        for _ in 0..grid.size {
            grid.elements.push(T::default());
        }

        grid
    }

    fn reset_all(&mut self) {
        for x in self.iter_mut() {
            *x = T::default();
        }
    }
}

impl<T: Copy> CopyGrid for StaticGrid<T> {
    fn new_copy(width: usize, height: usize, example: T) -> Self {
        let mut grid = StaticGrid::new_uninitialised(width, height);

        for _ in 0..grid.size {
            grid.elements.push(example);
        }

        grid
    }

    fn set_all(&mut self, example: T) {
        for x in self.iter_mut() {
            *x = example;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        self.elements.copy_from_slice(&other.elements);
    }
}

impl<T> StaticGrid<T> {
    pub fn new_call<F>(width: usize, height: usize, mut f: F) -> Self
        where F: FnMut(isize, isize) -> T
    {
        let mut grid = StaticGrid::new_uninitialised(width, height);

        for y in 0..height as isize {
            for x in 0..width as isize {
                grid.elements.push(f(x, y));
            }
        }

        grid
    }
}

impl<T> StaticGrid<T> {
    fn new_uninitialised(width: usize, height: usize) -> Self {

        let size = (width as usize)
            .checked_mul(height as usize)
            .expect("product of width and height overflows");

        StaticGrid {
            width: width,
            height: height,
            limits: Vector2::new(width as isize - 1, height as isize - 1),
            size: size,
            elements: Vec::with_capacity(size),
        }
    }

    fn to_index(&self, coord: Coord) -> usize {
        coord.x as usize + (coord.y as usize) * self.width
    }
}

impl<T> Grid for StaticGrid<T> {
    type Item = T;

    fn swap(&mut self, other: &mut Self) {
        if self.width == other.width && self.height == other.height {
            mem::swap(&mut self.elements, &mut other.elements);
        } else {
            panic!("tried to swap grids with different sizes");
        }
    }

    fn get_checked(&self, coord: Coord) -> &Self::Item {
        &self.elements[self.to_index(coord)]
    }

    fn get_checked_mut(&mut self, coord: Coord) -> &mut Self::Item {
        let index = self.to_index(coord);
        &mut self.elements[index]
    }

    unsafe fn get_unchecked(&self, coord: Coord) -> &Self::Item {
        self.elements.get_unchecked(self.to_index(coord))
    }

    unsafe fn get_unchecked_mut(&mut self, coord: Coord) -> &mut Self::Item {
        let index = self.to_index(coord);
        self.elements.get_unchecked_mut(index)
    }

    fn get(&self, coord: Coord) -> Option<&T> {
        if self.is_valid_coord(coord) {
            // the validity is checked explicitly
            Some(unsafe { self.get_unchecked(coord) })
        } else {
            None
        }
    }

    fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if self.is_valid_coord(coord) {
            // the validity is checked explicitly
            Some(unsafe { self.get_unchecked_mut(coord) })
        } else {
            None
        }
    }

    fn limits_min(&self) -> Coord {
        Coord::new(0, 0)
    }

    fn limits_max(&self) -> Coord {
        self.limits
    }

    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl<T> CoordIterGrid for StaticGrid<T> {
    type CoordIter = StaticCoordIter;

    fn coord_iter(&self) -> Self::CoordIter {
        StaticCoordIter::new(self.limits_min(), self.limits_max())
    }
}

impl<'a, T: 'a> IterGrid<'a> for StaticGrid<T> {
    type Iter = slice::Iter<'a, T>;
    type IterMut = slice::IterMut<'a, T>;

    fn iter(&'a self) -> Self::Iter {
        self.elements.iter()
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.elements.iter_mut()
    }
}

impl<'a, T: 'a> RowGrid<'a> for StaticGrid<T> {
    type RowIntoIter = &'a [T];
    type RowIter = slice::Chunks<'a, T>;

    type RowIntoIterMut = &'a mut [T];
    type RowIterMut = slice::ChunksMut<'a, T>;

    fn rows(&'a self) -> Self::RowIter {
        self.elements.chunks(self.width)
    }

    fn rows_mut(&'a mut self) -> Self::RowIterMut {
        self.elements.chunks_mut(self.width)
    }
}
