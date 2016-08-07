use std::collections::HashSet;
use std::collections::hash_set;
use game::entity::{Entity, EntityId, EntityTable};
use game::schedule::Schedule;

use game::spacial_hash::SpacialHashMap;

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Level {
    pub id: Option<EntityId>,
    pub width: usize,
    pub height: usize,
    pub entities: HashSet<EntityId>,
    pub schedule: RefCell<Schedule>,
    pub spacial_hash: RefCell<SpacialHashMap>,
}

pub struct EntityIter<'a> {
    hash_set_iter: hash_set::Iter<'a, EntityId>,
    entities: &'a EntityTable,
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = &'a Entity;
    fn next(&mut self) -> Option<Self::Item> {
        self.hash_set_iter.next().map(|id| {
            self.entities.get(*id)
        })
    }
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        Level {
            id: None,
            width: width,
            height: height,
            entities: HashSet::new(),
            schedule: RefCell::new(Schedule::new()),
            spacial_hash: RefCell::new(SpacialHashMap::new()),
        }
    }

    pub fn set_id(&mut self, id: EntityId) {
        self.id = Some(id);
        self.spacial_hash.borrow_mut().set_id(id);
    }

    pub fn add(&mut self, id: EntityId) {
        self.entities.insert(id);
    }

    // Makes the bookkeeping info reflect the contents of entities
    pub fn finalise(&self, entities: &EntityTable) {
        let mut spacial_hash = self.spacial_hash.borrow_mut();
        for entity in self.entities(entities) {
            spacial_hash.add_entity(entity);
        }
    }

    pub fn entities<'a>(&'a self, entities: &'a EntityTable) -> EntityIter<'a> {
        EntityIter {
            hash_set_iter: self.entities.iter(),
            entities: entities,
        }
    }
}
