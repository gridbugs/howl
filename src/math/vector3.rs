use std::marker::Copy;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use math::Dot;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn convert<S>(self) -> Vector3<S>
        where S: From<T>
    {
        Vector3 {
            x: S::from(self.x),
            y: S::from(self.y),
            z: S::from(self.z),
        }
    }
}

impl Vector3<f64> {
    pub fn from_radial(length: f64, h_angle: f64, v_angle: f64) -> Self {
        let r = length * v_angle.cos();
        Vector3::new(r * h_angle.cos(), r * h_angle.sin(), length * v_angle.sin())
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

// Vector Addition
impl<T, S> Add<Vector3<S>> for Vector3<T>
    where T: Add<S>
{
    type Output = Vector3<T::Output>;

    fn add(self, other: Vector3<S>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T, S> AddAssign<Vector3<S>> for Vector3<T>
    where T: AddAssign<S>
{
    fn add_assign(&mut self, other: Vector3<S>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vector Subtraction
impl<T, S> Sub<Vector3<S>> for Vector3<T>
    where T: Sub<S>
{
    type Output = Vector3<T::Output>;

    fn sub(self, other: Vector3<S>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T, S> SubAssign<Vector3<S>> for Vector3<T>
    where T: SubAssign<S>
{
    fn sub_assign(&mut self, other: Vector3<S>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Scalar Multiplication
impl<T, S> Mul<S> for Vector3<T>
    where T: Mul<S>,
          S: Copy
{
    type Output = Vector3<T::Output>;

    fn mul(self, other: S) -> Vector3<T::Output> {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T, S> MulAssign<S> for Vector3<T>
    where T: MulAssign<S>,
          S: Copy
{
    fn mul_assign(&mut self, other: S) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

// Scalar Division
impl<T, S> Div<S> for Vector3<T>
    where T: Div<S>,
          S: Copy
{
    type Output = Vector3<T::Output>;

    fn div(self, other: S) -> Vector3<T::Output> {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T, S> DivAssign<S> for Vector3<T>
    where T: DivAssign<S>,
          S: Copy
{
    fn div_assign(&mut self, other: S) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
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

impl<T> Vector3<T>
    where Vector3<T>: Dot<Vector3<T>>,
          T: Copy
{
    pub fn length_squared(self) -> <Self as Dot<Self>>::Output {
        self.dot(self)
    }
}
