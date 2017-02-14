use math::Vector2Index;
use coord::Coord;

#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
struct Octant {
    major_sign: isize,
    minor_sign: isize,
    major_axis: Vector2Index,
    minor_axis: Vector2Index,
}

fn choose_octant(delta: Coord) -> Octant {
    let (major_axis, minor_axis) = if delta.x.abs() > delta.y.abs() {
        (Vector2Index::X, Vector2Index::Y)
    } else {
        (Vector2Index::Y, Vector2Index::X)
    };

    let major_sign = if delta.get(major_axis) < 0 { -1 } else { 1 };
    let minor_sign = if delta.get(minor_axis) < 0 { -1 } else { 1 };

    Octant {
        major_sign: major_sign,
        minor_sign: minor_sign,
        major_axis: major_axis,
        minor_axis: minor_axis,
    }
}

#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
pub struct InfiniteLineState {
    octant: Octant,
    major_delta_abs: usize,
    minor_delta_abs: usize,
    accumulator: isize,
    zero: bool,
}

impl InfiniteLineState {
    pub fn new(delta: Coord, zero: bool) -> Self {
        let octant = choose_octant(delta);
        InfiniteLineState {
            major_delta_abs: delta.get(octant.major_axis).abs() as usize,
            minor_delta_abs: delta.get(octant.minor_axis).abs() as usize,
            accumulator: 0,
            octant: octant,
            zero: zero,
        }
    }

    pub fn step(&mut self) -> Coord {
        let mut coord = Coord::new(0, 0);

        if self.zero {
            self.zero = false;
            return coord;
        }

        // a single step of bresenham's algorithm
        self.accumulator += self.minor_delta_abs as isize;

        coord.set(self.octant.major_axis, self.octant.major_sign);

        if self.accumulator > (self.major_delta_abs as isize) / 2 {
            self.accumulator -= self.major_delta_abs as isize;
            coord.set(self.octant.minor_axis, self.octant.minor_sign);
        }

        coord
    }
}

impl Iterator for InfiniteLineState {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.step())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FiniteLineState {
    infinite_line_state: InfiniteLineState,
    count: usize,
}

impl FiniteLineState {
    pub fn new(delta: Coord, zero: bool) -> Self {
        FiniteLineState {
            infinite_line_state: InfiniteLineState::new(delta, zero),
            count: 0,
        }
    }

    fn step(&mut self) -> Coord {
        self.count += 1;
        self.infinite_line_state.step()
    }

    fn len(&self) -> usize {
        self.infinite_line_state.major_delta_abs
    }

    fn complete(&self) -> bool {
        self.count > self.len()
    }
}

impl Iterator for FiniteLineState {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.complete() {
            None
        } else {
            Some(self.step())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InfiniteAccumulatingLineState {
    infinite_line_state: InfiniteLineState,
    coord: Coord,
}

impl InfiniteAccumulatingLineState {
    pub fn new_from(delta: Coord, coord: Coord, zero: bool) -> Self {
        InfiniteAccumulatingLineState {
            infinite_line_state: InfiniteLineState::new(delta, zero),
            coord: coord,
        }
    }

    pub fn new(delta: Coord, zero: bool) -> Self {
        Self::new_from(delta, Coord::new(0, 0), zero)
    }

    pub fn new_between(start: Coord, end: Coord, zero: bool) -> Self {
        Self::new_from(end - start, start, zero)
    }

    pub fn step(&mut self) -> Coord {
        self.coord += self.infinite_line_state.step();
        self.coord
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FiniteAccumulatingLineState {
    finite_line_state: FiniteLineState,
    coord: Coord,
}

impl FiniteAccumulatingLineState {
    pub fn new_from(delta: Coord, coord: Coord, zero: bool) -> Self {
        FiniteAccumulatingLineState {
            finite_line_state: FiniteLineState::new(delta, zero),
            coord: coord,
        }
    }

    pub fn new(delta: Coord, zero: bool) -> Self {
        Self::new_from(delta, Coord::new(0, 0), zero)
    }

    pub fn new_between(start: Coord, end: Coord, zero: bool) -> Self {
        Self::new_from(end - start, start, zero)
    }
}

impl Iterator for FiniteAccumulatingLineState {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        self.finite_line_state.next().map(|offset| {
            self.coord += offset;
            self.coord
        })
    }
}
