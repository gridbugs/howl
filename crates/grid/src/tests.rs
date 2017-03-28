use math::Coord;
use grid::*;
use bidirectional_list::*;
use dynamic_grid::*;

#[test]
fn dynamic_grid_get_mut_with_default() {
    let mut grid = DynamicGrid::new();

    *grid.get_mut_with_default(Coord::new(4, 5)) = 1;
    *grid.get_mut_with_default(Coord::new(-1, 2)) = 2;

    assert_eq!(*grid.get_checked(Coord::new(4, 5)), 1);
    assert_eq!(*grid.get_checked(Coord::new(-1, 2)), 2);
    assert_eq!(grid.limits_min(), Coord::new(-1, 0));
    assert_eq!(grid.limits_max(), Coord::new(4, 5));
}

#[test]
fn bidirectional_list_get_mut_with_default() {
    let mut l = BidirectionalList::new();

    assert_eq!(l.get(0), None);

    *l.get_mut_with_default(-1) = 1;
    *l.get_mut_with_default(1) = 2;

    assert_eq!(l.get_checked(0), &0);
    assert_eq!(l.get_checked(-1), &1);
    assert_eq!(l.get_checked(1), &2);
}
