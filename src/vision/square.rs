use vision::{
    Opacity,
    VisibilityReport,
    VisionSystem,
};

use geometry::Vector2;

use grid::Grid;

pub struct Square;
impl<G, R> VisionSystem<G, R, usize> for Square
    where G: Grid,
          G::Item: Opacity,
          R: VisibilityReport<MetaData=f64>
{
    fn detect_visible_area(
        &mut self,
        eye: Vector2<isize>,
        grid: &G,
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
}
