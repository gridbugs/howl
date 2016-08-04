use ecs::entity::{Entity, EntityId, ComponentType, EntityTable};
use ecs::update::UpdateSummary;
use std::collections::HashMap;
use std::collections::HashSet;

use game::util;

use debug;

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
        if self.entities.contains(&entity.id.unwrap()) {
            // set already contains entity
            return;
        }

        // add entity id
        self.entities.insert(entity.id.unwrap());

        // update component counts
        for component_type in entity.slots.keys() {
            let count = self.get_count(*component_type);
            self.set_count(*component_type, count + 1);
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
        if let Some(id) = util::get_level(entity) {
            if id == self.id.unwrap() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_cell_mut(&mut self, coord: (isize, isize)) -> &mut SpacialHashCell {
        if !self.hash_map.contains_key(&coord) {
            self.hash_map.insert(coord, SpacialHashCell::new());
        }
        self.hash_map.get_mut(&coord).unwrap()
    }

    pub fn add_entity(&mut self, entity: &Entity) {
        if let Some(vec) = util::get_position(entity) {
            let cell = self.get_cell_mut(vec.to_tuple());
            cell.insert(entity);
        }
    }

    pub fn update(&mut self, update: &UpdateSummary, entities: &EntityTable) {
        for entity_id in &update.added_entities {
            let entity = entities.get(*entity_id);
            if self.entity_is_on_level(entity) {
                self.add_entity(entities.get(*entity_id));
            }
        }
    }
}
