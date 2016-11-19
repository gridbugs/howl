pub trait Dot<RHS = Self> {
    type Output;

    fn dot(self, rhs: RHS) -> Self::Output;
}
