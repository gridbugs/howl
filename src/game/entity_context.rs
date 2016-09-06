use game::{
    EntityId,
    HashMapEntityRef,
    HashMapEntityRefMut,
    Level,
    LevelId,
    Entity,
};

use reserver::LeakyReserver;

use table::TableTable;

use std::cell::RefCell;

pub struct EntityContext {
    pub levels: Vec<Level>,
    level_reserver: RefCell<LeakyReserver<LevelId>>,
    entity_reserver: RefCell<LeakyReserver<EntityId>>,
}

impl EntityContext {
    pub fn new() -> Self {
        EntityContext {
            levels: Vec::new(),
            level_reserver: RefCell::new(LeakyReserver::new()),
            entity_reserver: RefCell::new(LeakyReserver::new()),
        }
    }

    pub fn reserve_level_id(&self) -> LevelId {
        self.level_reserver.borrow_mut().reserve()
    }

    pub fn reserve_entity_id(&self) -> EntityId {
        self.entity_reserver.borrow_mut().reserve()
    }

    pub fn add_level(&mut self, mut level: Level) -> LevelId {

        let id = if let Some(id) = level.id {
            id
        } else {
            let id = self.reserve_level_id();
            level.id = Some(id);
            id
        };

        self.levels.push(level);

        id
    }

    pub fn add(&mut self, id: EntityId, level_id: LevelId, entity: Entity) -> Option<Entity> {
        self.levels[level_id].entities.add(id, entity)
    }

    pub fn remove(&mut self, id: EntityId, level_id: LevelId) -> Option<Entity> {
        self.levels[level_id].entities.remove(id)
    }

    pub fn get_mut(&mut self, id: EntityId, level_id: LevelId) -> Option<HashMapEntityRefMut> {
        self.levels[level_id].entities.get_mut(id)
    }

    pub fn get_from_level(&self, id: EntityId, level_id: LevelId) -> Option<HashMapEntityRef> {
        self.levels[level_id].entities.get(id)
    }

    pub fn level(&self, level_id: LevelId) -> Option<&Level> {
        Some(&self.levels[level_id])
    }

    pub fn level_mut(&mut self, level_id: LevelId) -> Option<&mut Level> {
        Some(&mut self.levels[level_id])
    }
}
