use clear::Clear;
use table::{ToType, TableRef, IterTableRef, TableRefMut};

use std::collections::HashMap;
use std::collections::hash_map;
use std::hash::Hash;

pub type TableId = u64;

#[derive(Debug, Clone)]
pub struct Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>
{
    pub slots: HashMap<EntryType, Entry>,
}

impl<EntryType, Entry> Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>
{
    pub fn new() -> Self {
        Table { slots: HashMap::new() }
    }
}

impl<'a, EntryType, Entry> TableRef<'a, EntryType, Entry> for &'a Table<EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    fn get(self, t: EntryType) -> Option<&'a Entry> {
        self.slots.get(&t)
    }

    fn has(self, t: EntryType) -> bool {
        self.slots.contains_key(&t)
    }
}

impl<'a, EntryType, Entry> IterTableRef<'a, EntryType, Entry> for &'a Table<EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    type Iter = hash_map::Iter<'a, EntryType, Entry>;
    type TypeIter = hash_map::Keys<'a, EntryType, Entry>;
    type EntryIter = hash_map::Values<'a, EntryType, Entry>;

    fn slots(self) -> Self::Iter {
        self.slots.iter()
    }

    fn entries(self) -> Self::EntryIter {
        self.slots.values()
    }

    fn types(self) -> Self::TypeIter {
        self.slots.keys()
    }
}

impl<'a, EntryType, Entry> TableRefMut<'a, EntryType, Entry> for Table<EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    fn add(&mut self, entry: Entry) -> Option<Entry> {
        self.slots.insert(entry.to_type(), entry)
    }

    fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.slots.remove(&t)
    }

    fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.slots.get_mut(&t)
    }
}

impl<'a, EntryType, Entry> TableRefMut<'a, EntryType, Entry> for &'a mut Table<EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    fn add(&mut self, entry: Entry) -> Option<Entry> {
        self.slots.insert(entry.to_type(), entry)
    }

    fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.slots.remove(&t)
    }

    fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.slots.get_mut(&t)
    }
}



impl<EntryType, Entry> Clear for Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>
{
    fn clear(&mut self) {
        self.slots.clear();
    }
}

impl<EntryType, Entry> Default for Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>
{
    fn default() -> Self {
        Table::new()
    }
}
