use table::{
    TableId,
};

use std::collections::{
    HashMap,
    HashSet,
};

use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct TypeIdMap<EntryType>(HashMap<EntryType, HashSet<TableId>>)
where EntryType: Eq + Hash + Copy;

impl<EntryType> TypeIdMap<EntryType>
where EntryType: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        TypeIdMap(HashMap::new())
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

    pub fn get(&self, entry_type: EntryType) -> Option<&HashSet<TableId>> {
        self.0.get(&entry_type)
    }
}
