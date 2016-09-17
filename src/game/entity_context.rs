use game::{
    EntityId,
    Level,
    LevelId,
    Entity,
    EntityStore,
    LevelEntityRef,
    LevelEntityRefMut,
};

use reserver::LeakyReserver;

use std::cell::RefCell;

pub struct EntityContext {
    pub levels: Vec<Option<Level>>,
    pub entity_ids: RefCell<LeakyReserver<EntityId>>,
}

pub trait ReserveEntityId {
    fn reserve_entity_id(&self) -> EntityId;
}

impl ReserveEntityId for RefCell<LeakyReserver<EntityId>> {
    fn reserve_entity_id(&self) -> EntityId {
        self.borrow_mut().reserve()
    }
}

impl ReserveEntityId for EntityContext {
    fn reserve_entity_id(&self) -> EntityId {
        self.entity_ids.reserve_entity_id()
    }
}

pub trait LevelStore {
    fn level(&self, level_id: LevelId) -> Option<&Level>;
    fn level_mut(&mut self, level_id: LevelId) -> Option<&mut Level>;
}

impl LevelStore for Vec<Option<Level>> {
    fn level(&self, level_id: LevelId) -> Option<&Level> {
        self[level_id].as_ref()
    }

    fn level_mut(&mut self, level_id: LevelId) -> Option<&mut Level> {
        self[level_id].as_mut()
    }
}

impl LevelStore for EntityContext {
    fn level(&self, level_id: LevelId) -> Option<&Level> {
        self.levels.level(level_id)
    }

    fn level_mut(&mut self, level_id: LevelId) -> Option<&mut Level> {
        self.levels.level_mut(level_id)
    }
}

impl EntityContext {
    pub fn new() -> Self {
        EntityContext {
            levels: Vec::new(),
            entity_ids: RefCell::new(LeakyReserver::new()),
        }
    }

    pub fn reserve_level_id(&mut self) -> LevelId {
        let id = self.levels.len();
        self.levels.push(None);
        id
    }

    pub fn add_level(&mut self, level: Level) {
        let id = level.id();
        self.levels[id] = Some(level);
    }

    pub fn add(&mut self, id: EntityId, level_id: LevelId, entity: Entity) -> Option<Entity> {
        self.level_mut(level_id).unwrap().add(id, entity)
    }

    pub fn remove(&mut self, id: EntityId, level_id: LevelId) -> Option<Entity> {
        self.level_mut(level_id).unwrap().remove(id)
    }

    pub fn get_mut(&mut self, id: EntityId, level_id: LevelId) -> Option<LevelEntityRefMut> {
        self.level_mut(level_id).unwrap().get_mut(id)
    }

    pub fn get_from_level(&self, id: EntityId, level_id: LevelId) -> Option<LevelEntityRef> {
        self.level(level_id).unwrap().get(id)
    }
}
