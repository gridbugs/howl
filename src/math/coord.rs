use std::cmp;

use math::Vector2;
use direction::OrdinalDirection;

pub type Coord = Vector2<isize>;

impl Coord {
    fn as_f64_vector(self) -> Vector2<f64> {
        Vector2::new(self.x as f64, self.y as f64)
    }

    pub fn cell_centre(self) -> Vector2<f64> {
        self.as_f64_vector() + Vector2::new(0.5, 0.5)
    }

    pub fn cell_corner(self, dir: OrdinalDirection) -> Vector2<f64> {
        self.as_f64_vector() + dir.corner_offset()
    }

    pub fn real_distance(self, other: Coord) -> f64 {
        ((self - other).length_squared() as f64).sqrt()
    }

    pub fn manhatten_distance(self, other: Coord) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    pub fn square_distance(self, other: Coord) -> usize {
        cmp::max((self.x - other.x).abs(), (self.y - other.y).abs()) as usize
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self == other {
            Some(cmp::Ordering::Equal)
        } else if self.x <= other.x && self.y <= other.y {
            Some(cmp::Ordering::Less)
        } else if self.x >= other.x && self.y >= other.y {
            Some(cmp::Ordering::Greater)
        } else {
            None
        }
    }
}
