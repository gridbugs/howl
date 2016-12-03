// Velocity in a cardinal or ordinal direction

use game::data::RealtimeSpeed;
use direction::Direction;

pub struct RealtimeAxisVelocity {
    pub speed: RealtimeSpeed,
    pub direction: Direction,
}

impl RealtimeAxisVelocity {
    pub fn from_cells_per_sec(cells_per_sec: f64, direction: Direction) -> Self {
        RealtimeAxisVelocity {
            speed: RealtimeSpeed::from_cells_per_sec(cells_per_sec),
            direction: direction,
        }
    }
}
