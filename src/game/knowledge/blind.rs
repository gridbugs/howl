/// Dummy vision system that sees only the cell containing the eye

use coord::Coord;
use game::{SpatialHashTable, LevelKnowledge, ActionEnv};

pub fn blind_observe<K: LevelKnowledge>(eye: Coord, world: &SpatialHashTable, knowledge: &mut K, action_env: ActionEnv) {
    knowledge.update_cell(eye, world.get(eye), 1.0, action_env);
}
