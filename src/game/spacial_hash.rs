use game::entity::{Entity, EntityId, ComponentType, Component, EntityTable};
use game::update::{UpdateSummary, ComponentChange};
use game::update::ComponentChange::*;
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

    pub fn change_entity(&mut self, entity: &Entity, changes: &HashMap<ComponentType, ComponentChange>) {
        for (component_type, change) in changes {

            // changing the position of an entity
            if let &Set(Component::Position(old_position)) = change {
                let new_position = entity.position().unwrap();
                self.get_mut(old_position.to_tuple()).unwrap().remove(entity);
                self.get_mut(new_position.to_tuple()).unwrap().insert(entity);
            } else if let Some(&Component::Position(v)) = entity.get(ComponentType::Position) {
                // update component counts
                if let &Add = change {
                    self.get_mut(v.to_tuple()).unwrap().increment_count(*component_type);
                } else if let &Remove(_) = change {
                    self.get_mut(v.to_tuple()).unwrap().decrement_count(*component_type);
                }
            }
        }
    }

    pub fn update(&mut self, update: &UpdateSummary, entities: &EntityTable) {
        for entity_id in &update.added_entities {
            self.get_entity(*entity_id, entities).map(|entity| {
                self.add_entity(entity);
            });
        }
        for (entity_id, components) in &update.changed_entities {
            self.get_entity(*entity_id, entities).map(|entity| {
                self.change_entity(entity, components);
            });
        }
    }
}
