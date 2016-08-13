use std::mem;
use std::slice;
use std::ops::{Index, IndexMut};

use geometry::direction;
use geometry::Vector2;
use grid::coord::Coord;
use grid::coord_cell::CoordCell;

#[derive(Debug)]
pub struct StaticGrid<T> {
    pub width: isize,
    pub height: isize,
    limits: Vector2<isize>,
    size: usize,
    elements: Vec<T>,
}

impl<T: Clone> Clone for StaticGrid<T> {
    fn clone(&self) -> Self {
        StaticGrid {
            width: self.width,
            height: self.height,
            limits: self.limits.clone(),
            size: self.size,
            elements: self.elements.clone(),
        }
    }
}

pub struct CoordIter {
    width: isize,
    height: isize,
    coord: Coord,
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.height {
            return None;
        }

        let coord = self.coord;
        self.coord.wrapping_increment_in_place(self.width);

        Some(coord)
    }
}

pub struct NeiCoordIter {
    coord: Coord,
    dir_iter: direction::Iter,
}

impl Iterator for NeiCoordIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        self.dir_iter.next().map(|dir| {
            self.coord + dir.vector().convert::<isize>()
        })
    }
}

pub struct NeiIter<'a, T: 'a> {
    grid: &'a StaticGrid<T>,
    nei_coord_iter: NeiCoordIter,
}

impl<'a, T> Iterator for NeiIter<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nei_coord_iter.next().map(|coord| {
            self.grid.get(coord)
        })
    }
}

pub struct SomeNeiIter<'a, T: 'a>(NeiIter<'a, T>);

impl<'a, T> Iterator for SomeNeiIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        while let Some(maybe_neighbour) = self.0.next() {
            if maybe_neighbour.is_some() {
                return maybe_neighbour;
            }
        }

        None
    }
}

pub struct SomeNeiCoordIter<'a, T: 'a> {
    grid: &'a StaticGrid<T>,
    nei_coord_iter: NeiCoordIter,
}

impl<'a, T> Iterator for SomeNeiCoordIter<'a, T> {
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




impl<T: CoordCell> StaticGrid<T> {
    pub fn new_coords(width: usize, height: usize, data: T::Data) -> StaticGrid<T> {
        let mut grid = StaticGrid::new_uninitialised(width, height);

        for _ in 0..grid.size {
            grid.elements.push(T::new(0, 0, data));
        }

        grid
    }
}

impl<T: Default> StaticGrid<T> {
    pub fn new_default(width: usize, height: usize) -> StaticGrid<T> {
        let mut grid = StaticGrid::new_uninitialised(width, height);

        for _ in 0..grid.size {
            grid.elements.push(T::default());
        }

        grid
    }

    pub fn reset_all(&mut self) {
        for x in self.iter_mut() {
            *x = T::default();
        }
    }
}

impl<T: Copy> StaticGrid<T> {
    pub fn new_copy(width: usize, height: usize, example: T) -> StaticGrid<T> {
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
    fn new_uninitialised(width: usize, height: usize) -> StaticGrid<T> {

        let size = (width as usize).checked_mul(height as usize)
            .expect("product of width and height overflows");

        StaticGrid {
            width: width as isize,
            height: height as isize,
            limits: Vector2::new(width as isize - 1, height as isize - 1),
            size: size,
            elements: Vec::with_capacity(size),
        }
    }

    pub fn swap(&mut self, other: &mut StaticGrid<T>) {
        if self.width == other.width && self.height == other.height {
            mem::swap(&mut self.elements, &mut other.elements);
        } else {
            panic!("tried to swap grids with different sizes");
        }
    }

    pub fn is_valid_coord(&self, Coord {x, y}: Coord) -> bool {
        x < self.width && y < self.height && x >= 0 && y >= 0
    }

    pub fn is_boorder_coord(&self, Coord {x, y}: Coord) -> bool {
        x == 0 || y == 0 || x == self.limits.x || y == self.limits.y
    }

    fn to_index(&self, coord: Coord) -> Option<usize> {
        if self.is_valid_coord(coord) {
            Some((coord.x + coord.y * self.width) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.to_index(coord).map(|index| { &self.elements[index] })
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.to_index(coord).map(move |index| { &mut self.elements[index] })
    }

    pub fn rows(&self) -> slice::Chunks<T> {
        self.elements.chunks(self.width as usize)
    }

    pub fn rows_mut(&mut self) -> slice::ChunksMut<T> {
        self.elements.chunks_mut(self.width as usize)
    }

    pub fn coord_iter(&self) -> CoordIter {
        CoordIter {
            width: self.width,
            height: self.height,
            coord: Coord::new(0, 0),
        }
    }

    pub fn nei_coord_iter(&self, coord: Coord) -> NeiCoordIter {
        NeiCoordIter {
            coord: coord,
            dir_iter: direction::iter(),
        }
    }

    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> { self.elements.iter() }
    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> { self.elements.iter_mut() }

    pub fn nei_iter<'a>(&'a self, coord: Coord) -> NeiIter<'a, T> {
        NeiIter {
            grid: self,
            nei_coord_iter: self.nei_coord_iter(coord),
        }
    }

    pub fn some_nei_iter<'a>(&'a self, coord: Coord) -> SomeNeiIter<'a, T> {
        SomeNeiIter(self.nei_iter(coord))
    }

    pub fn some_nei_coord_iter<'a>(&'a self, coord: Coord) -> SomeNeiCoordIter<'a, T> {
        SomeNeiCoordIter {
            grid: self,
            nei_coord_iter: self.nei_coord_iter(coord),
        }
    }

    pub fn flood_fill_region_coord_all<'a, P>(&'a self, predicate: P)
                                      -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        self.flood_fill_region_coord(&direction::DIRECTIONS, predicate)
    }

    pub fn flood_fill_region_coord_cardinal<'a, P>(&'a self, predicate: P)
                                      -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        self.flood_fill_region_coord(&direction::CARDINAL_DIRECTIONS, predicate)
    }

    pub fn flood_fill_region_coord_ordinal<'a, P>(&'a self, predicate: P)
                                      -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        self.flood_fill_region_coord(&direction::ORDINAL_DIRECTIONS, predicate)
    }

    pub fn flood_fill_region_coord<P>(&self, directions: &[direction::Direction], mut predicate: P)
                             -> Vec<Vec<Coord>>
        where P: FnMut(&T) -> bool
    {
        let mut regions = Vec::<Vec<Coord>>::new();

        let mut visited = StaticGrid::<bool>::new_copy(self.width as usize, self.height as usize, false);
        let mut to_visit = Vec::<Coord>::new();

        for (coord, data) in izip!(
            self.coord_iter(),
            self.iter())
        {
            if !visited[coord] && predicate(data) {
                regions.push(self.flood_fill_helper(|d| {predicate(d)}, coord, &mut visited,
                                               &mut to_visit, &directions));
            }
        }

        regions
    }

    fn flood_fill_helper<P>(&self, mut predicate: P,
                               start_coord: Coord,
                               visited: &mut StaticGrid<bool>,
                               to_visit: &mut Vec<Coord>,
                               directions: &[direction::Direction]) -> Vec<Coord>
        where P: FnMut(&T) -> bool
    {
        let mut region = Vec::<Coord>::new();

        assert!(to_visit.is_empty());
        assert!(!visited[start_coord]);
        assert!(predicate(&self[start_coord]));
        to_visit.push(start_coord);
        visited[start_coord] = true;

        while !to_visit.is_empty() {
            let current_coord = to_visit.pop().unwrap();
            region.push(current_coord);

            for direction in directions {
                let next_coord = current_coord + direction.vector().convert::<isize>();
                if self.is_valid_coord(next_coord) &&
                    !visited[next_coord] &&
                    predicate(&self[next_coord])
                {
                    visited[next_coord] = true;
                    to_visit.push(next_coord);
                }
            }
        }

        region
    }
}

impl<'a, T> Index<&'a Vector2<isize>> for StaticGrid<T> {
    type Output = T;
    fn index<'b>(&'b self, index: &'a Vector2<isize>) -> &'b T {
        self.get(*index).unwrap()
    }
}

impl<'a, T> IndexMut<&'a Vector2<isize>> for StaticGrid<T> {
    fn index_mut<'b>(&'b mut self, index: &'a Vector2<isize>) -> &'b mut T {
        self.get_mut(*index).unwrap()
    }
}

impl<T> Index<Vector2<isize>> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, index: Vector2<isize>) -> &'a T {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<Vector2<isize>> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, index: Vector2<isize>) -> &'a mut T {
        self.get_mut(index).unwrap()
    }
}

impl<'a, T> Index<&'a Vector2<usize>> for StaticGrid<T> {
    type Output = T;
    fn index<'b>(&'b self, index: &'a Vector2<usize>) -> &'b T {
        self.get(Vector2::new(index.x as isize, index.y as isize)).unwrap()
    }
}

impl<'a, T> IndexMut<&'a Vector2<usize>> for StaticGrid<T> {
    fn index_mut<'b>(&'b mut self, index: &'a Vector2<usize>) -> &'b mut T {
        self.get_mut(Vector2::new(index.x as isize, index.y as isize)).unwrap()
    }
}

impl<T> Index<Vector2<usize>> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, index: Vector2<usize>) -> &'a T {
        self.get(Vector2::new(index.x as isize, index.y as isize)).unwrap()
    }
}

impl<T> IndexMut<Vector2<usize>> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, index: Vector2<usize>) -> &'a mut T {
        self.get_mut(Vector2::new(index.x as isize, index.y as isize)).unwrap()
    }
}


impl<T> Index<(isize, isize)> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, (x, y): (isize, isize)) -> &'a T {
        &self[Coord { x: x, y: y }]
    }
}

impl<T> IndexMut<(isize, isize)> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, (x, y): (isize, isize)) -> &'a mut T {
        &mut self[Coord { x: x, y: y }]
    }
}


impl<T> Index<(usize, usize)> for StaticGrid<T> {
    type Output = T;
    fn index<'a>(&'a self, (x, y): (usize, usize)) -> &'a T {
        &self[Coord { x: x as isize, y: y as isize }]
    }
}

impl<T> IndexMut<(usize, usize)> for StaticGrid<T> {
    fn index_mut<'a>(&'a mut self, (x, y): (usize, usize)) -> &'a mut T {
        &mut self[Coord { x: x as isize, y: y as isize}]
    }
}
