use game::SpacialHashCell;
use game::vision::{
    Opacity,
    Observer,
    VisionReport,
};

use geometry::Vector2;

use std::collections::HashMap;

pub type DefaultOpacity = SpacialHashCell;

impl Opacity for DefaultOpacity {
    fn opacity(&self) -> f64 { self.opacity }
}

pub type DefaultVisionReport = HashMap<Vector2<isize>, f64>;

impl VisionReport for DefaultVisionReport {
    fn clear(&mut self) {
        HashMap::clear(self);
    }

    fn see(&mut self, coord: Vector2<isize>, visibility: f64) {
        self.insert(coord, visibility);
    }
}

pub type DefaultObserver =
    Observer<SpacialHashCell, DefaultVisionReport>;
