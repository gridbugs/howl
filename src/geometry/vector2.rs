use std::marker::Copy;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {x: x, y: y}
    }

    pub fn convert<S>(self) -> Vector2<S> where S: From<T> {
        Vector2 { x: S::from(self.x), y: S::from(self.y) }
    }
}

// Vector Addition
impl<T, S> Add<Vector2<S>> for Vector2<T> where T: Add<S> {
    type Output = Vector2<T::Output>;

    fn add(self, other: Vector2<S>) -> Vector2<T::Output> {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T, S> AddAssign<Vector2<S>> for Vector2<T> where T: AddAssign<S> {
    fn add_assign(&mut self, other: Vector2<S>) {
        self.x += other.x;
        self.y += other.y;
    }
}

// Vector Subtraction
impl<T, S> Sub<Vector2<S>> for Vector2<T> where T: Sub<S> {
    type Output = Vector2<T::Output>;

    fn sub(self, other: Vector2<S>) -> Vector2<T::Output> {
        Vector2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl<T, S> SubAssign<Vector2<S>> for Vector2<T> where T: SubAssign<S> {
    fn sub_assign(&mut self, other: Vector2<S>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// Scalar Multiplication
impl<T, S> Mul<S> for Vector2<T> where T: Mul<S>, S: Copy {
    type Output = Vector2<T::Output>;

    fn mul(self, other: S) -> Vector2<T::Output> {
        Vector2 { x: self.x * other, y: self.y * other }
    }
}

impl<T, S> MulAssign<S> for Vector2<T> where T: MulAssign<S>, S: Copy {
    fn mul_assign(&mut self, other: S) {
        self.x *= other;
        self.y *= other;
    }
}
