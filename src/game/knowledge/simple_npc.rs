use game::*;
use spatial_hash::*;
use grid::Grid;
use math::Coord;
use content_types::*;

impl LevelKnowledge for SimpleNpcKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, action_env: ActionEnv) -> bool {
        if let Some(knowledge_cell) = self.grid.get_mut(coord) {
            if knowledge_cell.update(world_cell, accuracy, action_env) {
                if world_cell.has_pc() {
                    if action_env.id == self.latest_target {
                        self.targets.insert(coord);
                    } else if action_env.id > self.latest_target {
                        self.targets.clear();
                        self.targets.insert(coord);
                        self.latest_target = action_env.id;
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl KnowledgeCell for SimpleNpcKnowledgeCell {
    fn update(&mut self, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {
        let mut changed = false;

        if self.last_updated <= world_cell.last_updated() {
            changed = true;

            self.solid = world_cell.is_solid();
            self.acid = world_cell.is_acid();
        }

        self.last_updated = action_env.id;

        changed
    }
}
