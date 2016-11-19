/// Dummy vision system that sees only the cell containing the eye

use math::Coord;
use spatial_hash::SpatialHashTable;
use knowledge::LevelKnowledge;

pub fn blind_observe<K: LevelKnowledge>(eye: Coord, world: &SpatialHashTable, knowledge: &mut K) {
    world.get(eye).map(|cell| knowledge.update_cell(eye, cell, 1.0));
}
