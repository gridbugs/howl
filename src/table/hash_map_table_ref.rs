use table::{
    TableId,
    ToType,
    Table,
    TableRef,
    IterTableRef,
    TableRefMut,
};

use std::collections::hash_map;
use std::hash::Hash;

pub struct HashMapTableRefMut<'a, EntryType, Entry>(&'a mut Table<EntryType, Entry>)
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>;

impl<'a, EntryType, Entry> HashMapTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    pub fn new(table: &'a mut Table<EntryType, Entry>) -> Self {
        HashMapTableRefMut(table)
    }
}

impl<'a, EntryType, Entry> TableRefMut<'a, EntryType, Entry> for HashMapTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    fn add(&mut self, entry: Entry) -> Option<Entry> {
        self.0.add(entry)
    }

    fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.0.remove(t)
    }

    fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.0.get_mut(t)
    }
}

pub struct HashMapTableRef<'a, EntryType, Entry>(&'a Table<EntryType, Entry>)
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

impl<'a, EntryType, Entry> TableRef<'a, EntryType, Entry> for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    fn id(self) -> Option<TableId> {
        self.0.id
    }

    fn get(self, entry_type: EntryType) -> Option<&'a Entry> {
        self.0.get(entry_type)
    }

    fn has(self, entry_type: EntryType) -> bool {
        self.0.has(entry_type)
    }
}

impl<'a, EntryType, Entry> IterTableRef<'a, EntryType, Entry> for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    type Iter = hash_map::Iter<'a, EntryType, Entry>;
    type TypeIter = hash_map::Keys<'a, EntryType, Entry>;
    type EntryIter = hash_map::Values<'a, EntryType, Entry>;

    fn slots(self) -> Self::Iter {
        self.0.slots()
    }

    fn entries(self) -> hash_map::Values<'a, EntryType, Entry> {
        self.0.entries()
    }

    fn types(self) -> hash_map::Keys<'a, EntryType, Entry> {
        self.0.types()
    }
}

impl<'a, EntryType, Entry> HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    pub fn new(table: &'a Table<EntryType, Entry>) -> Self {
        HashMapTableRef(table)
    }
}
