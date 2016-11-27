/// Dummy vision system that sees a square around the eye

use math::Coord;
use game::{SpatialHashTable, LevelKnowledge, Turn};

pub fn square_observe<K: LevelKnowledge>(eye: Coord,
                                         distance: usize,
                                         world: &SpatialHashTable,
                                         knowledge: &mut K,
                                         turn: Turn) {
    let distance = distance as isize;
    for i in -distance..distance + 1 {
        for j in -distance..distance + 1 {
            let coord = eye + Coord::new(j, i);
            knowledge.update_cell(coord, world.get(coord), 1.0, turn);
        }
    }
}
