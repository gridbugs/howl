use std::collections::HashSet;
use std::collections::hash_set;
use ecs::entity::{Entity, EntityId, EntityTable};
use ecs::systems::schedule::Schedule;

use std::cell::RefCell;

#[derive(Debug)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub entities: HashSet<EntityId>,
    pub schedule: RefCell<Schedule>
}

impl Clone for Level { fn clone(&self) -> Level { unimplemented!(); } }

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
            width: width,
            height: height,
            entities: HashSet::new(),
            schedule: RefCell::new(Schedule::new()),
        }
    }

    pub fn add(&mut self, id: EntityId) {
        self.entities.insert(id);
    }

    pub fn entities<'a>(&'a self, entities: &'a EntityTable) -> EntityIter<'a> {
        EntityIter {
            hash_set_iter: self.entities.iter(),
            entities: entities,
        }
    }
}
