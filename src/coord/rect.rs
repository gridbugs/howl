use std::result;

use math::Vector2;
use coord::Coord;

#[derive(Debug)]
pub enum Error {
    TooNarrow,
}

pub type Result<T> = result::Result<T, Error>;

pub struct Rect {
    top_left: Coord,
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(top_left: Coord, width: usize, height: usize) -> Result<Self> {
        if width < 2 || height < 2 {
            return Err(Error::TooNarrow);
        }

        Ok(Rect {
            top_left: top_left,
            width: width,
            height: height,
        })
    }

    pub fn new_centred_square(centre: Coord, radius: usize) -> Result<Self> {
        Self::new(centre - Coord::new(radius as isize, radius as isize), radius * 2 + 1, radius * 2 + 1)
    }

    pub fn border_iter(&self) -> RectBorderIter {
        RectBorderIter::new(self.top_left, self.width, self.height)
    }

    pub fn border_count(&self) -> usize {
        (self.width + self.height) * 2 - 4
    }

    pub fn border_get(&self, idx: usize) -> Option<Coord> {
        let v = if idx < self.width - 1 {
            Vector2::new(1 + idx, 0)
        } else if idx < self.width + self.height - 2 {
            let offset = idx - (self.width - 2);
            Vector2::new(self.width - 1, offset)
        } else if idx < self.width * 2 + self.height - 3 {
            let offset = idx - (self.width + self.height - 3);
            Vector2::new(self.width - offset - 1, self.height - 1)
        } else if idx < self.border_count() {
            let offset = idx - (self.width * 2 + self.height - 4);
            Vector2::new(0, self.height - offset - 1)
        } else {
            return None;
        };

        Some(self.top_left + Coord::new(v.x as isize, v.y as isize))
    }
}

#[derive(Clone, Copy)]
enum SideType {
    Top,
    Right,
    Bottom,
    Left,
}

impl SideType {
    fn step(self) -> Coord {
        match self {
            SideType::Top => Coord::new(1, 0),
            SideType::Right => Coord::new(0, 1),
            SideType::Bottom => Coord::new(-1, 0),
            SideType::Left => Coord::new(0, -1),
        }
    }
}

pub struct RectBorderIter {
    side_remain: usize,
    width: usize,
    height: usize,
    coord: Coord,
    side: SideType,
}

impl RectBorderIter {
    fn new(top_left: Coord, width: usize, height: usize) -> Self {
        RectBorderIter {
            side_remain: width - 1,
            width: width,
            height: height,
            coord: top_left,
            side: SideType::Top,
        }
    }
}

impl Iterator for RectBorderIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {

        if self.side_remain == 0 {
            self.side = match self.side {
                SideType::Top => SideType::Right,
                SideType::Right => SideType::Bottom,
                SideType::Bottom => SideType::Left,
                SideType::Left => return None,
            };

            self.side_remain = match self.side {
                SideType::Top | SideType::Bottom => self.width - 1,
                SideType::Left | SideType::Right => self.height - 1,
            };
        }

        self.side_remain -= 1;
        self.coord += self.side.step();

        Some(self.coord)
    }
}
