/// Dummy vision system that sees a square around the eye

use math::Coord;
use spatial_hash::SpatialHashTable;
use knowledge::LevelKnowledge;

pub fn square_observe<K: LevelKnowledge>(eye: Coord,
                                         distance: usize,
                                         world: &SpatialHashTable,
                                         knowledge: &mut K) {
    let distance = distance as isize;
    for i in -distance..distance + 1 {
        for j in -distance..distance + 1 {
            let coord = eye + Coord::new(j, i);
            if let Some(cell) = world.get(coord) {
                knowledge.update_cell(coord, cell, 1.0);
            }
        }
    }
}
