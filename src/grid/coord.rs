use geometry::{Vector2, OrdinalDirection};

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
}
