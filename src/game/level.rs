use std::collections::HashSet;
use std::collections::hash_set;
use ecs::entity::{Entity, EntityId};
use ecs::ecs_context::EcsContext;

#[derive(Debug)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub entities: HashSet<EntityId>,
}

pub struct EntityIter<'a> {
    hash_set_iter: hash_set::Iter<'a, EntityId>,
    ecs: &'a EcsContext,
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = &'a Entity;
    fn next(&mut self) -> Option<Self::Item> {
        self.hash_set_iter.next().map(|id| {
            self.ecs.get(*id)
        })
    }
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        Level {
            width: width,
            height: height,
            entities: HashSet::new(),
        }
    }

    pub fn add(&mut self, id: EntityId) {
        self.entities.insert(id);
    }

    pub fn entities<'a>(&'a self, ecs: &'a EcsContext) -> EntityIter<'a> {
        EntityIter {
            hash_set_iter: self.entities.iter(),
            ecs: ecs,
        }
    }
}
