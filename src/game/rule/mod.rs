mod types;
pub use self::types::*;

mod collision;
mod door;
pub mod rules {
    pub use super::collision::*;
    pub use super::door::*;
}
