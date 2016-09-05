use table::{
    TableId,
    ToType,
    HashMapTable,
    HashMapTableRef,
    HashMapTableMutRef,
};

use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct HashMapTableTable<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    next_id: Cell<TableId>,
    tables: HashMap<TableId, HashMapTable<EntryType, Entry>>,
}

impl<EntryType, Entry> HashMapTableTable<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    pub fn new() -> Self {
        HashMapTableTable {
            next_id: Cell::new(0),
            tables: HashMap::new(),
        }
    }

    pub fn reserve_id(&self) -> TableId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }

    pub fn add(&mut self, mut table: HashMapTable<EntryType, Entry>) -> TableId {

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

    pub fn remove(&mut self, id: TableId) -> Option<HashMapTable<EntryType, Entry>> {
        self.tables.remove(&id)
    }

    pub fn get(&self, id: TableId) -> Option<HashMapTableRef<EntryType, Entry>> {
        self.tables.get(&id).map(HashMapTableRef::new)
    }

    pub fn get_mut(&mut self, id: TableId) -> Option<HashMapTableMutRef<EntryType, Entry>> {
        self.tables.get_mut(&id).map(HashMapTableMutRef::new)
    }

    pub fn tables(&self) -> hash_map::Values<TableId, HashMapTable<EntryType, Entry>> {
        self.tables.values()
    }
}
