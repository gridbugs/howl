use game::{EntityId, Entity, ComponentType, Component, EntityWrapper, Metadata, MetadataWrapper};
use game::update::{Metadatum, MetadatumType};

use table::{ToType, TableRef, TableRefMut};

use std::collections::{HashSet, HashMap, hash_map};

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

impl<'a> MetadataWrapper<'a> for &'a UpdateSummary {
    fn get_metadata(self, md_type: MetadatumType) -> Option<&'a Metadatum> {
        self.metadata.get(md_type)
    }

    fn has_metadata(self, md_type: MetadatumType) -> bool {
        self.metadata.has(md_type)
    }
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

    pub fn add_component(&mut self, entity: EntityId, component: Component) {
        if !self.added_components.contains_key(&entity) {
            self.added_components.insert(entity, AddedComponents::new());
        }

        self.added_components.get_mut(&entity).unwrap().add(component);
    }

    pub fn remove_component(&mut self, entity: EntityId, component_type: ComponentType) {
        if !self.removed_components.contains_key(&entity) {
            self.removed_components.insert(entity, HashSet::new());
        }
        self.removed_components.get_mut(&entity).unwrap().insert(component_type);
    }

    pub fn set_metadata(&mut self, metadatum: Metadatum) {
        self.metadata.add(metadatum);
    }
}
