use geometry::vector2::Vector2;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Copy, Clone, Debug)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug)]
pub enum OrdinalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

#[derive(Copy, Clone, Debug)]
pub enum DirectionType {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection),
}

#[derive(Copy, Clone, Debug)]
pub struct DirectionProfile {
    pub direction : Direction,
    pub direction_type : DirectionType,
    pub opposite : Direction,
    pub vector : Vector2<i8>,
}

pub mod directions {

    use geometry::vector2::Vector2;

    use geometry::direction::DirectionProfile;
    use geometry::direction::Direction;
    use geometry::direction::CardinalDirection;
    use geometry::direction::OrdinalDirection;
    use geometry::direction::DirectionType;

    pub static NORTH : DirectionProfile = DirectionProfile {
        direction: Direction::North,
        direction_type: DirectionType::Cardinal(CardinalDirection::North),
        opposite: Direction::South,
        vector: Vector2 { x: 0, y: -1 },
    };

    pub static EAST : DirectionProfile = DirectionProfile {
        direction: Direction::East,
        direction_type: DirectionType::Cardinal(CardinalDirection::East),
        opposite: Direction::West,
        vector: Vector2 { x: 1, y: 0 },
    };

    pub static SOUTH : DirectionProfile = DirectionProfile {
        direction: Direction::South,
        direction_type: DirectionType::Cardinal(CardinalDirection::South),
        opposite: Direction::North,
        vector: Vector2 { x: 0, y: 1 },
    };

    pub static WEST : DirectionProfile = DirectionProfile {
        direction: Direction::West,
        direction_type: DirectionType::Cardinal(CardinalDirection::West),
        opposite: Direction::East,
        vector: Vector2 { x: -1, y: 0 },
    };

    pub static NORTH_EAST : DirectionProfile = DirectionProfile {
        direction: Direction::NorthEast,
        direction_type: DirectionType::Ordinal(OrdinalDirection::NorthEast),
        opposite: Direction::SouthWest,
        vector: Vector2 { x: 1, y: -1 },
    };

    pub static SOUTH_EAST : DirectionProfile = DirectionProfile {
        direction: Direction::SouthEast,
        direction_type: DirectionType::Ordinal(OrdinalDirection::SouthEast),
        opposite: Direction::NorthWest,
        vector: Vector2 { x: 1, y: 1 },
    };

    pub static SOUTH_WEST : DirectionProfile = DirectionProfile {
        direction: Direction::SouthWest,
        direction_type: DirectionType::Ordinal(OrdinalDirection::SouthWest),
        opposite: Direction::NorthEast,
        vector: Vector2 { x: -1, y: 1 },
    };

    pub static NORTH_WEST : DirectionProfile = DirectionProfile {
        direction: Direction::NorthEast,
        direction_type: DirectionType::Ordinal(OrdinalDirection::NorthWest),
        opposite: Direction::SouthEast,
        vector: Vector2 { x: -1, y: -1 },
    };
}

pub const NUM_DIRECTIONS : usize = 8;
pub static DIRECTIONS : [Direction; NUM_DIRECTIONS] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

pub const NUM_CARDINAL_DIRECTIONS : usize = 4;
pub static CARDINAL_DIRECTIONS : [Direction; NUM_CARDINAL_DIRECTIONS] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub const NUM_ORDINAL_DIRECTIONS : usize = 4;
pub static ORDINAL_DIRECTIONS : [Direction; NUM_ORDINAL_DIRECTIONS] = [
    Direction::NorthEast,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::NorthWest,
];

pub static DIRECTION_PROFILES : [&'static DirectionProfile; NUM_DIRECTIONS] = [
    &directions::NORTH,
    &directions::NORTH_EAST,
    &directions::EAST,
    &directions::SOUTH_EAST,
    &directions::SOUTH,
    &directions::SOUTH_WEST,
    &directions::WEST,
    &directions::NORTH_WEST,
];

impl Direction {
    pub fn index(self) -> usize {
        self as usize
    }
    pub fn profile(self) -> &'static DirectionProfile {
        DIRECTION_PROFILES[self.index()]
    }
    pub fn sub_index(self) -> usize {
        match self.profile().direction_type {
            DirectionType::Cardinal(index) => index as usize,
            DirectionType::Ordinal(index) => index as usize,
        }
    }
    pub fn opposite(self) -> Direction {
        self.profile().opposite
    }
    pub fn vector(self) -> Vector2<i8> {
        self.profile().vector
    }
}

pub trait SubDirection {
    fn direction(&self) -> Direction;

    fn profile(&self) -> &'static DirectionProfile {
        self.direction().profile()
    }
}

impl SubDirection for CardinalDirection {
    fn direction(&self) -> Direction {
        match *self {
            CardinalDirection::North => Direction::North,
            CardinalDirection::East => Direction::East,
            CardinalDirection::South => Direction::South,
            CardinalDirection::West => Direction::West,
        }
    }
}

impl SubDirection for OrdinalDirection {
    fn direction(&self) -> Direction {
        match *self {
            OrdinalDirection::NorthEast => Direction::NorthEast,
            OrdinalDirection::SouthEast => Direction::SouthEast,
            OrdinalDirection::SouthWest => Direction::SouthWest,
            OrdinalDirection::NorthWest => Direction::NorthWest,
        }
    }
}

fn iter_helper<Item: Copy>(index: &mut usize, size: usize, array: &[Item]) -> Option<Item> {
     if *index < size {
        let result = array[*index];
        *index += 1;
        Some(result)
    } else {
        None
    }
}

pub struct Iter(usize);
impl Iterator for Iter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        iter_helper(&mut self.0, NUM_DIRECTIONS, &DIRECTIONS)
    }
}

pub struct CardinalIter(usize);
impl Iterator for CardinalIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        iter_helper(&mut self.0, NUM_CARDINAL_DIRECTIONS, &CARDINAL_DIRECTIONS)
    }
}

pub struct OrdinalIter(usize);
impl Iterator for OrdinalIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        iter_helper(&mut self.0, NUM_ORDINAL_DIRECTIONS, &ORDINAL_DIRECTIONS)
    }
}

pub fn iter() -> Iter { Iter(0) }
pub fn cardinal_iter() -> CardinalIter { CardinalIter(0) }
pub fn ordinal_iter() -> OrdinalIter { OrdinalIter(0) }
