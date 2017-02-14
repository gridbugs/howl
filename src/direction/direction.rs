use math::{Vector2, Vector2Index};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub direction: Direction,
    pub direction_type: DirectionType,
    pub opposite: Direction,
    pub vector: Vector2<isize>,
    pub left90: Direction,
    pub right90: Direction,
    pub multiplier: f64,
}

pub mod directions {

    use math::{Vector2, SQRT2};
    use direction::*;

    pub static NORTH: DirectionProfile = DirectionProfile {
        direction: Direction::North,
        direction_type: DirectionType::Cardinal(CardinalDirection::North),
        opposite: Direction::South,
        vector: Vector2 { x: 0, y: -1 },
        left90: Direction::West,
        right90: Direction::East,
        multiplier: 1.0,
    };

    pub static EAST: DirectionProfile = DirectionProfile {
        direction: Direction::East,
        direction_type: DirectionType::Cardinal(CardinalDirection::East),
        opposite: Direction::West,
        vector: Vector2 { x: 1, y: 0 },
        left90: Direction::North,
        right90: Direction::South,
        multiplier: 1.0,
    };

    pub static SOUTH: DirectionProfile = DirectionProfile {
        direction: Direction::South,
        direction_type: DirectionType::Cardinal(CardinalDirection::South),
        opposite: Direction::North,
        vector: Vector2 { x: 0, y: 1 },
        left90: Direction::East,
        right90: Direction::West,
        multiplier: 1.0,
    };

    pub static WEST: DirectionProfile = DirectionProfile {
        direction: Direction::West,
        direction_type: DirectionType::Cardinal(CardinalDirection::West),
        opposite: Direction::East,
        vector: Vector2 { x: -1, y: 0 },
        left90: Direction::South,
        right90: Direction::North,
        multiplier: 1.0,
    };

    pub static NORTH_EAST: DirectionProfile = DirectionProfile {
        direction: Direction::NorthEast,
        direction_type: DirectionType::Ordinal(OrdinalDirection::NorthEast),
        opposite: Direction::SouthWest,
        vector: Vector2 { x: 1, y: -1 },
        left90: Direction::NorthWest,
        right90: Direction::SouthEast,
        multiplier: SQRT2,
    };

    pub static SOUTH_EAST: DirectionProfile = DirectionProfile {
        direction: Direction::SouthEast,
        direction_type: DirectionType::Ordinal(OrdinalDirection::SouthEast),
        opposite: Direction::NorthWest,
        vector: Vector2 { x: 1, y: 1 },
        left90: Direction::NorthEast,
        right90: Direction::SouthWest,
        multiplier: SQRT2,
    };

    pub static SOUTH_WEST: DirectionProfile = DirectionProfile {
        direction: Direction::SouthWest,
        direction_type: DirectionType::Ordinal(OrdinalDirection::SouthWest),
        opposite: Direction::NorthEast,
        vector: Vector2 { x: -1, y: 1 },
        left90: Direction::SouthEast,
        right90: Direction::NorthWest,
        multiplier: SQRT2,
    };

    pub static NORTH_WEST: DirectionProfile = DirectionProfile {
        direction: Direction::NorthEast,
        direction_type: DirectionType::Ordinal(OrdinalDirection::NorthWest),
        opposite: Direction::SouthEast,
        vector: Vector2 { x: -1, y: -1 },
        left90: Direction::SouthWest,
        right90: Direction::NorthEast,
        multiplier: SQRT2,
    };
}

#[derive(Copy, Clone, Debug)]
pub struct CardinalDirectionProfile {
    pub vector2_index: Vector2Index,
    pub opposite: CardinalDirection,
}

pub mod cardinal_directions {
    use math::Vector2Index;

    use direction::{CardinalDirectionProfile, CardinalDirection};

    pub static NORTH: CardinalDirectionProfile = CardinalDirectionProfile {
        vector2_index: Vector2Index::Y,
        opposite: CardinalDirection::South,
    };
    pub static EAST: CardinalDirectionProfile = CardinalDirectionProfile {
        vector2_index: Vector2Index::X,
        opposite: CardinalDirection::West,
    };
    pub static SOUTH: CardinalDirectionProfile = CardinalDirectionProfile {
        vector2_index: Vector2Index::Y,
        opposite: CardinalDirection::North,
    };
    pub static WEST: CardinalDirectionProfile = CardinalDirectionProfile {
        vector2_index: Vector2Index::X,
        opposite: CardinalDirection::East,
    };
}

#[derive(Copy, Clone, Debug)]
pub struct OrdinalDirectionProfile {
    pub corner_offset: Vector2<f64>,
    pub opposite: OrdinalDirection,
}

pub mod ordinal_directions {
    use math::Vector2;

    use direction::{OrdinalDirectionProfile, OrdinalDirection};

    pub static NORTH_EAST: OrdinalDirectionProfile = OrdinalDirectionProfile {
        corner_offset: Vector2 { x: 1.0, y: 0.0 },
        opposite: OrdinalDirection::SouthWest,
    };
    pub static SOUTH_EAST: OrdinalDirectionProfile = OrdinalDirectionProfile {
        corner_offset: Vector2 { x: 1.0, y: 1.0 },
        opposite: OrdinalDirection::NorthWest,
    };
    pub static SOUTH_WEST: OrdinalDirectionProfile = OrdinalDirectionProfile {
        corner_offset: Vector2 { x: 0.0, y: 1.0 },
        opposite: OrdinalDirection::NorthEast,
    };
    pub static NORTH_WEST: OrdinalDirectionProfile = OrdinalDirectionProfile {
        corner_offset: Vector2 { x: 0.0, y: 0.0 },
        opposite: OrdinalDirection::SouthEast,
    };
}

pub const NUM_DIRECTIONS: usize = 8;
pub static DIRECTIONS: [Direction; NUM_DIRECTIONS] = [Direction::North,
                                                      Direction::NorthEast,
                                                      Direction::East,
                                                      Direction::SouthEast,
                                                      Direction::South,
                                                      Direction::SouthWest,
                                                      Direction::West,
                                                      Direction::NorthWest];

pub const NUM_CARDINAL_DIRECTIONS: usize = 4;
pub static CARDINAL_DIRECTIONS: [Direction; NUM_CARDINAL_DIRECTIONS] =
    [Direction::North,
     Direction::East,
     Direction::South,
     Direction::West];

pub static CARDINALS: [CardinalDirection; NUM_CARDINAL_DIRECTIONS] =
    [CardinalDirection::North,
     CardinalDirection::East,
     CardinalDirection::South,
     CardinalDirection::West];

pub const NUM_ORDINAL_DIRECTIONS: usize = 4;
pub static ORDINAL_DIRECTIONS: [Direction; NUM_ORDINAL_DIRECTIONS] =
    [Direction::NorthEast,
     Direction::SouthEast,
     Direction::SouthWest,
     Direction::NorthWest];

pub static ORDINALS: [OrdinalDirection; NUM_ORDINAL_DIRECTIONS] =
    [OrdinalDirection::NorthEast,
     OrdinalDirection::SouthEast,
     OrdinalDirection::SouthWest,
     OrdinalDirection::NorthWest];

pub static DIRECTION_PROFILES: [&'static DirectionProfile; NUM_DIRECTIONS] =
    [&directions::NORTH,
     &directions::NORTH_EAST,
     &directions::EAST,
     &directions::SOUTH_EAST,
     &directions::SOUTH,
     &directions::SOUTH_WEST,
     &directions::WEST,
     &directions::NORTH_WEST];

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
    pub fn vector(self) -> Vector2<isize> {
        self.profile().vector
    }
    pub fn left90(self) -> Direction {
        self.profile().left90
    }
    pub fn right90(self) -> Direction {
        self.profile().right90
    }
    pub fn multiplier(self) -> f64 {
        self.profile().multiplier
    }
    pub fn sub_direction(self) -> DirectionType {
        self.profile().direction_type
    }
}

pub static CARDINAL_DIRECTION_PROFILES:
[&'static CardinalDirectionProfile; NUM_CARDINAL_DIRECTIONS] = [
    &cardinal_directions::NORTH,
    &cardinal_directions::EAST,
    &cardinal_directions::SOUTH,
    &cardinal_directions::WEST,
];

pub static CARDINAL_DIRECTION_COMBINATIONS:
[[Option<OrdinalDirection>; NUM_CARDINAL_DIRECTIONS]; NUM_CARDINAL_DIRECTIONS] = [
    // North
    [None, Some(OrdinalDirection::NorthEast), None, Some(OrdinalDirection::NorthWest)],
    // East
    [Some(OrdinalDirection::NorthEast), None, Some(OrdinalDirection::SouthEast), None],
    // South
    [None, Some(OrdinalDirection::SouthEast), None, Some(OrdinalDirection::SouthWest)],
    // West
    [Some(OrdinalDirection::NorthWest), None, Some(OrdinalDirection::SouthWest), None],
];

impl CardinalDirection {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn profile(self) -> &'static CardinalDirectionProfile {
        CARDINAL_DIRECTION_PROFILES[self.index()]
    }

    pub fn vector2_index(self) -> Vector2Index {
        self.profile().vector2_index
    }

    pub fn combine(self, other: CardinalDirection) -> Option<OrdinalDirection> {
        CARDINAL_DIRECTION_COMBINATIONS[self.index()][other.index()]
    }

    pub fn opposite(self) -> CardinalDirection {
        self.profile().opposite
    }
}

pub static ORDINAL_DIRECTION_PROFILES:
[&'static OrdinalDirectionProfile; NUM_ORDINAL_DIRECTIONS] = [
    &ordinal_directions::NORTH_EAST,
    &ordinal_directions::SOUTH_EAST,
    &ordinal_directions::SOUTH_WEST,
    &ordinal_directions::NORTH_WEST,
];

impl OrdinalDirection {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn profile(self) -> &'static OrdinalDirectionProfile {
        ORDINAL_DIRECTION_PROFILES[self.index()]
    }

    pub fn corner_offset(self) -> Vector2<f64> {
        self.profile().corner_offset
    }

    pub fn opposite(self) -> OrdinalDirection {
        self.profile().opposite
    }
}

pub trait SubDirection: Sized {
    fn direction(self) -> Direction;

    fn profile(self) -> &'static DirectionProfile {
        self.direction().profile()
    }
    fn opposite(self) -> Direction {
        self.profile().opposite
    }
    fn vector(self) -> Vector2<isize> {
        self.profile().vector
    }
    fn left90(self) -> Direction {
        self.profile().left90
    }
    fn right90(self) -> Direction {
        self.profile().right90
    }
    fn multiplier(self) -> f64 {
        self.profile().multiplier
    }
}

impl SubDirection for CardinalDirection {
    fn direction(self) -> Direction {
        match self {
            CardinalDirection::North => Direction::North,
            CardinalDirection::East => Direction::East,
            CardinalDirection::South => Direction::South,
            CardinalDirection::West => Direction::West,
        }
    }
}

impl SubDirection for OrdinalDirection {
    fn direction(self) -> Direction {
        match self {
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

pub struct CardinalDirectionIter(usize);
impl Iterator for CardinalDirectionIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        iter_helper(&mut self.0, NUM_CARDINAL_DIRECTIONS, &CARDINAL_DIRECTIONS)
    }
}

pub struct CardinalIter(usize);
impl Iterator for CardinalIter {
    type Item = CardinalDirection;
    fn next(&mut self) -> Option<Self::Item> {
        iter_helper(&mut self.0, NUM_CARDINAL_DIRECTIONS, &CARDINALS)
    }
}

pub struct OrdinalDirectionIter(usize);
impl Iterator for OrdinalDirectionIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        iter_helper(&mut self.0, NUM_ORDINAL_DIRECTIONS, &ORDINAL_DIRECTIONS)
    }
}

pub struct OrdinalIter(usize);
impl Iterator for OrdinalIter {
    type Item = OrdinalDirection;
    fn next(&mut self) -> Option<Self::Item> {
        iter_helper(&mut self.0, NUM_ORDINAL_DIRECTIONS, &ORDINALS)
    }
}

pub fn iter() -> Iter {
    Iter(0)
}
pub fn cardinal_direction_iter() -> CardinalDirectionIter {
    CardinalDirectionIter(0)
}
pub fn cardinal_iter() -> CardinalIter {
    CardinalIter(0)
}
pub fn ordinal_direction_iter() -> OrdinalDirectionIter {
    OrdinalDirectionIter(0)
}
pub fn ordinal_iter() -> OrdinalIter {
    OrdinalIter(0)
}
