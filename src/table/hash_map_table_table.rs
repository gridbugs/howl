use table::{
    TableId,
    ToType,
    Table,
    TableTable,
    TableRef,
    TableRefMut,
    IterTableRef,
    IdTableRef,
    EntryAccessor,
    TypeIdMap,
    TableIdIter,
    AccessorIter,
    AccessorEntryIter,
};

use std::collections::{
    HashMap,
    hash_map,
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
    type Accessor = HashMapEntryAccessor<'a, EntryType, Entry>;

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

    fn accessor(&'a self, entry_type: EntryType) -> Self::Accessor {
        HashMapEntryAccessor::new(entry_type, self)
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

pub struct HashMapEntryAccessor<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    entry_type: EntryType,
    table_table: &'a HashMapTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> HashMapEntryAccessor<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn new(entry_type: EntryType,
           table_table: &'a HashMapTableTable<EntryType, Entry>) -> Self
    {
        HashMapEntryAccessor {
            entry_type: entry_type,
            table_table: table_table,
        }
    }
}

impl<'a, EntryType, Entry> Clone for HashMapEntryAccessor<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        HashMapEntryAccessor::new(self.entry_type, self.table_table)
    }
}

impl<'a, EntryType, Entry> Copy for HashMapEntryAccessor<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>
{}

impl<'a, EntryType, Entry> EntryAccessor <'a, EntryType, Entry>
for HashMapEntryAccessor<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{

    type IdIter = TableIdIter<'a>;
    type EntryIter = AccessorEntryIter<'a, Self, EntryType, Entry>;
    type Iter = AccessorIter<'a, Self, EntryType, Entry>;

    fn ids(self) -> Self::IdIter {
        self.table_table.entry_type_map.ids(self.entry_type)
    }

    fn entries(self) -> Self::EntryIter {
        AccessorEntryIter::new(self)
    }

    fn iter(self) -> Self::Iter {
        AccessorIter::new(self)
    }

    fn entry_type(self) -> EntryType {
        self.entry_type
    }

    fn access(self, id: TableId) -> Option<&'a Entry> {
        if let Some(t) = self.table_table.get(id) {
            t.get(self.entry_type)
        } else {
            None
        }
    }

    fn has(self, id: TableId) -> bool {
        if let Some(t) = self.table_table.get(id) {
            t.has(self.entry_type)
        } else {
            false
        }
    }
}
