use clear::Clear;

use std::collections::HashMap;
use std::collections::hash_map;
use std::hash::Hash;

pub trait ToType<EntryType> {
    fn to_type(&self) -> EntryType;
}

pub type TableId = u64;

#[derive(Debug, Clone)]
pub struct Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    pub id: Option<TableId>,
    pub slots: HashMap<EntryType, Entry>,
}

impl<EntryType, Entry> Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    pub fn new() -> Table<EntryType, Entry> {
        Table {
            id: None,
            slots: HashMap::new(),
        }
    }

    pub fn id(&self) -> Option<TableId> {
        self.id
    }

    pub fn add(&mut self, entry: Entry) -> Option<Entry> {
        self.slots.insert(entry.to_type(), entry)
    }

    pub fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.slots.remove(&t)
    }

    pub fn get(&self, t: EntryType) -> Option<&Entry> {
        self.slots.get(&t)
    }

    pub fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.slots.get_mut(&t)
    }

    pub fn has(&self, t: EntryType) -> bool {
        self.slots.contains_key(&t)
    }

    pub fn slots<'a>(&'a self) -> hash_map::Iter<'a, EntryType, Entry> {
        self.slots.iter()
    }

    pub fn entries<'a>(&'a self) -> hash_map::Values<'a, EntryType, Entry> {
        self.slots.values()
    }

    pub fn types<'a>(&'a self) -> hash_map::Keys<'a, EntryType, Entry> {
        self.slots.keys()
    }
}

impl<EntryType, Entry> Clear for Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    fn clear(&mut self) {
        self.slots.clear();
    }
}

impl<EntryType, Entry> Default for Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    fn default() -> Self {
        Table::new()
    }
}
