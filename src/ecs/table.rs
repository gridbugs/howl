use std::collections::HashMap;
use std::hash::Hash;

pub trait ToType<EntryType> {
    fn to_type(&self) -> EntryType;
}

pub type TableId = u64;

#[derive(Debug)]
pub struct Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    pub id: Option<TableId>,
    pub slots: HashMap<EntryType, Entry>,
}

impl<EntryType, Entry> Table<EntryType, Entry>
    where EntryType: Eq + Hash,
          Entry: ToType<EntryType>,
{
    pub fn new() -> Table<EntryType, Entry> {
        Table {
            id: None,
            slots: HashMap::new(),
        }
    }

    pub fn add(&mut self, entry: Entry) {
        self.slots.insert(entry.to_type(), entry);
    }
}

macro_rules! table {
    ( $( $x:expr ),* ) => {
        {
            let mut table = ecs::table::Table::new();

            $(
                table.add($x);
            )*

            table
        }
    }
}
