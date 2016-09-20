use table::{TableId, ToType};
use std::hash::Hash;

pub trait EntryAccessor<'a, EntryType, Entry>: Clone + Copy
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    type IdIter: Iterator<Item = &'a TableId>;
    type Iter: Iterator<Item = (&'a TableId, &'a Entry)>;
    type EntryIter: Iterator<Item = &'a Entry>;

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
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>
{
    id_iter: Accessor::IdIter,
    accessor: Accessor,
}

impl<'a, Accessor, EntryType, Entry> AccessorIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>
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
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>
{
    type Item = (&'a TableId, &'a Entry);
    fn next(&mut self) -> Option<Self::Item> {
        self.id_iter.next().map(|id| (id, self.accessor.access(*id).unwrap()))
    }
}

pub struct AccessorEntryIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>
{
    id_iter: Accessor::IdIter,
    accessor: Accessor,
}

impl<'a, Accessor, EntryType, Entry> AccessorEntryIter<'a, Accessor, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>
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
          Accessor: 'a + EntryAccessor<'a, EntryType, Entry>
{
    type Item = &'a Entry;
    fn next(&mut self) -> Option<Self::Item> {
        self.id_iter.next().map(|id| self.accessor.access(*id).unwrap())
    }
}
