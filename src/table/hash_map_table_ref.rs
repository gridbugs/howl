use table::{
    TableId,
    ToType,
    HashMapTable,
};

use std::collections::hash_map;
use std::hash::Hash;

pub struct HashMapTableMutRef<'a, EntryType, Entry>(&'a mut HashMapTable<EntryType, Entry>)
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>;

impl<'a, EntryType, Entry> HashMapTableMutRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    pub fn new(table: &'a mut HashMapTable<EntryType, Entry>) -> Self {
        HashMapTableMutRef(table)
    }

    pub fn add(&mut self, entry: Entry) -> Option<Entry> {
        self.0.add(entry)
    }

    pub fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.0.remove(t)
    }

    pub fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.0.get_mut(t)
    }
}

pub struct HashMapTableRef<'a, EntryType, Entry>(&'a HashMapTable<EntryType, Entry>)
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>;

impl<'a, EntryType, Entry> Clone for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        HashMapTableRef(self.0)
    }
}

impl<'a, EntryType, Entry> Copy for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>
{}


impl<'a, EntryType, Entry> HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    pub fn new(table: &'a HashMapTable<EntryType, Entry>) -> Self {
        HashMapTableRef(table)
    }

    pub fn has(self, entry_type: EntryType) -> bool {
        self.0.has(entry_type)
    }

    pub fn get(self, entry_type: EntryType) -> Option<&'a Entry> {
        self.0.get(entry_type)
    }

    pub fn id(self) -> Option<TableId> {
        self.0.id
    }

    pub fn slots(self) -> hash_map::Iter<'a, EntryType, Entry> {
        self.0.slots()
    }

    pub fn entries(self) -> hash_map::Values<'a, EntryType, Entry> {
        self.0.entries()
    }

    pub fn types(self) -> hash_map::Keys<'a, EntryType, Entry> {
        self.0.types()
    }
}