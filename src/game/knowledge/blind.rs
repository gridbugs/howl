/// Dummy vision system that sees only the cell containing the eye

use math::Coord;
use game::{SpatialHashTable, LevelKnowledge, Turn};

pub fn blind_observe<K: LevelKnowledge>(eye: Coord, world: &SpatialHashTable, knowledge: &mut K, turn: Turn) {
    knowledge.update_cell(eye, world.get(eye), 1.0, turn);
}
