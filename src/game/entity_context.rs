use game::{
    EntityId,
    HashMapEntityRef,
    HashMapEntityRefMut,
    HashMapEntityTable,
    Level,
    LevelId,
    Entity,
    LevelSpacialHashMap,
    EntityTable,
};

use reserver::LeakyReserver;

use table::TableTable;

use std::collections::{
    HashSet,
    hash_set,
    HashMap,
};

use std::cell::RefCell;

pub struct EntityIter<'a, Tab: 'a + EntityTable<'a>> {
    hash_set_iter: hash_set::Iter<'a, EntityId>,
    entities: &'a Tab,
}

impl<'a, Tab: 'a + EntityTable<'a>> Iterator for EntityIter<'a, Tab> {
    type Item = (EntityId, Option<Tab::Ref>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.hash_set_iter.next() {
            Some((*id, self.entities.get(*id)))
        } else {
            None
        }
    }
}

pub struct EntityContext {
    pub entities: HashMapEntityTable,
    pub levels: HashMap<LevelId, Level>,
    level_reserver: RefCell<LeakyReserver>,
    entity_reserver: RefCell<LeakyReserver>,
}

impl EntityContext {
    pub fn new() -> Self {
        EntityContext {
            entities: HashMapEntityTable::new(),
            levels: HashMap::new(),
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

        self.levels.insert(id, level);

        id
    }

    pub fn add(&mut self, id: EntityId, mut entity: Entity) -> Option<Entity> {

        // TODO remove this
        entity.id = Some(id);

        self.entities.add(id, entity)
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(id)
    }

    pub fn get(&self, id: EntityId) -> Option<HashMapEntityRef> {
        self.entities.get(id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<HashMapEntityRefMut> {
        self.entities.get_mut(id)
    }

    pub fn id_set_iter<'a>(&'a self, set: &'a HashSet<EntityId>)
        -> EntityIter<'a, HashMapEntityTable>
    {
        EntityIter {
            hash_set_iter: set.iter(),
            entities: &self.entities,
        }
    }

    pub fn level(&self, level_id: LevelId) -> Option<&Level> {
        self.levels.get(&level_id)
    }

    pub fn level_mut(&mut self, level_id: LevelId) -> Option<&mut Level> {
        self.levels.get_mut(&level_id)
    }

    pub fn spacial_hash(&self, level_id: LevelId) -> Option<&LevelSpacialHashMap> {
        self.level(level_id).map(|level| {
            &level.spacial_hash
        })
    }
}
