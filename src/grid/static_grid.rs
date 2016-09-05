use std::mem;
use std::slice;
use std::ops::{Index, IndexMut};

use geometry::Vector2;

use grid::{
    Grid,
    DefaultGrid,
    IterGrid,
    RowGrid,
    Coord,
    CoordCell,
};

#[derive(Debug, Clone)]
pub struct StaticGrid<T> {
    pub width: usize,
    pub height: usize,
    limits: Vector2<isize>,
    size: usize,
    elements: Vec<T>,
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

impl<T: Copy> StaticGrid<T> {
    pub fn new_copy(width: usize, height: usize, example: T) -> Self {
        let mut grid = StaticGrid::new_uninitialised(width, height);

        for _ in 0..grid.size {
            grid.elements.push(example);
        }

        grid
    }

    pub fn set_all(&mut self, example: T) {
        for x in self.iter_mut() {
            *x = example;
        }
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

        let size = (width as usize).checked_mul(height as usize)
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

    fn get_unsafe(&self, coord: Coord) -> &Self::Item {
        &self.elements[self.to_index(coord)]
    }

    fn get_mut_unsafe(&mut self, coord: Coord) -> &mut Self::Item {
        let index = self.to_index(coord);
        &mut self.elements[index]
    }

    fn get(&self, coord: Coord) -> Option<&T> {
        if self.is_valid_coord(coord) {
            Some(self.get_unsafe(coord))
        } else {
            None
        }
    }

    fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if self.is_valid_coord(coord) {
            Some(self.get_mut_unsafe(coord))
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

    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
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

impl<'a, T> Index<&'a Coord> for StaticGrid<T> {
    type Output = T;
    fn index<'b>(&'b self, index: &'a Coord) -> &'b T {
        &self.elements[(index.x as usize) + (index.y as usize) * self.width]
    }
}

impl<'a, T> IndexMut<&'a Coord> for StaticGrid<T> {
    fn index_mut<'b>(&'b mut self, index: &'a Coord) -> &'b mut T {
        &mut self.elements[(index.x as usize) + (index.y as usize) * self.width]
    }
}

impl<T> Index<Coord> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, index: Coord) -> &'a T {
        &self.elements[(index.x as usize) + (index.y as usize) * self.width]
    }
}

impl<T> IndexMut<Coord> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, index: Coord) -> &'a mut T {
        &mut self.elements[(index.x as usize) + (index.y as usize) * self.width]
    }
}

impl<'a, T> Index<&'a Vector2<usize>> for StaticGrid<T> {
    type Output = T;
    fn index<'b>(&'b self, index: &'a Vector2<usize>) -> &'b T {
        &self.elements[index.x + index.y * self.width]
    }
}

impl<'a, T> IndexMut<&'a Vector2<usize>> for StaticGrid<T> {
    fn index_mut<'b>(&'b mut self, index: &'a Vector2<usize>) -> &'b mut T {
        &mut self.elements[index.x + index.y * self.width]
    }
}

impl<T> Index<(isize, isize)> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, (x, y): (isize, isize)) -> &'a T {
        &self.elements[(x as usize) + (y as usize) * self.width]
    }
}

impl<T> IndexMut<(isize, isize)> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, (x, y): (isize, isize)) -> &'a mut T {
        &mut self.elements[(x as usize) + (y as usize) * self.width]
    }
}

impl<T> Index<Vector2<usize>> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, index: Vector2<usize>) -> &'a T {
        &self.elements[index.x + index.y * self.width]
    }
}

impl<T> IndexMut<Vector2<usize>> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, index: Vector2<usize>) -> &'a mut T {
        &mut self.elements[index.x + index.y * self.width]
    }
}

impl<T> Index<(usize, usize)> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, (x, y): (usize, usize)) -> &'a T {
        &self.elements[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, (x, y): (usize, usize)) -> &'a mut T {
        &mut self.elements[x + y * self.width]
    }
}
