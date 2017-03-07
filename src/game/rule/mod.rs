mod types;
pub use self::types::*;

mod collision;
mod realtime_velocity;
mod level_switch;
mod projectile_collision;
mod death;
mod enemy_collision;
mod bump_attack;
mod physics;
mod driving;
mod run_over;
mod bounds;
mod then;
mod acid;

pub mod rules {
    pub use super::collision::*;
    pub use super::realtime_velocity::*;
    pub use super::level_switch::*;
    pub use super::projectile_collision::*;
    pub use super::death::*;
    pub use super::enemy_collision::*;
    pub use super::bump_attack::*;
    pub use super::physics::*;
    pub use super::driving::*;
    pub use super::run_over::*;
    pub use super::bounds::*;
    pub use super::then::*;
    pub use super::acid::*;
}
