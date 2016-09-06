use game::{
    IdEntityRef,
    EntityId,
    LevelSpacialHashMap,
};

use std::collections::{
    hash_set,
    HashSet,
};

pub struct EntityIter<'a, Store: 'a + EntityStore<'a>> {
    hash_set_iter: hash_set::Iter<'a, EntityId>,
    entities: &'a Store,
}

impl<'a, Store: 'a + EntityStore<'a>> Iterator for EntityIter<'a, Store> {
    type Item = (EntityId, Option<Store::Ref>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.hash_set_iter.next() {
            Some((*id, self.entities.get(*id)))
        } else {
            None
        }
    }
}


pub trait EntityStore<'a> {
    type Ref: IdEntityRef<'a>;

    fn get(&'a self, id: EntityId) -> Option<Self::Ref>;
    fn spacial_hash(&self) -> &LevelSpacialHashMap;

    fn id_set_iter(&'a self, set: &'a HashSet<EntityId>)
        -> EntityIter<'a, Self>
    where Self: Sized
    {
        EntityIter {
            hash_set_iter: set.iter(),
            entities: self,
        }
    }
}
