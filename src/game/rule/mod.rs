mod types;
pub use self::types::*;

mod collision;
mod open_door;
mod close_door;
pub mod rules {
    pub use super::collision::*;
    pub use super::open_door::*;
    pub use super::close_door::*;
}
