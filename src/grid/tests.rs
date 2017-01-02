use coord::Coord;
use grid::*;

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
