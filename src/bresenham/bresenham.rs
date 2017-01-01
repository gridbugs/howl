use math::{Vector2Index, Coord, CoordLine};

#[derive(Debug)]
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

pub struct InfiniteLineState {
    octant: Octant,
    major_delta_abs: usize,
    minor_delta_abs: usize,
    accumulator: isize,
}

impl InfiniteLineState {
    pub fn new(delta: Coord) -> Self {
        let octant = choose_octant(delta);
        InfiniteLineState {
            major_delta_abs: delta.get(octant.major_axis).abs() as usize,
            minor_delta_abs: delta.get(octant.minor_axis).abs() as usize,
            accumulator: 0,
            octant: octant,
        }
    }

    pub fn step(&mut self) -> Coord {
        self.accumulator += self.minor_delta_abs as isize;

        let mut coord = Coord::new(0, 0);
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

pub struct FiniteLineState {
    infinite_line_state: InfiniteLineState,
    count: usize,
}

impl FiniteLineState {
    pub fn new(delta: Coord) -> Self {
        FiniteLineState {
            infinite_line_state: InfiniteLineState::new(delta),
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
        self.count >= self.len()
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

pub struct InfiniteAccumulatingLineState {
    infinite_line_state: InfiniteLineState,
    coord: Coord,
}

impl InfiniteAccumulatingLineState {
    pub fn new_from(delta: Coord, coord: Coord) -> Self {
        InfiniteAccumulatingLineState {
            infinite_line_state: InfiniteLineState::new(delta),
            coord: coord,
        }
    }

    pub fn new(delta: Coord) -> Self {
        Self::new_from(delta, Coord::new(0, 0))
    }

    pub fn step(&mut self) -> Coord {
        self.coord += self.infinite_line_state.step();
        self.coord
    }
}

pub struct FiniteAccumulatingLineState {
    finite_line_state: FiniteLineState,
    coord: Coord,
}

impl FiniteAccumulatingLineState {
    pub fn new_from(delta: Coord, coord: Coord) -> Self {
        FiniteAccumulatingLineState {
            finite_line_state: FiniteLineState::new(delta),
            coord: coord,
        }
    }

    pub fn new(delta: Coord) -> Self {
        Self::new_from(delta, Coord::new(0, 0))
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

pub fn make_line(src: Coord, dst: Coord, line: &mut CoordLine) {

    line.clear(src);

    if src == dst {
        return;
    }

    let delta = dst - src;
    for coord in FiniteAccumulatingLineState::new_from(delta, src) {
        line.extend(coord);
    }
}
