mod vector;
mod vector2;
mod vector3;

pub use self::vector::{
    Dot,
    Length,
};
pub use self::vector2::{
    Vector2,
    Vector2Index,
};
pub use self::vector3::Vector3;

pub mod direction;
pub use self::direction::{
    Direction,
    CardinalDirection,
    OrdinalDirection,
    SubDirection,
};

#[cfg(test)]
mod tests;
