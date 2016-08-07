use ecs::table::ToType;
use ecs::entity::{
    EntityId,
    Entity,
    ComponentType,
    Component,
    EntityTable,
};
use ecs::update_monad::{UpdateMonad, Action};

use game::game_entity::GameEntity;

use std::collections::HashSet;
use std::collections::HashMap;
use std::cell::RefCell;

use std::mem;

pub fn set_entity_component(summary: &mut UpdateSummary,
                            entities: &mut EntityTable,
                            entity_id: EntityId,
                            new_component: Component)
{
    let mut entity = entities.get_mut(entity_id);

    if let Some(current_component) = entity.get_mut(new_component.to_type()) {
        let original_component = mem::replace(current_component, new_component);
        summary.change_entity(entity_id, original_component);
    } else {
        panic!("No component of type {:?} found.", new_component.to_type());
    }
}



#[derive(Debug)]
pub struct UpdateSummary {
    pub added_entities: HashSet<EntityId>,
    pub removed_entities: HashMap<EntityId, Entity>,
    pub changed_entities: HashMap<EntityId, HashMap<ComponentType, Component>>,
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

    pub fn change_entity(&mut self, entity: EntityId, old_component: Component) {
        if !self.changed_entities.contains_key(&entity) {
            self.changed_entities.insert(entity, HashMap::new());
        }

        let component_type = old_component.to_type();
        self.changed_entities.get_mut(&entity).unwrap().insert(component_type, old_component);
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

    pub fn to_revert_action(mut self) -> Action {
        let mut action = UpdateMonad::ret(());

        for _ in self.added_entities {
            // TODO
        }

        for (_, _) in self.removed_entities.drain() {
            // TODO
        }

        for (entity, mut changed_components) in self.changed_entities.drain() {
            for (_, component) in changed_components.drain() {
                action = action.bind(move |()| {
                    let intermediate = component.clone();
                    UpdateMonad::new(move |summary, entities| {
                        set_entity_component(summary, entities, entity, intermediate.clone());
                    })
                });
            }
        }

        action
    }
}
