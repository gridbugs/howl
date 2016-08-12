use game::entity::{
    EntityId,
    Entity,
    ComponentType,
    Component,
    EntityTable,
};
use game::table::ToType;
use game::update::statement::{UpdateProgram, UpdateStatement};

use game::game_entity::GameEntity;

use std::collections::HashSet;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
pub enum ComponentChange {
    Add,
    Set(Component),
    Remove(Component),
}
use self::ComponentChange::*;

pub enum Change {
    Add(Component),
    Set(Component),
    Remove,
}

pub struct UpdateSummary_ {
    next_id: u64,
    pub added_entities: HashMap<EntityId, Entity>,
    pub removed_entities: HashSet<EntityId>,
    pub added_components: HashMap<EntityId, Entity>,
    pub removed_components: HashMap<EntityId, HashSet<ComponentType>>,
    levels: RefCell<HashSet<EntityId>>,
}

impl UpdateSummary_ {
    pub fn new() -> Self {
        UpdateSummary_ {
            next_id: 0,
            added_entities: HashMap::new(),
            removed_entities: HashSet::new(),
            added_components: HashMap::new(),
            removed_components: HashMap::new(),
            levels: RefCell::new(HashSet::new()),
        }
    }

    fn get_next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_entity(&mut self, mut entity: Entity) -> EntityId {
        let id = self.get_next_id();
        entity.id = Some(id);

        self.added_entities.insert(id, entity);

        id
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        self.removed_entities.insert(entity);
    }

    pub fn add_component(&mut self, entity: EntityId,
                         component: Component)
    {
        if !self.added_components.contains_key(&entity) {
            self.added_components.insert(entity, Entity::new());
        }
        self.added_components.get_mut(&entity).unwrap().add(component);
    }

    pub fn remove_component(&mut self, entity: EntityId,
                            component_type: ComponentType)
    {
        if !self.removed_components.contains_key(&entity) {
            self.removed_components.insert(entity, HashSet::new());
        }
        self.removed_components.get_mut(&entity).unwrap().insert(component_type);
    }

    pub fn commit(mut self, entities: &mut EntityTable) -> Self {

        self.update_spacial_hashes(entities);

        let mut revert = Self::new();

        for (_, entity) in self.added_entities.drain() {
            let id = entities.add(entity);
            revert.remove_entity(id);
        }

        for entity_id in self.removed_entities.drain() {
            let entity = entities.remove(entity_id).
                expect("Tried to remove non-existent entity.");
            revert.add_entity(entity);
        }

        for (entity_id, mut changes) in self.added_components.drain() {
            let mut entity = entities.get_mut(entity_id);
            for (component_type, component) in changes.slots.drain() {
                if let Some(original) = entity.add(component) {
                    revert.add_component(entity_id, original);
                } else {
                    revert.remove_component(entity_id, component_type);
                }
            }
        }

        for (entity_id, mut component_types) in self.removed_components.drain() {
            let mut entity = entities.get_mut(entity_id);
            for component_type in component_types.drain() {
                if let Some(component) = entity.remove(component_type) {
                    revert.add_component(entity_id, component);
                }
            }
        }

        revert
    }

    fn update_spacial_hashes(&self, entities: &EntityTable) {
        self.update_levels(entities);
        let levels = self.levels.borrow();

        for level_id in levels.iter() {
            let mut spacial_hash = {
                let level = entities.get(*level_id).level_data().unwrap();
                level.spacial_hash.borrow_mut()
            };
            spacial_hash.update(self, entities);
        }
    }

    fn update_levels(&self, entities: &EntityTable) {
        let mut levels = self.levels.borrow_mut();
        levels.clear();

        for entity in self.added_entities.values() {
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for entity_id in &self.removed_entities {
            let entity = entities.get(*entity_id);
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for entity_id in self.added_components.keys() {
            let entity = entities.get(*entity_id);
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for entity_id in self.removed_components.keys() {
            let entity = entities.get(*entity_id);
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }
    }
}


#[derive(Debug)]
pub struct UpdateSummary {
    pub added_entities: HashSet<EntityId>,
    pub removed_entities: HashMap<EntityId, Entity>,
    pub changed_entities: HashMap<EntityId, HashMap<ComponentType, ComponentChange>>,
    pub changed_components: HashSet<ComponentType>,
    levels: RefCell<HashSet<EntityId>>,
}

impl UpdateSummary {
    pub fn new() -> Self {
        UpdateSummary {
            added_entities: HashSet::new(),
            removed_entities: HashMap::new(),
            changed_entities: HashMap::new(),
            changed_components: HashSet::new(),
            levels: RefCell::new(HashSet::new()),
        }
    }

    pub fn add_entity_component(&mut self, entity: EntityId, component: Component) {

    }

    pub fn set_entity_component(&mut self, entity: EntityId, component: Component) {

    }

    pub fn remove_entity_component(&mut self, entity: EntityId, component_type: ComponentType) {

    }

    pub fn add_entity(&mut self, entity: EntityId) {
        self.added_entities.insert(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.removed_entities.insert(entity.id.unwrap(), entity);
    }

    fn ensure_changed_entity_entry(&mut self, entity: EntityId) {
        if !self.changed_entities.contains_key(&entity) {
            self.changed_entities.insert(entity, HashMap::new());
        }
    }

    pub fn change_entity(&mut self, entity: EntityId, old_component: Component) {
        self.ensure_changed_entity_entry(entity);

        let component_type = old_component.to_type();

        let existing = self.changed_entities.get_mut(&entity).unwrap()
            .insert(component_type, Set(old_component));

        assert!(existing.is_none());

        self.changed_components.insert(component_type);
    }

    pub fn add_component(&mut self, entity: EntityId, component_type: ComponentType) {
        self.ensure_changed_entity_entry(entity);

        let existing = self.changed_entities.get_mut(&entity).unwrap()
            .insert(component_type, Add);
        assert!(existing.is_none());

        self.changed_components.insert(component_type);
    }

    pub fn remove_component(&mut self, entity: EntityId, component: Component) {
        self.ensure_changed_entity_entry(entity);

        let component_type = component.to_type();

        let existing = self.changed_entities.get_mut(&entity).unwrap()
            .insert(component_type, Remove(component));

        assert!(existing.is_none());

        self.changed_components.insert(component_type);
    }

    pub fn update_spacial_hashes(&self, entities: &EntityTable) {
        let mut levels = self.levels.borrow_mut();
        levels.clear();

        for entity_id in &self.added_entities {
            let entity = entities.get(*entity_id);
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for (_, entity) in &self.removed_entities {
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for (entity_id, _) in &self.changed_entities {
            let entity = entities.get(*entity_id);
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for level_id in levels.iter() {
            let mut spacial_hash = {
                let level = entities.get(*level_id).level_data().unwrap();
                level.spacial_hash.borrow_mut()
            };
//            spacial_hash.update(self, entities);
        }
    }

    pub fn to_revert_program(mut self) -> UpdateProgram {
        let mut program = UpdateProgram::new_empty();

        for _ in self.added_entities {
            // TODO
        }

        for (_, _) in self.removed_entities.drain() {
            // TODO
        }

        for (entity, mut changed_components) in self.changed_entities.drain() {
            for (_, change) in changed_components.drain() {
                match change {
                    Set(original) => {
                        program.append(UpdateStatement::SetComponent(entity, original));
                    },
                    _ => {
                        unimplemented!();
                    },
                }
            }
        }

        program
    }
}
