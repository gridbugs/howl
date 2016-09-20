use table::TableId;

use std::collections::{HashMap, HashSet, hash_set};

use std::hash::Hash;

pub struct TableIdIter<'a>(Option<hash_set::Iter<'a, TableId>>);

impl<'a> Iterator for TableIdIter<'a> {
    type Item = &'a TableId;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.0 {
            iter.next()
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeIdMap<EntryType>(HashMap<EntryType, HashSet<TableId>>)
    where EntryType: Eq + Hash + Copy;

impl<EntryType> TypeIdMap<EntryType>
    where EntryType: Eq + Hash + Copy
{
    pub fn new() -> Self {
        TypeIdMap(HashMap::new())
    }

    fn get(&self, entry_type: EntryType) -> Option<&HashSet<TableId>> {
        self.0.get(&entry_type)
    }

    fn ensure_entry_type_map(&mut self, entry_type: EntryType) {
        if !self.0.contains_key(&entry_type) {
            self.0.insert(entry_type, HashSet::new());
        }
    }

    pub fn add(&mut self, id: TableId, entry_type: EntryType) {
        self.ensure_entry_type_map(entry_type);
        self.0.get_mut(&entry_type).unwrap().insert(id);
    }

    pub fn remove(&mut self, id: TableId, entry_type: EntryType) {
        self.ensure_entry_type_map(entry_type);
        self.0.get_mut(&entry_type).unwrap().remove(&id);
    }


    pub fn ids(&self, entry_type: EntryType) -> TableIdIter {
        TableIdIter(self.get(entry_type).map(|s| s.iter()))
    }
}
