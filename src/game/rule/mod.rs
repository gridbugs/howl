mod types;
pub use self::types::*;

mod collision;
mod open_door;
mod close_door;
mod moon_transform;
mod realtime_velocity;
mod level_switch;
mod projectile_collision;
mod death;
mod enemy_collision;
mod bump_attack;

pub mod rules {
    pub use super::collision::*;
    pub use super::open_door::*;
    pub use super::close_door::*;
    pub use super::moon_transform::*;
    pub use super::realtime_velocity::*;
    pub use super::level_switch::*;
    pub use super::projectile_collision::*;
    pub use super::death::*;
    pub use super::enemy_collision::*;
    pub use super::bump_attack::*;
}
