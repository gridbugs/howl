use table::table::{
    TableId,
    Table,
    ToType,
};

use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct TableTable<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    next_id: Cell<TableId>,
    tables: HashMap<TableId, Table<EntryType, Entry>>,
}

impl<EntryType, Entry> TableTable<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    pub fn new() -> Self {
        TableTable {
            next_id: Cell::new(0),
            tables: HashMap::new(),
        }
    }

    pub fn reserve_id(&self) -> TableId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }

    pub fn add(&mut self, mut table: Table<EntryType, Entry>) -> TableId {

        let id = if let Some(id) = table.id {
            id
        } else {
            let id = self.reserve_id();
            table.id = Some(id);
            id
        };

        self.tables.insert(id, table);

        id
    }

    pub fn remove(&mut self, id: TableId) -> Option<Table<EntryType, Entry>> {
        self.tables.remove(&id)
    }

    pub fn get(&self, id: TableId) -> &Table<EntryType, Entry> {
        self.tables.get(&id).unwrap()
    }

    pub fn get_mut(&mut self, id: TableId) -> &mut Table<EntryType, Entry> {
        self.tables.get_mut(&id).unwrap()
    }

    pub fn tables(&self) -> hash_map::Values<TableId, Table<EntryType, Entry>> {
        self.tables.values()
    }
}

