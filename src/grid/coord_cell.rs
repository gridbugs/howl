pub trait CoordCell {
    type Data: Copy;
    fn new(x: isize, y: isize, data: Self::Data) -> Self;
}
