use game::{
    EntityId,
    Entity,
    ComponentType,
    Component,
    EntityContext,
    LevelId,
    EntityWrapper,
};
use game::update::{
    Metadata,
    Metadatum,
    MetadatumType,
};

use table::{
    ToType,
    TableRef,
    TableRefMut,
};

use std::collections::{
    HashSet,
    HashMap,
    hash_map,
};

#[derive(Clone)]
pub struct AddedComponents(HashMap<ComponentType, Component>);

impl<'a> EntityWrapper<'a> for &'a AddedComponents {
    fn get_component(self, component_type: ComponentType) -> Option<&'a Component> {
        self.0.get(&component_type)
    }

    fn has_component(self, component_type: ComponentType) -> bool {
        self.0.contains_key(&component_type)
    }
}

impl AddedComponents {
    pub fn new() -> Self {
        AddedComponents(HashMap::new())
    }

    pub fn from_entity(mut entity: Entity) -> Self {
        let mut ret = Self::new();

        for (_, component) in entity.slots.drain() {
            ret.add(component)
        }

        ret
    }

    pub fn add(&mut self, component: Component) {
        self.0.insert(component.to_type(), component);
    }

    pub fn iter(&self) -> hash_map::Iter<ComponentType, Component> {
        self.0.iter()
    }

    pub fn drain(&mut self) -> hash_map::Drain<ComponentType, Component> {
        self.0.drain()
    }

    pub fn get(&self, component_type: ComponentType) -> Option<&Component> {
        self.get_component(component_type)
    }

    pub fn has(&self, component_type: ComponentType) -> bool {
        self.has_component(component_type)
    }
}

#[derive(Clone)]
pub struct UpdateSummary {
    pub added_entities: HashMap<EntityId, Entity>,
    pub removed_entities: HashSet<EntityId>,
    pub added_components: HashMap<EntityId, AddedComponents>,
    pub removed_components: HashMap<EntityId, HashSet<ComponentType>>,
    pub metadata: Metadata,
}

impl UpdateSummary {
    pub fn new() -> Self {
        UpdateSummary {
            added_entities: HashMap::new(),
            removed_entities: HashSet::new(),
            added_components: HashMap::new(),
            removed_components: HashMap::new(),
            metadata: Metadata::new(),
        }
    }

    pub fn add_entity(&mut self, id: EntityId, entity: Entity) {
        self.added_entities.insert(id, entity);
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        self.removed_entities.insert(entity);
    }

    pub fn add_component(&mut self, entity: EntityId,
                         component: Component)
    {
        if !self.added_components.contains_key(&entity) {
            self.added_components.insert(entity, AddedComponents::new());
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

    pub fn commit(mut self, entities: &mut EntityContext, level_id: LevelId, turn_count: u64) {

        let mut level = entities.level_mut(level_id).unwrap();

        level.update_spatial_hash(&self, turn_count);

        for (id, entity) in self.added_entities.drain() {
            level.add(id, entity);
        }

        for entity_id in self.removed_entities.iter() {
            level.remove(*entity_id).
                expect("Tried to remove non-existent entity.");
        }

        for (entity_id, mut components) in self.added_components.drain() {
            let mut entity = level.get_mut(entity_id).unwrap();
            for (_, component) in components.drain() {
                entity.add(component);
            }
        }

        for (entity_id, component_types) in self.removed_components.iter() {
            let mut entity = level.get_mut(*entity_id).unwrap();
            for component_type in component_types.iter() {
                entity.remove(*component_type);
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
