use table::{
    TableId,
    ToType,
    Table,
    TableTable,
    TableRef,
    TableRefMut,
    IterTableRef,
    IdTableRef,
    EntryTypeTableRef,
    TypeIdMap,
};

use std::collections::{
    HashMap,
    hash_map,
    HashSet,
    hash_set,
};
use std::hash::Hash;

pub struct HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    id: TableId,
    table: &'a Table<EntryType, Entry>,
    entry_type_map: &'a TypeIdMap<EntryType>,
}

impl<'a, EntryType, Entry> HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn new(id: TableId,
           entry_type_map: &'a TypeIdMap<EntryType>,
           table: &'a Table<EntryType, Entry>) -> Self
    {
        HashMapTableRef {
            id: id,
            table: table,
            entry_type_map: entry_type_map,
        }
    }
}

pub struct HashMapTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    id: TableId,
    table: &'a mut Table<EntryType, Entry>,
    entry_type_map: &'a mut TypeIdMap<EntryType>,
}

impl<'a, EntryType, Entry> HashMapTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn new(id: TableId,
           entry_type_map: &'a mut TypeIdMap<EntryType>,
           table: &'a mut Table<EntryType, Entry>) -> Self
    {
        HashMapTableRefMut {
            id: id,
            table: table,
            entry_type_map: entry_type_map,
        }
    }

    fn add_type(&mut self, entry_type: EntryType) {
        self.entry_type_map.add(self.id, entry_type);
    }

    fn remove_type(&mut self, entry_type: EntryType) {
        self.entry_type_map.remove(self.id, entry_type);
    }
}

#[derive(Debug, Clone)]
pub struct HashMapTableTable<EntryType, Entry>
    where EntryType: Eq + Hash + Copy,
          Entry: ToType<EntryType>,
{
    // map ids to tables
    tables: HashMap<TableId, Table<EntryType, Entry>>,

    // map entry types to ids
    entry_type_map: TypeIdMap<EntryType>,
}

impl<EntryType, Entry> HashMapTableTable<EntryType, Entry>
where EntryType: Eq + Hash + Copy,
      Entry: ToType<EntryType>,
{
    pub fn new() -> Self {
        HashMapTableTable {
            tables: HashMap::new(),
            entry_type_map: TypeIdMap::new(),
        }
    }

    fn add_type(&mut self, id: TableId, entry_type: EntryType) {
        self.entry_type_map.add(id, entry_type);
    }

    fn remove_type(&mut self, id: TableId, entry_type: EntryType) {
        self.entry_type_map.remove(id, entry_type);
    }
}

impl<'a, EntryType, Entry> TableTable<'a, EntryType, Entry>
for HashMapTableTable<EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Ref = HashMapTableRef<'a, EntryType, Entry>;
    type RefMut = HashMapTableRefMut<'a, EntryType, Entry>;
    type EntryTypeRef = HashMapEntryTypeTableRef<'a, EntryType, Entry>;

    fn add(&mut self, table_id: TableId, table: Table<EntryType, Entry>)
        -> Option<Table<EntryType, Entry>>
    {
        for entry_type in table.types() {
            self.add_type(table_id, *entry_type);
        }

        self.tables.insert(table_id, table)
    }

    fn remove(&mut self, id: TableId) -> Option<Table<EntryType, Entry>> {
        let table = self.tables.remove(&id);

        if let Some(t) = table.as_ref() {
            for entry_type in t.types() {
                self.remove_type(id, *entry_type);
            }
        }

        table
    }

    fn get(&'a self, id: TableId) -> Option<Self::Ref> {
        self.tables.get(&id).map(|t| {
            HashMapTableRef::new(id, &self.entry_type_map, t)
        })
    }

    fn get_mut(&'a mut self, id: TableId) -> Option<Self::RefMut> {
        if let Some(t) = self.tables.get_mut(&id) {
            Some(HashMapTableRefMut::new(id, &mut self.entry_type_map, t))
        } else {
            None
        }
    }

    fn entry_type(&'a self, entry_type: EntryType) -> Option<Self::EntryTypeRef> {
        if let Some(ids) = self.entry_type_map.get(entry_type) {
            Some(HashMapEntryTypeTableRef::new(entry_type, &self, ids))
        } else {
            None
        }
    }
}

impl<'a, EntryType, Entry> TableRefMut<'a, EntryType, Entry> for HashMapTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn add(&mut self, entry: Entry) -> Option<Entry> {
        self.add_type(entry.to_type());
        self.table.add(entry)
    }

    fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.remove_type(t);
        self.table.remove(t)
    }

    fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.table.get_mut(t)
    }
}

impl<'a, EntryType, Entry> Clone for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        HashMapTableRef::new(self.id, self.entry_type_map, self.table)
    }
}

impl<'a, EntryType, Entry> Copy for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>
{}

impl<'a, EntryType, Entry> TableRef<'a, EntryType, Entry> for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn get(self, entry_type: EntryType) -> Option<&'a Entry> {
        self.table.get(entry_type)
    }

    fn has(self, entry_type: EntryType) -> bool {
        self.table.has(entry_type)
    }
}

impl<'a, EntryType, Entry> IterTableRef<'a, EntryType, Entry> for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Iter = hash_map::Iter<'a, EntryType, Entry>;
    type TypeIter = hash_map::Keys<'a, EntryType, Entry>;
    type EntryIter = hash_map::Values<'a, EntryType, Entry>;

    fn slots(self) -> Self::Iter {
        self.table.slots()
    }

    fn entries(self) -> Self::EntryIter {
        self.table.entries()
    }

    fn types(self) -> Self::TypeIter {
        self.table.types()
    }
}

impl<'a, EntryType, Entry> IdTableRef<'a, EntryType, Entry> for HashMapTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn id(self) -> TableId {
        self.id
    }
}

pub struct HashMapEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    entry_type: EntryType,
    table_table: &'a HashMapTableTable<EntryType, Entry>,
    ids: &'a HashSet<TableId>,
}

impl<'a, EntryType, Entry> HashMapEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn new(entry_type: EntryType,
           table_table: &'a HashMapTableTable<EntryType, Entry>,
           ids: &'a HashSet<TableId>) -> Self
    {
        HashMapEntryTypeTableRef {
            entry_type: entry_type,
            table_table: table_table,
            ids: ids,
        }
    }
}

impl<'a, EntryType, Entry> Clone for HashMapEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        HashMapEntryTypeTableRef::new(self.entry_type, self.table_table, self.ids)
    }
}

impl<'a, EntryType, Entry> Copy for HashMapEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>
{}

pub struct HashMapEntryIter<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    entry_type_table_ref: HashMapEntryTypeTableRef<'a, EntryType, Entry>,
    iter: hash_set::Iter<'a, TableId>,
}

impl<'a, EntryType, Entry> Iterator for HashMapEntryIter<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Item = HashMapTableRef<'a, EntryType, Entry>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.iter.next() {
            Some(self.entry_type_table_ref.table_table.get(*id).unwrap())
        } else {
            None
        }
    }
}

impl<'a, EntryType, Entry> EntryTypeTableRef <'a, EntryType, Entry>
for HashMapEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Ref = HashMapTableRef<'a, EntryType, Entry>;
    type IdIter = hash_set::Iter<'a, TableId>;
    type Iter = HashMapEntryIter<'a, EntryType, Entry>;

    fn iter(self) -> Self::Iter {
        HashMapEntryIter {
            entry_type_table_ref: self,
            iter: self.id_iter(),
        }
    }

    fn id_iter(self) -> Self::IdIter {
        self.ids.iter()
    }
}
