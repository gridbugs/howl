use std::marker::Copy;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use geometry::vector::Dot;
use rand;
use rand::Rng;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug, Default, PartialEq, Hash, Eq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug)]
pub enum Vector2Index {
    X,
    Y,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 {x: x, y: y}
    }

    pub fn convert<S>(self) -> Vector2<S> where S: From<T> {
        Vector2 { x: S::from(self.x), y: S::from(self.y) }
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn from_tuple(tuple: (T, T)) -> Self {
        Vector2::new(tuple.0, tuple.1)
    }
}

impl Vector2<f64> {
    pub fn from_radial(length: f64, angle: f64) -> Self {
        Vector2::new(length * angle.cos(), length * angle.sin())
    }
    pub fn random_unit_vector() -> Self {
        Self::from_radial(1.0, rand::thread_rng().gen_range(-PI, PI))
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

// Scalar Division
impl<T, S> Div<S> for Vector2<T> where T: Div<S>, S: Copy {
    type Output = Vector2<T::Output>;

    fn div(self, other: S) -> Vector2<T::Output> {
        Vector2 { x: self.x / other, y: self.y / other }
    }
}

impl<T, S> DivAssign<S> for Vector2<T> where T: DivAssign<S>, S: Copy {
    fn div_assign(&mut self, other: S) {
        self.x /= other;
        self.y /= other;
    }
}

// Dot Product
impl<T, S> Dot<Vector2<S>> for Vector2<T>
    where T: Mul<S>,
          <T as Mul<S>>::Output: Add
{
    type Output = <T::Output as Add>::Output;

    fn dot(self, rhs: Vector2<S>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
