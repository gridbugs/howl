use vision::{Opacity, VisibilityReport, VisionSystem};

use geometry::Vector2;

use grid::Grid;

pub struct Omniscient;
impl<G, R> VisionSystem<G, R, usize> for Omniscient
    where G: Grid,
          G::Item: Opacity,
          R: VisibilityReport<MetaData = f64>
{
    fn detect_visible_area(&mut self, _: Vector2<isize>, grid: &G, _: usize, report: &mut R) {

        for coord in grid.coord_iter() {
            report.see(coord, 1.0);
        }
    }
}
