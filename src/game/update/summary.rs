use game::{
    EntityId,
    Entity,
    ComponentType,
    Component,
    EntityContext,
    EntityTable,
    LevelId,
    Level,
};
use game::update::{
    Metadata,
    Metadatum,
    MetadatumType,
};

use std::collections::HashSet;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Clone)]
pub struct UpdateSummary {
    pub added_entities: HashMap<EntityId, Entity>,
    pub removed_entities: HashSet<EntityId>,
    pub added_components: HashMap<EntityId, Entity>,
    pub removed_components: HashMap<EntityId, HashSet<ComponentType>>,
    levels: RefCell<HashSet<EntityId>>,
    pub metadata: Metadata,
}

impl UpdateSummary {
    pub fn new() -> Self {
        UpdateSummary {
            added_entities: HashMap::new(),
            removed_entities: HashSet::new(),
            added_components: HashMap::new(),
            removed_components: HashMap::new(),
            levels: RefCell::new(HashSet::new()),
            metadata: Metadata::new(),
        }
    }

    pub fn add_entity(&mut self, id: EntityId, mut entity: Entity) {
        entity.id = Some(id);
        self.added_entities.insert(id, entity);
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

    pub fn commit(mut self, entities: &mut EntityContext, turn_count: u64) {

        self.update_spacial_hashes(&mut entities.levels, &entities.entities, turn_count);

        for (_, entity) in self.added_entities.drain() {
            entities.add(entity);
        }

        for entity_id in self.removed_entities.drain() {
            entities.remove(entity_id).
                expect("Tried to remove non-existent entity.");
        }

        for (entity_id, mut changes) in self.added_components.drain() {
            let mut entity = entities.get_mut(entity_id).unwrap();
            for (_, component) in changes.slots.drain() {
                entity.add(component);
            }
        }

        for (entity_id, mut component_types) in self.removed_components.drain() {
            let mut entity = entities.get_mut(entity_id).unwrap();
            for component_type in component_types.drain() {
                entity.remove(component_type);
            }
        }
    }

    fn update_spacial_hashes(&self, levels: &mut HashMap<LevelId, Level>, entities: &EntityTable, turn_count: u64) {
        self.update_levels(entities);

        for level_id in self.levels.borrow().iter() {
            let mut level = levels.get_mut(level_id).unwrap();
            level.spacial_hash.update(self, entities, turn_count);
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
            let entity = entities.get(*entity_id).unwrap();
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for entity_id in self.added_components.keys() {
            let entity = entities.get(*entity_id).unwrap();
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }

        for entity_id in self.removed_components.keys() {
            let entity = entities.get(*entity_id).unwrap();
            if let Some(level) = entity.on_level() {
                levels.insert(level);
            }
        }
    }

    pub fn set_metadata(&mut self, metadatum: Metadatum) {
        self.metadata.add(metadatum);
    }

    pub fn get_metadata(&self, metadatum_type: MetadatumType) -> Option<&Metadatum> {
        self.metadata.get(metadatum_type)
    }

    pub fn has_metadata(&self, metadatum_type: MetadatumType) ->  bool {
        self.metadata.has(metadatum_type)
    }
}
