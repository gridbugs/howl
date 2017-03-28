use math::Coord;
use generated::SpatialHashTableCoord;

impl SpatialHashTableCoord for Coord {
    fn x(self) -> usize { self.x as usize }
    fn y(self) -> usize { self.y as usize }
}
