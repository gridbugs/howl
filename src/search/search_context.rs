use grid::Grid;
use search::{Query, Traverse, Path};

#[derive(Debug)]
pub enum SearchError {
    StartOutOfGrid,
    NonTraversableStart,
    Exhausted,
    AtDestination,
}

pub trait SearchContext {
    fn search<T: Traverse, G: Grid<Item = T>>(&self,
                                              grid: &G,
                                              query: &Query<T>)
                                              -> Result<Path, SearchError>;
}
