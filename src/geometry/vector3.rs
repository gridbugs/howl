use std::marker::Copy;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use geometry::vector::Dot;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 {x: x, y: y, z: z}
    }

    pub fn convert<S>(self) -> Vector3<S> where S: From<T> {
        Vector3 { x: S::from(self.x), y: S::from(self.y), z: S::from(self.z) }
    }
}

// Vector Addition
impl<T, S> Add<Vector3<S>> for Vector3<T> where T: Add<S> {
    type Output = Vector3<T::Output>;

    fn add(self, other: Vector3<S>) -> Vector3<T::Output> {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<T, S> AddAssign<Vector3<S>> for Vector3<T> where T: AddAssign<S> {
    fn add_assign(&mut self, other: Vector3<S>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vector Subtraction
impl<T, S> Sub<Vector3<S>> for Vector3<T> where T: Sub<S> {
    type Output = Vector3<T::Output>;

    fn sub(self, other: Vector3<S>) -> Vector3<T::Output> {
        Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<T, S> SubAssign<Vector3<S>> for Vector3<T> where T: SubAssign<S> {
    fn sub_assign(&mut self, other: Vector3<S>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Scalar Multiplication
impl<T, S> Mul<S> for Vector3<T> where T: Mul<S>, S: Copy {
    type Output = Vector3<T::Output>;

    fn mul(self, other: S) -> Vector3<T::Output> {
        Vector3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl<T, S> MulAssign<S> for Vector3<T> where T: MulAssign<S>, S: Copy {
    fn mul_assign(&mut self, other: S) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

// Dot Product
impl<T, S> Dot<Vector3<S>> for Vector3<T>
    where T: Mul<S>,
          <T as Mul<S>>::Output: Add,
          <<T as Mul<S>>::Output as Add>::Output: Add<<T as Mul<S>>::Output>
{
    type Output = <<<T as Mul<S>>::Output as Add>::Output as Add<<T as Mul<S>>::Output>>::Output;

    fn dot(self, rhs: Vector3<S>) -> Self::Output {
        (self.x * rhs.x + self.y * rhs.y) + self.z * rhs.z
    }
}
