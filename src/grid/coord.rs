use geometry::Vector2;

pub type Coord = Vector2<isize>;

impl Coord {
    pub fn wrapping_increment_in_place(&mut self, width: usize) {
        self.x += 1;
        if self.x == width as isize {
            self.x = 0;
            self.y += 1;
        }
    }

    pub fn wrapping_increment(&self, width: usize) -> Coord {
        let mut ret = *self;
        ret.wrapping_increment_in_place(width);
        ret
    }
}
