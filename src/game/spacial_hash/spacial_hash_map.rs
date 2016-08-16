use game::{
    Entity,
    EntityId,
    Component,
    ComponentType,
    EntityTable,
    UpdateSummary,
    ToType,
};

use std::collections::HashMap;
use std::collections::HashSet;

use grid::StaticGrid;
use geometry::Vector2;

#[derive(Debug, Clone)]
pub struct SpacialHashCell {
    pub entities: HashSet<EntityId>,
    pub components: HashMap<ComponentType, usize>,
    pub opacity: f64,
}

impl Default for SpacialHashCell {
    fn default() -> Self {
        SpacialHashCell {
            entities: HashSet::new(),
            components: HashMap::new(),
            opacity: 0.0,
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

    fn add_entity(&mut self, entity: &Entity) {
        if self.entities.insert(entity.id.unwrap()) {
            for component in entity.slots.values() {
                self.add_component(entity, component);
            }
        }
    }

    fn remove_entity(&mut self, entity: &Entity) {
        if self.entities.remove(&entity.id.unwrap()) {
            for component in entity.slots.values() {
                self.remove_component(entity, component);
            }
        }
    }

    fn change_component(&mut self, _: &Entity, old: &Component, new: &Component) {
        if let &Component::Opacity(old_opacity) = old {
            if let &Component::Opacity(new_opacity) = new {
                self.opacity += new_opacity - old_opacity;
            }
        }
    }

    fn add_component(&mut self, _: &Entity, component: &Component) {
        self.increment_count(component.to_type());

        if let &Component::Opacity(opacity) = component {
            self.opacity += opacity;
        }
    }

    fn remove_component(&mut self , _: &Entity, component: &Component) {
        self.decrement_count(component.to_type());

        if let &Component::Opacity(opacity) = component {
            self.opacity -= opacity;
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
            cell.add_entity(entity);
        }
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        if let Some(vec) = entity.position() {
            let cell = self.get_mut(vec.to_tuple()).unwrap();
            cell.remove_entity(entity);
        }
    }

    pub fn add_components(&mut self, entity: &Entity, changes: &Entity) {

        // position will be set to the position of entity after the change
        let position = if let Some(new_position) = changes.position() {

            // position is special as it indicates which cell to update
            if let Some(old_position) = entity.position() {
                // entity is moving from old_position to new_position
                self.get_mut(old_position.to_tuple()).unwrap().remove_entity(entity);
            }

            // the entity's position is changing or the entity is gaining a position
            // in either case, add the entity to the position's cell
            self.get_mut(new_position.to_tuple()).unwrap().add_entity(entity);

            // entity will eventually end up here
            Some(new_position)
        } else if let Some(current_position) = entity.position() {
            // entity isn't moving, so use its current position
            Some(current_position)
        } else {
            // entity has no position, so the spacial hash won't be updated
            None
        };

        if let Some(position) = position {
            let mut cell = self.get_mut(position.to_tuple()).unwrap();
            for (component_type, new_component) in &changes.slots {
                if *component_type == ComponentType::Position {
                    // this has already been handled
                    continue;
                }

                if let Some(ref old_component) = entity.get(*component_type) {
                    cell.change_component(entity, old_component, new_component);
                } else {
                    // only update the component count if the component is being added
                    cell.add_component(entity, new_component);
                }
            }
        }
    }

    pub fn remove_components(&mut self, entity: &Entity, component_types: &HashSet<ComponentType>) {
        if let Some(position) = entity.position() {
            if component_types.contains(&ComponentType::Position) {
                // removing position - remove the entity
                self.remove_entity(entity);
            } else {
                let mut cell = self.get_mut(position.to_tuple()).unwrap();
                for component_type in component_types {
                    if let Some(ref component) = entity.get(*component_type) {
                        cell.remove_component(entity, component);
                    }
                }
            }
        }
    }

    /// Update the spacial hash's metadata. This should be called before the update is applied.
    pub fn update(&mut self, update: &UpdateSummary, entities: &EntityTable) {
        for entity in update.added_entities.values() {
            if self.entity_is_on_level(entity) {
                self.add_entity(entity);
            }
        }

        for entity_id in &update.removed_entities {
            let entity = entities.get(*entity_id);
            if self.entity_is_on_level(entity) {
                self.remove_entity(entity);
            }
        }

        for (entity_id, changes) in &update.added_components {
            let entity = entities.get(*entity_id);
            if self.entity_is_on_level(entity) {
                self.add_components(entity, changes);
            }
        }

        for (entity_id, component_types) in &update.removed_components {
            let entity = entities.get(*entity_id);
            if self.entity_is_on_level(entity) {
                self.remove_components(entity, component_types);
            }
        }
    }
}
