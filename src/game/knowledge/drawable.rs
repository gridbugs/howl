use game::*;
use content_types::*;
use ecs_content::Entity;

use spatial_hash::*;
use grid::Grid;
use math::Coord;

impl LevelKnowledge for DrawableKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, action_env: ActionEnv) -> bool {

        if let Some(knowledge_cell) = self.grid.get_mut(coord) {

            if knowledge_cell.last_updated == action_env.id {
                // this cell has already been updated
                return false;
            }

            let change = knowledge_cell.update(world_cell, accuracy, action_env);

            if self.last_action_id != action_env.id {
                self.targets.clear();
                self.last_action_id = action_env.id;
            }

            if world_cell.has_enemy() {
                self.targets.push(coord);
            }

            change
        } else {
            false
        }
    }
}

impl KnowledgeCell for DrawableKnowledgeCell {
    fn update(&mut self, world_cell: &SpatialHashCell, _accuracy: f64, action_env: ActionEnv) -> bool {

        let mut changed = false;

        if self.last_updated <= world_cell.last_updated() {

            self.foreground.clear();
            self.background.clear();
            self.name.clear();
            self.description.clear();
            self.health_overlay.clear();

            for entity in action_env.ecs.entity_iter(world_cell.entity_id_iter()) {
                entity.copy_tile_depth().map(|depth| {
                    entity.copy_tile().map(|tile| {
                        self.foreground.insert(depth, tile);
                        if tile.opaque_bg() {
                            self.background.insert(depth, tile);
                        }
                    });
                    entity.copy_name().map(|name| {
                        self.name.insert(depth, name);
                    });
                    entity.copy_description().map(|description| {
                        self.description.insert(depth, description);
                    });
                    if entity.contains_health_bar() {
                        entity.copy_hit_points().map(|hit_points| {
                            self.health_overlay.insert(depth, hit_points);
                        });
                    }
                });
            }

            changed = true;
        }

        self.last_updated = action_env.id;

        changed
    }
}

pub struct CellDrawInfo {
    pub foreground: Option<TileType>,
    pub background: Option<TileType>,
    pub visible: bool,
    pub front: bool,
    pub health_overlay: Option<HitPoints>,
}

impl Default for CellDrawInfo {
    fn default() -> Self {
        CellDrawInfo {
            foreground: None,
            background: None,
            visible: false,
            front: false,
            health_overlay: None,
        }
    }
}
