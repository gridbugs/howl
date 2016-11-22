use math::Coord;
use game::SpatialHashCell;

/// Trait implemented by representations of knowledge about a level
pub trait LevelKnowledge {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64);
}
