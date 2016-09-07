use table::{
    TableId,
    ToType,
};
use std::hash::Hash;

pub trait TableRef<'a, EntryType, Entry>: Clone + Copy
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
{
    fn has(self, entry_type: EntryType) -> bool;
    fn get(self, entry_type: EntryType) -> Option<&'a Entry>;
}

pub trait IterTableRef<'a, EntryType, Entry>: TableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
{
    type Iter: Iterator<Item=(&'a EntryType, &'a Entry)>;
    type TypeIter: Iterator<Item=&'a EntryType>;
    type EntryIter: Iterator<Item=&'a Entry>;

    fn slots(self) -> Self::Iter;
    fn types(self) -> Self::TypeIter;
    fn entries(self) -> Self::EntryIter;
}

pub trait TableRefMut<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
{
    fn add(&mut self, entry: Entry) -> Option<Entry>;
    fn remove(&mut self, entry_type: EntryType) -> Option<Entry>;
    fn get_mut(&mut self, entry_type: EntryType) -> Option<&mut Entry>;
}

pub trait IdTableRef<'a, EntryType, Entry>: IterTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
{
    fn id(self) -> TableId;
}


pub trait EntryAccessor<'a, EntryType, Entry>: Clone + Copy
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
{
    type IdIter: Iterator<Item=&'a TableId>;
    type Iter: Iterator<Item=(&'a TableId, &'a Entry)>;
    type EntryIter: Iterator<Item=&'a Entry>;

    fn iter(self) -> Self::Iter;
    fn entries(self) -> Self::EntryIter;
    fn ids(self) -> Self::IdIter;

    fn entry_type(self) -> EntryType;
    fn access(self, id: TableId) -> Option<&'a Entry>;
    fn has(self, id: TableId) -> bool;
}

pub struct AccessorIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>,
{
    id_iter: Accessor::IdIter,
    accessor: Accessor,
}

impl<'a, Accessor, EntryType, Entry> AccessorIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>,
{
    pub fn new(accessor: Accessor) -> Self {
        AccessorIter {
            id_iter: accessor.ids(),
            accessor: accessor,
        }
    }
}

impl<'a, Accessor, EntryType, Entry> Iterator for AccessorIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>,
{
    type Item = (&'a TableId, &'a Entry);
    fn next(&mut self) -> Option<Self::Item> {
        self.id_iter.next().map(|id| {
            (id, self.accessor.access(*id).unwrap())
        })
    }
}

pub struct AccessorEntryIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>,
{
    id_iter: Accessor::IdIter,
    accessor: Accessor,
}

impl<'a, Accessor, EntryType, Entry> AccessorEntryIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>,
{
    pub fn new(accessor: Accessor) -> Self {
        AccessorEntryIter {
            id_iter: accessor.ids(),
            accessor: accessor,
        }
    }
}

impl<'a, Accessor, EntryType, Entry> Iterator for AccessorEntryIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>,
{
    type Item = &'a Entry;
    fn next(&mut self) -> Option<Self::Item> {
        self.id_iter.next().map(|id| {
            self.accessor.access(*id).unwrap()
        })
    }
}
