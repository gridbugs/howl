use table::TableId;

use std::collections::{HashMap, HashSet, hash_set};

use std::hash::Hash;

pub struct EntryTypeIter<'a, EntryType>(Option<hash_set::Iter<'a, EntryType>>)
    where EntryType: 'a + Eq + Hash + Copy;

impl<'a, EntryType> Iterator for EntryTypeIter<'a, EntryType>
    where EntryType: 'a + Eq + Hash + Copy
{
    type Item = &'a EntryType;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.0 {
            iter.next()
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct IdTypeMap<EntryType>(HashMap<TableId, HashSet<EntryType>>)
    where EntryType: Eq + Hash + Copy;

impl<EntryType> IdTypeMap<EntryType>
    where EntryType: Eq + Hash + Copy
{
    pub fn new() -> Self {
        IdTypeMap(HashMap::new())
    }

    pub fn get(&self, id: TableId) -> Option<&HashSet<EntryType>> {
        self.0.get(&id)
    }

    fn ensure_entry_type_map(&mut self, id: TableId) {
        if !self.0.contains_key(&id) {
            self.0.insert(id, HashSet::new());
        }
    }

    pub fn add(&mut self, id: TableId, entry_type: EntryType) {
        self.ensure_entry_type_map(id);
        self.0.get_mut(&id).unwrap().insert(entry_type);
    }

    pub fn remove(&mut self, id: TableId, entry_type: EntryType) {
        self.ensure_entry_type_map(id);
        self.0.get_mut(&id).unwrap().remove(&entry_type);
    }

    pub fn types(&self, id: TableId) -> EntryTypeIter<EntryType> {
        EntryTypeIter(self.get(id).map(|s| s.iter()))
    }

    pub fn remove_types(&mut self, id: TableId) -> Option<HashSet<EntryType>> {
        self.0.remove(&id)
    }

    pub fn contains_id(&self, id: TableId) -> bool {
        self.0.contains_key(&id)
    }
}
