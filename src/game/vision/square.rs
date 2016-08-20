use game::vision::{
    Opacity,
    VisibilityReport,
    VisionSystem,
};

use geometry::Vector2;

use grid::{
    Grid,
    StaticGrid,
};

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

pub struct Square;
impl<O, R> VisionSystem<O, R, usize> for Square
    where O: Opacity,
          R: VisibilityReport<MetaData=f64>
{
    fn detect_visible_area(
        &self,
        eye: Vector2<isize>,
        grid: &StaticGrid<O>,
        info: usize,
        report: &mut R)
    {
        square(eye, grid, info, report);
    }
}
