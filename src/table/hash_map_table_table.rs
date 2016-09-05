use table::{
    TableId,
    ToType,
    Table,
    TableTable,
    HashMapTableRef,
    HashMapTableRefMut,
};

use std::collections::HashMap;
use std::hash::Hash;
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct HashMapTableTable<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    next_id: Cell<TableId>,
    tables: HashMap<TableId, Table<EntryType, Entry>>,
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
}

impl<'a, EntryType, Entry> TableTable<'a, EntryType, Entry>
for HashMapTableTable<EntryType, Entry>
where EntryType: 'a + Eq + Hash,
      Entry: 'a + ToType<EntryType>,
{
    type Ref = HashMapTableRef<'a, EntryType, Entry>;
    type RefMut = HashMapTableRefMut<'a, EntryType, Entry>;

    fn add(&mut self, table_id: TableId, mut table: Table<EntryType, Entry>)
        -> Option<Table<EntryType, Entry>>
    {
        let id = if let Some(id) = table.id {
            assert_eq!(table_id, id);
            id
        } else {
            table.id = Some(table_id);
            table_id
        };

        self.tables.insert(id, table)
    }

    fn remove(&mut self, id: TableId) -> Option<Table<EntryType, Entry>> {
        self.tables.remove(&id)
    }

    fn get(&'a self, id: TableId) -> Option<Self::Ref> {
        self.tables.get(&id).map(HashMapTableRef::new)
    }

    fn get_mut(&'a mut self, id: TableId) -> Option<Self::RefMut> {
        self.tables.get_mut(&id).map(HashMapTableRefMut::new)
    }
}
