use grid::StaticGrid;
use geometry::Vector2;

pub trait Opacity {
    fn opacity(&self) -> f64;
}

pub trait VisionReport {
    fn clear(&mut self);
    fn see(&mut self, coord: Vector2<isize>, visibility: f64);
}

// Information about observer's vision
#[derive(Clone, Copy, Debug)]
pub struct VisionInfo {
    pub distance: usize,
}

impl VisionInfo {
    pub fn new(distance: usize) -> Self {
        VisionInfo {
            distance: distance,
        }
    }
}

pub trait Observer<O: Opacity, R: VisionReport> {
    fn observe(
        &self,
        eye: Vector2<isize>,
        grid: &StaticGrid<O>,
        info: VisionInfo,
        report: &mut R);
}

impl<O, R, F> Observer<O, R> for F
    where O: Opacity,
          R: VisionReport,
          F: Fn(Vector2<isize>, &StaticGrid<O>, VisionInfo, &mut R),
{
    fn observe(
        &self,
        eye: Vector2<isize>,
        grid: &StaticGrid<O>,
        info: VisionInfo,
        report: &mut R)
    {
        self(eye, grid, info, report);
    }
}
