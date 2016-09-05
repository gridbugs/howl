pub trait ToType<EntryType> {
    fn to_type(&self) -> EntryType;
}
