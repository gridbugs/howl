use vision::VisibilityReport;

use geometry::Vector2;

use std::collections::HashMap;
use std::cell::RefMut;

pub type DefaultVisibilityReport = HashMap<Vector2<isize>, f64>;

impl VisibilityReport for DefaultVisibilityReport {
    type MetaData = f64;

    fn see(&mut self, coord: Vector2<isize>, visibility: Self::MetaData) {
        self.insert(coord, visibility);
    }
}

impl<'a> VisibilityReport for &'a mut DefaultVisibilityReport {
    type MetaData = f64;

    fn see(&mut self, coord: Vector2<isize>, visibility: Self::MetaData) {
        self.insert(coord, visibility);
    }
}

impl<'a> VisibilityReport for RefMut<'a, DefaultVisibilityReport> {
    type MetaData = f64;

    fn see(&mut self, coord: Vector2<isize>, visibility: Self::MetaData) {
        self.insert(coord, visibility);
    }
}

impl<'a> VisibilityReport for &'a mut RefMut<'a, DefaultVisibilityReport> {
    type MetaData = f64;

    fn see(&mut self, coord: Vector2<isize>, visibility: Self::MetaData) {
        self.insert(coord, visibility);
    }
}
