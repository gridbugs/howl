pub trait ToIndex {
    fn to_index(&self) -> usize;
    fn num_indices() -> usize;
}
