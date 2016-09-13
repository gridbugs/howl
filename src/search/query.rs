use grid::Coord;

pub struct CellInfo<'a, T: 'a> {
    pub value: &'a T,
    pub coord: Coord,
}

impl<'a, T: 'a> CellInfo<'a, T> {
    pub fn new(value: &'a T, coord: Coord) -> Self {
        CellInfo {
            value: value,
            coord: coord,
        }
    }
}

pub enum Destination<T> {
    Predicate(Box<Fn(CellInfo<T>) -> bool>),
    Coord(Coord),
}

impl<T> Destination<T> {
    pub fn matches(&self, info: CellInfo<T>) -> bool {
        match self {
            &Destination::Predicate(ref predicate) => predicate(info),
            &Destination::Coord(coord) => coord == info.coord,
        }
    }
}

pub struct Query<T> {
    pub start: Coord,
    pub end: Destination<T>,
}

impl<T> Query<T> {
    pub fn new(start: Coord, end: Destination<T>) -> Self {
        Query {
            start: start,
            end: end,
        }
    }

    pub fn new_to_coord(start: Coord, end: Coord) -> Self {
        Self::new(start, Destination::Coord(end))
    }

    pub fn new_to_predicate<F>(start: Coord, end: F) -> Self
    where F: 'static + Fn(CellInfo<T>) -> bool
    {
        Self::new(start, Destination::Predicate(Box::new(end)))
    }

    pub fn matches(&self, info: CellInfo<T>) -> bool {
        self.end.matches(info)
    }
}
