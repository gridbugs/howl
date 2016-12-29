use math::*;
use bresenham;

#[test]
fn make_line() {
    let mut line = CoordLine::new();
    let start = Coord::new(2, 2);
    let end = Coord::new(-3, -1);
    bresenham::make_line(start, end, &mut line);

    let mut iter = line.iter();

    assert_eq!(iter.next(), Some(Coord::new(2, 2)));
    assert_eq!(iter.next(), Some(Coord::new(1, 1)));
    assert_eq!(iter.next(), Some(Coord::new(0, 1)));
    assert_eq!(iter.next(), Some(Coord::new(-1, 0)));
    assert_eq!(iter.next(), Some(Coord::new(-2, 0)));
    assert_eq!(iter.next(), Some(Coord::new(-3, -1)));
    assert_eq!(iter.next(), None);
}
