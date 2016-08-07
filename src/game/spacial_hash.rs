use game::entity::{Entity, EntityId, ComponentType, Component, EntityTable};
use game::update::UpdateSummary;
use std::collections::HashMap;
use std::collections::HashSet;

use game::game_entity::GameEntity;

#[derive(Debug, Clone)]
pub struct SpacialHashCell {
    pub entities: HashSet<EntityId>,
    pub components: HashMap<ComponentType, usize>,
}

impl SpacialHashCell {
    pub fn new() -> Self {
        SpacialHashCell {
            entities: HashSet::new(),
            components: HashMap::new(),
        }
    }

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

    pub fn insert(&mut self, entity: &Entity) {
        if self.entities.insert(entity.id.unwrap()) {

            // update component counts
            for component_type in entity.slots.keys() {
                let count = self.get_count(*component_type);
                self.set_count(*component_type, count + 1);
            }
        }
    }

    pub fn remove(&mut self, entity: &Entity) {
        if self.entities.remove(&entity.id.unwrap()) {
            // update component counts
            for component_type in entity.slots.keys() {
                let count = self.get_count(*component_type);
                self.set_count(*component_type, count - 1);
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct SpacialHashMap {
    pub id: Option<EntityId>,
    pub hash_map: HashMap<(isize, isize), SpacialHashCell>,
}

impl SpacialHashMap {
    pub fn new() -> Self {
        SpacialHashMap {
            id: None,
            hash_map: HashMap::new(),
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
        self.hash_map.get(&coord)
    }

    fn get_mut(&mut self, coord: (isize, isize)) -> &mut SpacialHashCell {
        if !self.hash_map.contains_key(&coord) {
            self.hash_map.insert(coord, SpacialHashCell::new());
        }
        self.hash_map.get_mut(&coord).unwrap()
    }

    pub fn add_entity(&mut self, entity: &Entity) {
        if let Some(vec) = entity.position() {
            let cell = self.get_mut(vec.to_tuple());
            cell.insert(entity);
        }
    }

    pub fn change_entity(&mut self, entity: &Entity, changes: &HashMap<ComponentType, Component>) {
        for (_, component) in changes {
            if let &Component::Position(old_position) = component {
                let new_position = entity.position().unwrap();
                self.get_mut(old_position.to_tuple()).remove(entity);
                self.get_mut(new_position.to_tuple()).insert(entity);
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
