use table::{TableId, ToType};
use std::hash::Hash;

pub trait TableRef<'a, EntryType, Entry>: Clone + Copy
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    fn has(self, entry_type: EntryType) -> bool;
    fn get(self, entry_type: EntryType) -> Option<&'a Entry>;
}

pub trait IterTableRef<'a, EntryType, Entry>: TableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    type Iter: Iterator<Item = (&'a EntryType, &'a Entry)>;
    type TypeIter: Iterator<Item = &'a EntryType>;
    type EntryIter: Iterator<Item = &'a Entry>;

    fn slots(self) -> Self::Iter;
    fn types(self) -> Self::TypeIter;
    fn entries(self) -> Self::EntryIter;
}

pub trait TableRefMut<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    fn add(&mut self, entry: Entry) -> Option<Entry>;
    fn remove(&mut self, entry_type: EntryType) -> Option<Entry>;
    fn get_mut(&mut self, entry_type: EntryType) -> Option<&mut Entry>;
}

pub trait IdTableRef<'a, EntryType, Entry>: IterTableRef<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>
{
    fn id(self) -> TableId;
}
