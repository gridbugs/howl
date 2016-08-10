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
            spacial_hash.update(self, entities);
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
