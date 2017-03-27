// Automatically generated. Do not edit.
use std::collections::{BTreeSet, btree_set};
use std::collections::{BTreeMap, btree_map};
use std::collections::{hash_set, hash_map};


use fnv::{FnvHashMap, FnvHashSet};




pub type EntityId = u64;

#[derive(Clone, Serialize, Deserialize)]
pub struct EntityBTreeSet(BTreeSet<EntityId>);

impl EntityBTreeSet {
    pub fn new() -> Self {
        EntityBTreeSet(BTreeSet::new())
    }

    pub fn insert(&mut self, entity: EntityId) -> bool {
        self.0.insert(entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> bool {
        self.0.remove(&entity)
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        self.0.contains(&entity)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn iter(&self) -> EntityBTreeSetIter {
        EntityBTreeSetIter(self.0.iter())
    }



    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn append(&mut self, other: &mut EntityBTreeSet) {
        self.0.append(&mut other.0);
    }
}

pub struct EntityBTreeSetIter<'a>(btree_set::Iter<'a, EntityId>);

impl<'a> Iterator for EntityBTreeSetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}



#[derive(Clone, Serialize, Deserialize)]
pub struct EntityBTreeMap<T>(BTreeMap<EntityId, T>);

impl<T> EntityBTreeMap<T> {
    pub fn new() -> Self {
        EntityBTreeMap(BTreeMap::new())
    }

    pub fn insert(&mut self, entity: EntityId, value: T) -> Option<T> {
        self.0.insert(entity, value)
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.0.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.0.get_mut(&entity)
    }

    pub fn contains_key(&self, entity: EntityId) -> bool {
        self.0.contains_key(&entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.0.remove(&entity)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn entry(&mut self, entity: EntityId) -> btree_map::Entry<EntityId, T> {
        self.0.entry(entity)
    }

    pub fn iter(&self) -> EntityBTreeMapIter<T> {
        EntityBTreeMapIter(self.0.iter())
    }

    pub fn keys(&self) -> EntityBTreeMapKeys<T> {
        EntityBTreeMapKeys(self.0.keys())
    }

    pub fn append(&mut self, other: &mut EntityBTreeMap<T>) {
        self.0.append(&mut other.0)
    }
}

impl<T: Copy> EntityBTreeMap<T> {
    pub fn copy_iter(&self) -> EntityBTreeMapCopyIter<T> {
        EntityBTreeMapCopyIter(self.0.iter())
    }
}

pub struct EntityBTreeMapKeys<'a, T: 'a>(btree_map::Keys<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityBTreeMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityBTreeMapIter<'a, T: 'a>(btree_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityBTreeMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, v))
    }
}

pub struct EntityBTreeMapCopyIter<'a, T: 'a + Copy>(btree_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a + Copy> Iterator for EntityBTreeMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}


type InnerHashSet = FnvHashSet<EntityId>;


#[derive(Clone, Serialize, Deserialize)]
pub struct EntityHashSet(InnerHashSet);


impl EntityHashSet {
    pub fn new() -> Self {
        EntityHashSet(InnerHashSet::default())
    }

    pub fn insert(&mut self, entity: EntityId) -> bool {
        self.0.insert(entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> bool {
        self.0.remove(&entity)
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        self.0.contains(&entity)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn drain(&mut self) -> hash_set::Drain<EntityId> {
        self.0.drain()
    }

    pub fn append(&mut self, other: &mut EntityHashSet) {
        for id in other.drain() {
            self.insert(id);
        }
    }

    pub fn iter(&self) -> EntityHashSetIter {
        EntityHashSetIter(self.0.iter())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct EntityHashSetIter<'a>(hash_set::Iter<'a, EntityId>);

impl<'a> Iterator for EntityHashSetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}



type InnerHashMap<T> = FnvHashMap<EntityId, T>;


#[derive(Clone, Serialize, Deserialize)]
pub struct EntityHashMap<T>(InnerHashMap<T>);

impl<T> EntityHashMap<T> {
    pub fn new() -> Self {
        EntityHashMap(InnerHashMap::default())
    }

    pub fn insert(&mut self, entity: EntityId, value: T) -> Option<T> {
        self.0.insert(entity, value)
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.0.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.0.get_mut(&entity)
    }

    pub fn contains_key(&self, entity: EntityId) -> bool {
        self.0.contains_key(&entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.0.remove(&entity)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn entry(&mut self, entity: EntityId) -> hash_map::Entry<EntityId, T> {
        self.0.entry(entity)
    }

    pub fn iter(&self) -> EntityHashMapIter<T> {
        EntityHashMapIter(self.0.iter())
    }

    pub fn keys(&self) -> EntityHashMapKeys<T> {
        EntityHashMapKeys(self.0.keys())
    }

    pub fn drain(&mut self) -> hash_map::Drain<EntityId, T> {
        self.0.drain()
    }

    pub fn append(&mut self, other: &mut EntityHashMap<T>) {
        for (id, value) in other.drain() {
            self.insert(id, value);
        }
    }
}

impl<T: Copy> EntityHashMap<T> {
    pub fn copy_iter(&self) -> EntityHashMapCopyIter<T> {
        EntityHashMapCopyIter(self.0.iter())
    }
}

pub struct EntityHashMapKeys<'a, T: 'a>(hash_map::Keys<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityHashMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityHashMapCopyIter<'a, T: 'a + Copy>(hash_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a + Copy> Iterator for EntityHashMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}

pub struct EntityHashMapIter<'a, T: 'a>(hash_map::Iter<'a, EntityId, T>);

impl<'a, T: 'a> Iterator for EntityHashMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id_ref, v)| (*id_ref, v))
    }
}





pub type EcsCtxEntitySet = EntityHashSet;
pub type EcsCtxEntityMap<T> = EntityHashMap<T>;
pub type EcsCtxEntitySetIter<'a> = EntityHashSetIter<'a>;
pub type EcsCtxEntityMapIter<'a, T> = EntityHashMapIter<'a, T>;
pub type EcsCtxEntityMapCopyIter<'a, T> = EntityHashMapCopyIter<'a, T>;
pub type EcsCtxEntityMapKeys<'a, T> = EntityHashMapKeys<'a, T>;

pub type EcsCtxFlagIdIter<'a> = EcsCtxEntitySetIter<'a>;





pub type EcsActionEntitySet = EntityHashSet;
pub type EcsActionEntityMap<T> = EntityHashMap<T>;
pub type EcsActionEntitySetIter<'a> = EntityHashSetIter<'a>;
pub type EcsActionEntityMapIter<'a, T> = EntityHashMapIter<'a, T>;
pub type EcsActionEntityMapCopyIter<'a, T> = EntityHashMapCopyIter<'a, T>;
pub type EcsActionEntityMapKeys<'a, T> = EntityHashMapKeys<'a, T>;

pub type EcsActionFlagIdIter<'a> = EcsActionEntitySetIter<'a>;



pub type EntitySet = EcsCtxEntitySet;
pub type EntityMap<T> = EcsCtxEntityMap<T>;
pub type EntitySetIter<'a> = EcsCtxEntitySetIter<'a>;
pub type EntityMapIter<'a, T> = EcsCtxEntityMapIter<'a, T>;
pub type EntityMapCopyIter<'a, T> = EcsCtxEntityMapCopyIter<'a, T>;
pub type EntityMapKeys<'a, T> = EcsCtxEntityMapKeys<'a, T>;
