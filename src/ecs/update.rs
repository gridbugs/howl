use ecs::entity::{EntityId, Entity, ComponentType, Component};

use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

pub enum Update {
    SetEntityComponent {
        entity_id: EntityId,
        component_type: ComponentType,
        component_value: Component,
    },
    AddEntity(Entity),
    RemoveEntity(EntityId),
    WithEntity(EntityId, Box<Fn(&mut Entity)>),

    Null,   // Makes no change
    Error(&'static str),  // Panics if applied

    ThenWithEntity(Box<Update>, Box<Fn(EntityId) -> Update>),
    Then(Box<Update>, Box<Update>),
}

impl fmt::Debug for Update {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Update {{}}")
    }
}

pub fn then_with_entity<F: 'static + Fn(EntityId) -> Update>(update: Update, f: F) -> Update {
    Update::ThenWithEntity(
        Box::new(update),
        Box::new(f)
    )
}

pub fn then(first: Update, second: Update) -> Update {
    Update::Then(
        Box::new(first),
        Box::new(second)
    )
}

pub fn with_entity<F: 'static + Fn(&mut Entity)>(id: EntityId, f: F) -> Update {
    Update::WithEntity(id, Box::new(f))
}

pub struct UpdateSummary {
    pub added_entities: HashSet<EntityId>,
    pub removed_entities: HashSet<EntityId>,
    pub changed_entities: HashMap<EntityId, HashSet<ComponentType>>,
}

impl UpdateSummary {
    pub fn new() -> Self {
        UpdateSummary {
            added_entities: HashSet::new(),
            removed_entities: HashSet::new(),
            changed_entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: EntityId) {
        self.added_entities.insert(entity);
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        self.removed_entities.insert(entity);
    }

    pub fn change_entity(&mut self, entity: EntityId, component: ComponentType) {
        if !self.changed_entities.contains_key(&entity) {
            self.changed_entities.insert(entity, HashSet::new());
        }

        self.changed_entities.get_mut(&entity).unwrap().insert(component);
    }
}
