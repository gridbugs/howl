use game::vision::{
    Opacity,
    VisibilityReport,
    VisionSystem,
};

use geometry::Vector2;
use grid::StaticGrid;

pub struct Square;

impl<O: Opacity, R: VisibilityReport<MetaData=f64>> VisionSystem<O, R> for Square {

    type VisionInfo = usize; // distance

    fn detect_visible_area(
        &self,
        eye: Vector2<isize>,
        grid: &StaticGrid<O>,
        info: Self::VisionInfo,
        report: &mut R)
    {
        let distance = info as isize;
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
