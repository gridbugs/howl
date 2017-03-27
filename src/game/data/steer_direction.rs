use math::Direction;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SteerDirection {
    Up,
    Down,
}

impl From<SteerDirection> for Direction {
    fn from(s: SteerDirection) -> Self {
        match s {
            SteerDirection::Up => Direction::North,
            SteerDirection::Down => Direction::South,
        }
    }
}
