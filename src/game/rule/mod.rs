mod types;
pub use self::types::*;

mod collision;
mod open_door;
mod close_door;
mod realtime_axis_velocity;
mod burst_fire;
pub mod rules {
    pub use super::collision::*;
    pub use super::open_door::*;
    pub use super::close_door::*;
    pub use super::realtime_axis_velocity::*;
    pub use super::burst_fire::*;
}
