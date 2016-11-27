use std::collections::HashSet;

use ecs::*;
use math::Coord;
use grid::{Grid, DynamicGrid};
use game::Turn;

pub struct SpatialHashCell {
    // sum of opacities of everything in this cell
    opacity: f64,

    // count of solid entities in this cell
    solid: usize,

    // set of entities currently in this cell
    entities: EntitySet,

    // turn on which this cell was last updated
    last_updated: u64,
}

pub struct SpatialHashTable {
    grid: DynamicGrid<SpatialHashCell>,
    empty: SpatialHashCell,
}

impl SpatialHashCell {
    fn new() -> Self {
        SpatialHashCell {
            opacity: 0.0,
            solid: 0,
            entities: EntitySet::new(),
            last_updated: 0,
        }
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }

    pub fn entity_ids(&self) -> &EntitySet {
        &self.entities
    }

    pub fn entity_id_iter(&self) -> EntitySetIter {
        self.entities.iter()
    }

    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    pub fn solid(&self) -> bool {
        self.solid != 0
    }
}

impl Default for SpatialHashCell {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialHashTable {
    pub fn new() -> Self {
        SpatialHashTable {
            grid: DynamicGrid::new(),
            empty: SpatialHashCell::new(),
        }
    }

    pub fn get(&self, coord: Coord) -> &SpatialHashCell {
        self.grid.get(coord).unwrap_or(&self.empty)
    }

    pub fn limits_min(&self) -> Coord {
        self.grid.limits_min()
    }

    pub fn limits_max(&self) -> Coord {
        self.grid.limits_max()
    }

    fn get_mut_with_default(&mut self, coord: Coord) -> &mut SpatialHashCell {
        self.grid.get_mut_with_default(coord)
    }

    fn change_entity_position(&mut self, entity: EntityRef, current_position: Coord, new_position: Coord, turn_id: u64) {
        self.remove_entity_position(entity, current_position, turn_id);
        self.add_entity_position(entity, new_position, turn_id);
    }

    fn remove_entity_position(&mut self, entity: EntityRef, position: Coord, turn_id: u64) {
        let mut cell = self.get_mut_with_default(position);
        if entity.contains_solid() {
            cell.solid -= 1;
        }
        if let Some(opacity) = entity.opacity() {
            cell.opacity -= opacity;
        }
        cell.entities.remove(entity.id());
        cell.last_updated = turn_id;
    }

    fn add_entity_position(&mut self, entity: EntityRef, position: Coord, turn_id: u64) {
        let mut cell = self.get_mut_with_default(position);
        if entity.contains_solid() {
            cell.solid += 1;
        }
        if let Some(opacity) = entity.opacity() {
            cell.opacity += opacity;
        }
        cell.entities.insert(entity.id());
        cell.last_updated = turn_id;
    }

    fn update_insertions(&mut self, turn: Turn, insertions: &ActionInsertionTable) {
        for (entity_id, new_position) in insertions.position.iter() {
            let entity = turn.ecs.entity(*entity_id);
            // Add and remove tracked components based on the current data stored about the
            // entity, ignoring any component changes in the current action. These will be
            // applied later.
            if let Some(current_position) = entity.position() {
                // the entity is changing position
                self.change_entity_position(entity, current_position, *new_position, turn.id);
            } else {
                // the entity is gaining a position
                self.add_entity_position(entity, *new_position, turn.id);
            }
        }

        for (entity_id, new_opacity) in insertions.opacity.iter() {
            let entity = turn.ecs.post_insertion_entity(*entity_id, insertions);
            if let Some(position) = entity.position() {
                let current_opacity = entity.current_opacity().unwrap_or(0.0);
                let opacity_increase = new_opacity - current_opacity;
                let cell = self.get_mut_with_default(position);
                cell.opacity += opacity_increase;
                cell.last_updated = turn.id;
            }
        }

        for entity_id in insertions.solid.iter() {
            let entity = turn.ecs.post_insertion_entity(*entity_id, insertions);
            if let Some(position) = entity.position() {
                if !entity.current_contains_solid() {
                    // entity is becoming solid
                    let cell = self.get_mut_with_default(position);
                    cell.solid += 1;
                    cell.last_updated = turn.id;
                }
            }
        }
    }

    fn update_removals(&mut self, turn: Turn, removals: &ActionRemovalTable) {

        // loop through each component tracked by the spatial hash

        for entity_id in removals.solid.iter() {
            let entity = turn.ecs.entity(*entity_id);
            if let Some(position) = entity.position() {
                if entity.contains_solid() {
                    // removing solid from entity with position
                    let cell = self.get_mut_with_default(position);
                    cell.solid -= 1;
                    cell.last_updated = turn.id;
                }
            }
        }

        for entity_id in removals.opacity.iter() {
            let entity = turn.ecs.entity(*entity_id);
            if let Some(position) = entity.position() {
                if let Some(opacity) = entity.opacity() {
                    // removing opacity from entity with position
                    let cell = self.get_mut_with_default(position);
                    cell.opacity -= opacity;
                    cell.last_updated = turn.id;
                }
            }
        }

        // remove entities whose positions were removed
        for entity_id in removals.position.iter() {
            let entity = turn.ecs.entity(*entity_id);
            if let Some(position) = entity.position() {
                self.remove_entity_position(entity, position, turn.id);
            }
        }
    }

    fn update_removed_entities(&mut self,
                               turn: Turn,
                               entities: &HashSet<EntityId>) {
        for entity_id in entities.iter() {
            let entity = turn.ecs.entity(*entity_id);
            if let Some(position) = entity.position() {
                self.remove_entity_position(entity, position, turn.id);
            }
        }
    }

    pub fn update(&mut self, turn: Turn, action: &EcsAction) {
        self.update_insertions(turn, &action.insertions);
        self.update_removals(turn, &action.removals);
        self.update_removed_entities(turn, &action.removed_entities);
    }
}
