use game::vision::{
    Opacity,
    VisibilityReport,
    VisionSystem,
};

use geometry::Vector2;

use grid::Grid;

pub fn square<'a, G, R>(
    eye: Vector2<isize>,
    grid: &G,
    distance: usize,
    report: &mut R)
    where G: Grid<'a>,
          G::Item: Opacity,
          R: VisibilityReport<MetaData=f64>
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
impl<'a, G, R> VisionSystem<'a, G, R, usize> for Square
    where G: Grid<'a>,
          G::Item: Opacity,
          R: VisibilityReport<MetaData=f64>
{
    fn detect_visible_area(
        &self,
        eye: Vector2<isize>,
        grid: &G,
        info: usize,
        report: &mut R)
    {
        square(eye, grid, info, report);
    }
}
