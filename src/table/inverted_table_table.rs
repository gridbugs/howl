use table::{TableId, ToType, ToIndex, IdTypeMap, TableRef, TableRefMut, IterTableRef,
            EntryTypeIter, IdTableRef, EntryAccessor, TableTable, Table};

use std::collections::{HashMap, hash_map};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct InvertedTableTable<EntryType, Entry>
    where EntryType: Eq + Hash + Copy + ToIndex,
          Entry: ToType<EntryType>
{
    tables: Vec<HashMap<TableId, Entry>>,
    id_types: IdTypeMap<EntryType>,
}

impl<EntryType, Entry> InvertedTableTable<EntryType, Entry>
    where EntryType: Eq + Hash + Copy + ToIndex,
          Entry: ToType<EntryType>
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

    fn get_table(&self, entry_type: EntryType) -> &HashMap<TableId, Entry> {
        &self.tables[entry_type.to_index()]
    }

    fn get_table_mut(&mut self, entry_type: EntryType) -> &mut HashMap<TableId, Entry> {
        &mut self.tables[entry_type.to_index()]
    }
}

pub struct InvertedTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    id: TableId,
    table_table: &'a InvertedTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> InvertedTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
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
          Entry: 'a + ToType<EntryType>
{
    fn clone(&self) -> Self {
        InvertedTableRef::new(self.id, self.table_table)
    }
}

impl<'a, EntryType, Entry> Copy for InvertedTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
}


pub struct InvertedTableRefMut<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    id: TableId,
    table_table: &'a mut InvertedTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> InvertedTableRefMut<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
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
          Entry: 'a + ToType<EntryType>
{
    entry_type: EntryType,
    hash_map: &'a HashMap<TableId, Entry>,
}

impl<'a, EntryType, Entry> InvertedEntryAccessor<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    fn new(entry_type: EntryType, hash_map: &'a HashMap<TableId, Entry>) -> Self {
        InvertedEntryAccessor {
            entry_type: entry_type,
            hash_map: hash_map,
        }
    }
}

impl<'a, EntryType, Entry> TableRef<'a, EntryType, Entry> for InvertedTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    fn get(self, entry_type: EntryType) -> Option<&'a Entry> {
        self.table_table.get_table(entry_type).get(&self.id)
    }

    fn has(self, entry_type: EntryType) -> bool {
        self.table_table.get_table(entry_type).contains_key(&self.id)
    }
}

impl<'a, EntryType, Entry> TableRefMut<'a, EntryType, Entry>
    for InvertedTableRefMut<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    fn add(&mut self, entry: Entry) -> Option<Entry> {
        let entry_type = entry.to_type();
        self.table_table.id_types.add(self.id, entry_type);
        self.table_table.get_table_mut(entry_type).insert(self.id, entry)
    }

    fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.table_table.id_types.remove(self.id, t);
        self.table_table.get_table_mut(t).remove(&self.id)
    }

    fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.table_table.get_table_mut(t).get_mut(&self.id)
    }
}

impl<'a, EntryType, Entry> IterTableRef<'a, EntryType, Entry>
    for InvertedTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    type Iter = InvertedTableIter<'a, EntryType, Entry>;
    type TypeIter = EntryTypeIter<'a, EntryType>;
    type EntryIter = InvertedTableEntryIter<'a, EntryType, Entry>;

    fn slots(self) -> Self::Iter {
        InvertedTableIter {
            id: self.id,
            iter: self.types(),
            table_table: self.table_table,
        }
    }

    fn entries(self) -> Self::EntryIter {
        InvertedTableEntryIter(self.slots())
    }

    fn types(self) -> Self::TypeIter {
        self.table_table.id_types.types(self.id)
    }
}

pub struct InvertedTableIter<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    id: TableId,
    iter: EntryTypeIter<'a, EntryType>,
    table_table: &'a InvertedTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> Iterator for InvertedTableIter<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    type Item = (&'a EntryType, &'a Entry);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry_type) = self.iter.next() {
            Some((entry_type, self.table_table.get_table(*entry_type).get(&self.id).unwrap()))
        } else {
            None
        }
    }
}

pub struct InvertedTableEntryIter<'a, EntryType, Entry>(InvertedTableIter<'a, EntryType, Entry>)
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>;

impl<'a, EntryType, Entry> Iterator for InvertedTableEntryIter<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    type Item = &'a Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, entry)) = self.0.next() {
            Some(entry)
        } else {
            None
        }
    }
}

impl<'a, EntryType, Entry> IdTableRef<'a, EntryType, Entry>
    for InvertedTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    fn id(self) -> TableId {
        self.id
    }
}

impl<'a, EntryType, Entry> Clone for InvertedEntryAccessor<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    fn clone(&self) -> Self {
        InvertedEntryAccessor::new(self.entry_type, self.hash_map)
    }
}

impl<'a, EntryType, Entry> Copy for InvertedEntryAccessor<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
}

impl<'a, EntryType, Entry> EntryAccessor<'a, EntryType, Entry>
    for InvertedEntryAccessor<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    type IdIter = hash_map::Keys<'a, TableId, Entry>;
    type EntryIter = hash_map::Values<'a, TableId, Entry>;
    type Iter = hash_map::Iter<'a, TableId, Entry>;

    fn ids(self) -> Self::IdIter {
        self.hash_map.keys()
    }

    fn entries(self) -> Self::EntryIter {
        self.hash_map.values()
    }

    fn iter(self) -> Self::Iter {
        self.hash_map.iter()
    }

    fn entry_type(self) -> EntryType {
        self.entry_type
    }

    fn access(self, id: TableId) -> Option<&'a Entry> {
        self.hash_map.get(&id)
    }

    fn has(self, id: TableId) -> bool {
        self.hash_map.contains_key(&id)
    }
}

impl<'a, EntryType, Entry> TableTable<'a, EntryType, Entry> for InvertedTableTable<EntryType, Entry>
    where EntryType: 'a + Eq + Hash + Copy + ToIndex,
          Entry: 'a + ToType<EntryType>
{
    type Ref = InvertedTableRef<'a, EntryType, Entry>;
    type RefMut = InvertedTableRefMut<'a, EntryType, Entry>;
    type Accessor = InvertedEntryAccessor<'a, EntryType, Entry>;

    fn add(&mut self,
           id: TableId,
           mut table: Table<EntryType, Entry>)
           -> Option<Table<EntryType, Entry>> {
        let ret = self.remove(id);

        for (entry_type, entry) in table.slots.drain() {
            self.get_table_mut(entry_type).insert(id, entry);
            self.id_types.add(id, entry_type);
        }

        ret
    }

    fn remove(&mut self, id: TableId) -> Option<Table<EntryType, Entry>> {
        if let Some(mut entry_types) = self.id_types.remove_types(id) {
            let mut table = Table::<EntryType, Entry>::new();
            for entry_type in entry_types.drain() {
                let entry = self.get_table_mut(entry_type).remove(&id).unwrap();
                table.add(entry);
            }
            Some(table)
        } else {
            None
        }
    }

    fn get(&'a self, id: TableId) -> Option<Self::Ref> {
        if self.id_types.contains_id(id) {
            Some(InvertedTableRef::new(id, self))
        } else {
            None
        }
    }

    fn get_mut(&'a mut self, id: TableId) -> Option<Self::RefMut> {
        if self.id_types.contains_id(id) {
            Some(InvertedTableRefMut::new(id, self))
        } else {
            None
        }
    }

    fn accessor(&'a self, entry_type: EntryType) -> Self::Accessor {
        InvertedEntryAccessor::new(entry_type, self.get_table(entry_type))
    }
}
