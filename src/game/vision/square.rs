use game::vision::{
    Opacity,
    VisibilityReport,
};

use geometry::Vector2;
use grid::StaticGrid;

pub fn square<O: Opacity, R: VisibilityReport<MetaData=f64>>(
    eye: Vector2<isize>,
    grid: &StaticGrid<O>,
    distance: usize,
    report: &mut R)
{
    let distance = distance as isize;
    for i in -distance..distance + 1 {
        for j in -distance..distance + 1 {
            let coord = eye + Vector2::new(j, i);
            if grid.is_valid_coord(coord) {
                report.see(coord, 1.0);
            }
        }
    }
}
