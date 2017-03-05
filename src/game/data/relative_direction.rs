use direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelativeDirection {
    Front,
    Rear,
    Left,
    Right,
}

impl From<Direction> for RelativeDirection {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => RelativeDirection::Left,
            Direction::South => RelativeDirection::Right,
            Direction::East => RelativeDirection::Front,
            Direction::West => RelativeDirection::Rear,
            _ => panic!("Invalid direction"),
        }
    }
}
