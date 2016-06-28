use geometry::vector2::Vector2;

pub type Coord = Vector2<isize>;

impl Coord {
    pub fn wrapping_increment_in_place(&mut self, width: isize) {
        self.x += 1;
        if self.x == width {
            self.x = 0;
            self.y += 1;
        }
    }

    pub fn wrapping_increment(&self, width: isize) -> Coord {
        let mut ret = *self;
        ret.wrapping_increment_in_place(width);
        ret
    }
}
