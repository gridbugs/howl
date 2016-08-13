use game::entity::{
    Entity,
    EntityId,
    ComponentType,
    EntityTable,
};
use game::update::UpdateSummary;
use game::game_entity::GameEntity;
use std::collections::HashMap;
use std::collections::HashSet;

use grid::StaticGrid;
use geometry::Vector2;

#[derive(Debug, Clone)]
pub struct SpacialHashCell {
    pub entities: HashSet<EntityId>,
    pub components: HashMap<ComponentType, usize>,
}

impl Default for SpacialHashCell {
    fn default() -> Self {
        SpacialHashCell {
            entities: HashSet::new(),
            components: HashMap::new(),
        }
    }
}

impl SpacialHashCell {
    pub fn has(&self, component_type: ComponentType) -> bool {
        if let Some(count) = self.components.get(&component_type) {
            *count != 0
        } else {
            false
        }
    }

    fn get_count(&self, component_type: ComponentType) -> usize {
        if let Some(count) = self.components.get(&component_type) {
            *count
        } else {
            0
        }
    }

    fn set_count(&mut self, component_type: ComponentType, new_count: usize) {
        if self.components.contains_key(&component_type) {
            *self.components.get_mut(&component_type).unwrap() = new_count;
        } else {
            self.components.insert(component_type, new_count);
        }
    }

    fn increment_count(&mut self, component_type: ComponentType) {
        let count = self.get_count(component_type);
        self.set_count(component_type, count + 1);
    }

    fn decrement_count(&mut self, component_type: ComponentType) {
        let count = self.get_count(component_type);
        self.set_count(component_type, count - 1);
    }

    fn insert(&mut self, entity: &Entity) {
        if self.entities.insert(entity.id.unwrap()) {

            // update component counts
            for component_type in entity.slots.keys() {
                self.increment_count(*component_type);
            }
        }
    }

    fn remove(&mut self, entity: &Entity) {
        if self.entities.remove(&entity.id.unwrap()) {
            // update component counts
            for component_type in entity.slots.keys() {
                self.decrement_count(*component_type);
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct SpacialHashMap {
    pub id: Option<EntityId>,
    pub grid: StaticGrid<SpacialHashCell>,
}

impl SpacialHashMap {
    pub fn new(width: usize, height: usize) -> Self {
        SpacialHashMap {
            id: None,
            grid: StaticGrid::new_default(width, height),
        }
    }

    pub fn set_id(&mut self, id: EntityId) {
        self.id = Some(id);
    }

    fn entity_is_on_level(&self, entity: &Entity) -> bool {
        if let Some(id) = entity.on_level() {
            if id == self.id.unwrap() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_entity<'a, 'b>(&'a self, entity_id: EntityId, entities: &'b EntityTable) -> Option<&'b Entity> {
        let entity = entities.get(entity_id);
        if self.entity_is_on_level(entity) {
            Some(entities.get(entity_id))
        } else {
            None
        }
    }

    pub fn get(&self, coord: (isize, isize)) -> Option<&SpacialHashCell> {
        self.grid.get(Vector2::from_tuple(coord))
    }

    fn get_mut(&mut self, coord: (isize, isize)) -> Option<&mut SpacialHashCell> {
        self.grid.get_mut(Vector2::from_tuple(coord))
    }

    pub fn add_entity(&mut self, entity: &Entity) {
        if let Some(vec) = entity.position() {
            let cell = self.get_mut(vec.to_tuple()).unwrap();
            cell.insert(entity);
        }
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        if let Some(vec) = entity.position() {
            let cell = self.get_mut(vec.to_tuple()).unwrap();
            cell.remove(entity);
        }
    }

    pub fn add_components(&mut self, entity: &Entity, changes: &Entity) {

        // position will be set to the position of entity after the change
        let position = if let Some(new_position) = changes.position() {

            // position is special as it indicates which cell to update
            if let Some(old_position) = entity.position() {
                // entity is moving from old_position to new_position
                self.get_mut(old_position.to_tuple()).unwrap().remove(entity);
            }

            // the entity's position is changing or the entity is gaining a position
            // in either case, add the entity to the position's cell
            self.get_mut(new_position.to_tuple()).unwrap().insert(entity);

            // entity will eventually end up here
            Some(new_position)
        } else if let Some(current_position) = entity.position() {
            // entity isn't moving, so use its current position
            Some(current_position)
        } else {
            // entity has no position, so the spacial hash won't be updated
            None
        };

        if let Some(position) = position {
            let mut cell = self.get_mut(position.to_tuple()).unwrap();
            for component_type in changes.slots.keys() {
                if *component_type == ComponentType::Position {
                    // this has already been handled
                    continue;
                }

                // only update the component count if the component is being added
                if !entity.has(*component_type) {
                    cell.increment_count(*component_type);
                }
            }
        }
    }

    pub fn remove_components(&mut self, entity: &Entity, component_types: &HashSet<ComponentType>) {
        if let Some(position) = entity.position() {
            if component_types.contains(&ComponentType::Position) {
                // removing position - remove the entity
                self.remove_entity(entity);
            } else {
                let mut cell = self.get_mut(position.to_tuple()).unwrap();
                // decrement count for each removed component 
                for component_type in component_types {
                    if entity.has(*component_type) {
                        cell.decrement_count(*component_type);
                    }
                }
            }
        }
    }

    pub fn update(&mut self, update: &UpdateSummary, entities: &EntityTable) {
        for entity in update.added_entities.values() {
            if self.entity_is_on_level(entity) {
                self.add_entity(entity);
            }
        }

        for entity_id in &update.removed_entities {
            let entity = entities.get(*entity_id);
            if self.entity_is_on_level(entity) {
                self.remove_entity(entity);
            }
        }

        for (entity_id, changes) in &update.added_components {
            let entity = entities.get(*entity_id);
            if self.entity_is_on_level(entity) {
                self.add_components(entity, changes);
            }
        }

        for (entity_id, component_types) in &update.removed_components {
            let entity = entities.get(*entity_id);
            if self.entity_is_on_level(entity) {
                self.remove_components(entity, component_types);
            }
        }
    }
}
