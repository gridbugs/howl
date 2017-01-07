use coord::*;

pub type StraightLineIter = FiniteAccumulatingLineState;
pub type StraightLineInfiniteIter = InfiniteAccumulatingLineState;

#[derive(Debug, Clone, Copy)]
pub struct StraightLine {
    start: Coord,
    end: Coord,
}

impl StraightLine {
    pub fn new(start: Coord, end: Coord) -> Self {
        StraightLine {
            start: start,
            end: end,
        }
    }

    pub fn new_point(point: Coord) -> Self {
        Self::new(point, point)
    }

    pub fn new_zero() -> Self {
        Self::new_point(Coord::new(0, 0))
    }

    pub fn start(&self) -> Coord {
        self.start
    }

    pub fn end(&self) -> Coord {
        self.end
    }

    pub fn set_start(&mut self, start: Coord) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: Coord) {
        self.end = end;
    }

    pub fn real_len(&self) -> f64 {
        self.end.real_distance(self.start)
    }

    pub fn manhatten_len(&self) -> usize {
        self.end.manhatten_distance(self.start)
    }

    pub fn square_len(&self) -> usize {
        self.end.square_distance(self.start)
    }

    pub fn iter(&self) -> StraightLineIter {
        StraightLineIter::new_between(self.start, self.end, true)
    }

    pub fn infinite_iter(&self) -> StraightLineInfiniteIter {
        StraightLineInfiniteIter::new_between(self.start, self.end, true)
    }
}

impl Default for StraightLine {
    fn default() -> Self {
        StraightLine::new_zero()
    }
}
