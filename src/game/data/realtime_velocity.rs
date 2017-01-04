use game::data::*;
use coord::{InfiniteLineState, Coord};

#[derive(Clone, Copy, Debug)]
pub struct RealtimeVelocity {
    line_state: InfiniteLineState,
    speed: RealtimeSpeed,
}

impl RealtimeVelocity {
    pub fn new(delta: Coord, cells_per_sec: f64) -> Self {
        RealtimeVelocity {
            line_state: InfiniteLineState::new(delta, false),
            speed: RealtimeSpeed::from_cells_per_sec(cells_per_sec),
        }
    }

    pub fn step_in_place(&mut self) -> Coord {
        self.line_state.step()
    }

    pub fn step(mut self) -> (Self, Coord) {
        let change = self.line_state.step();
        (self, change)
    }

    pub fn ms_per_cell(&self) -> u64 {
        self.speed.ms_per_cell()
    }
}
