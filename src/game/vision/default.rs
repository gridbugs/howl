use game::SpacialHashCell;
use game::vision::{
    Opacity,
    VisibilityReport,
};

use geometry::Vector2;

use std::collections::HashMap;

pub type DefaultOpacity = SpacialHashCell;

impl Opacity for DefaultOpacity {
    fn opacity(&self) -> f64 { self.opacity }
}

pub type DefaultVisibilityReport = HashMap<Vector2<isize>, f64>;

impl VisibilityReport for DefaultVisibilityReport {

    type MetaData = f64;

    fn see(&mut self, coord: Vector2<isize>, visibility: Self::MetaData) {
        self.insert(coord, visibility);
    }
}
