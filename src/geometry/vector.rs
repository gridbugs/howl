use std::marker::{Sized, Copy};
use std::ops::Div;
use num::Float;

pub trait Dot<RHS = Self> {
    type Output;

    fn dot(self, rhs: RHS) -> Self::Output;
}

pub trait LengthSquared: Dot<Self>
    where Self: Sized + Copy
{
    fn len_sq(self) -> Self::Output {
        self.dot(self)
    }
}

pub trait Length<T: Float> {
    fn length_squared(self) -> T;
    fn length(self) -> T
        where Self: Sized
    {
        self.length_squared().sqrt()
    }
    fn normalize(self) -> <Self as Div<T>>::Output
        where Self: Copy + Sized + Div<T>
    {
        self / self.length()
    }
}

impl<T> Length<T::Output> for T
    where T: Dot<T> + Copy,
          T::Output: Float
{
    fn length_squared(self) -> T::Output {
        self.dot(self)
    }
}
