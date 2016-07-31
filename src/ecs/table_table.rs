use ecs::table::{TableId, Table, ToType};
use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct TableTable<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    next_id: TableId,
    tables: HashMap<TableId, Table<EntryType, Entry>>,
}

impl<EntryType, Entry> TableTable<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    pub fn new() -> Self {
        TableTable {
            next_id: 0,
            tables: HashMap::new(),
        }
    }

    pub fn add(&mut self, mut table: Table<EntryType, Entry>) -> TableId {
        let id = self.next_id;
        self.next_id += 1;

        table.id = Some(id);
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

