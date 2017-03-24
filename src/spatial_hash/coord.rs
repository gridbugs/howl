use coord::Coord;
use spatial_hash::generated::SpatialHashTableCoord;

impl SpatialHashTableCoord for Coord {
    fn x(self) -> usize { self.x as usize }
    fn y(self) -> usize { self.y as usize }
}
