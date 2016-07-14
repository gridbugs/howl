use ecs::entity::{EntityId, Entity};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EcsContext {
    next_entity_id: EntityId,
    entities: HashMap<EntityId, Entity>,
}

impl EcsContext {
    pub fn new() -> EcsContext {
        EcsContext {
            next_entity_id: 0,
            entities: HashMap::new(),
        }
    }

    pub fn add(&mut self, mut entity: Entity) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        entity.id = Some(id);
        self.entities.insert(id, entity);

        id
    }

    pub fn get(&self, id: EntityId) -> &Entity {
        self.entities.get(&id).unwrap()
    }

    pub fn get_mut(&mut self, id: EntityId) -> &mut Entity {
        self.entities.get_mut(&id).unwrap()
    }
}
