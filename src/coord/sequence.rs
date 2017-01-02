use coord::Coord;

pub struct CoordSequenceIter<I>
    where I::Item = Coord
{
    iter: I,
    initial: Option<Coord>,
}
