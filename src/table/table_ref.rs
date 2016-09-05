use table::{
    Table,
    TableId,
    ToType,
};

use std::collections::hash_map;
use std::hash::Hash;

pub struct TableRef<'a, EntryType, Entry>(&'a Table<EntryType, Entry>)
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>;

impl<'a, EntryType, Entry> Clone for TableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        TableRef(self.0)
    }
}

impl<'a, EntryType, Entry> Copy for TableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>
{}


impl<'a, EntryType, Entry> TableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{

    // TODO: remove this
    pub fn new(table: &'a Table<EntryType, Entry>) -> Self {
        TableRef(table)
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
