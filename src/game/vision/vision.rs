use grid::StaticGrid;
use geometry::Vector2;

/// Trait used to convey the opacity of a cell to vision systems
pub trait Opacity {
    fn opacity(&self) -> f64;
}

/// Trait used by vision systems to communicate which cells are visible
pub trait VisibilityReport {

    /// Aditional information about the visibility of a cell
    type MetaData;

    /// Called by vision systems to mark a cell as visible
    fn see(&mut self, coord: Vector2<isize>, info: Self::MetaData);
}

/// Trait implemented by vision systems
pub trait VisionSystem<O: Opacity, R: VisibilityReport> {

    /// Information describing a viewer's vision
    type VisionInfo;

    fn detect_visible_area(
        &self,
        eye: Vector2<isize>,
        grid: &StaticGrid<O>,
        info: Self::VisionInfo,
        report: &mut R);
}
