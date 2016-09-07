use table::{
    TableId,
    ToType,
    ToIndex,
    IdTypeMap,
};

use std::collections::{
    HashMap,
};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct InvertedTableTable<EntryType, Entry>
where EntryType: Eq + Hash + Copy + ToIndex,
      Entry: ToType<EntryType>,
{
    tables: Vec<HashMap<TableId, Entry>>,
    id_types: IdTypeMap<EntryType>,
}

impl<EntryType, Entry> InvertedTableTable<EntryType, Entry>
where EntryType: Eq + Hash + Copy + ToIndex,
      Entry: ToType<EntryType>,
{
    pub fn new() -> Self {
        let num_indices = EntryType::num_indices();
        let mut tables = Vec::with_capacity(num_indices);
        for _ in 0..num_indices {
            tables.push(HashMap::new());
        }

        InvertedTableTable {
            tables: tables,
            id_types: IdTypeMap::new(),
        }
    }
}

pub struct InvertedTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>,
{
    id: TableId,
    table_table: &'a InvertedTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> InvertedTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>,
{
    fn new(id: TableId, table_table: &'a InvertedTableTable<EntryType, Entry>) -> Self {
        InvertedTableRef {
            id: id,
            table_table: table_table,
        }
    }
}

impl<'a, EntryType, Entry> Clone for InvertedTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        InvertedTableRef::new(self.id, self.table_table)
    }
}

impl<'a, EntryType, Entry> Copy for InvertedTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>
{}


pub struct InvertedTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>,
{
    id: TableId,
    table_table: &'a mut InvertedTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> InvertedTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>,
{
    fn new(id: TableId, table_table: &'a mut InvertedTableTable<EntryType, Entry>) -> Self {
        InvertedTableRefMut {
            id: id,
            table_table: table_table,
        }
    }
}

pub struct InvertedEntryAccessor<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>,
{
    entry_type: EntryType,
    hash_map: &'a HashMap<TableId, Entry>,
}

impl<'a, EntryType, Entry> InvertedEntryAccessor<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy + ToIndex,
      Entry: 'a + ToType<EntryType>,
{
    fn new(entry_type: EntryType,
           table_table: &'a InvertedTableTable<EntryType, Entry>) -> Self
    {
        InvertedEntryAccessor {
            entry_type: entry_type,
            hash_map: &table_table.tables[entry_type.to_index()],
        }
    }
}
