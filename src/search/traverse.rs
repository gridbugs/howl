#[derive(Clone, Copy, Debug)]
pub enum TraverseType {
    Traversable(f64),
    NonTraversable,
}

pub trait Traverse {
    fn get_type(&self) -> TraverseType;

    fn is_traversable(&self) -> bool {
        match self.get_type() {
            TraverseType::Traversable(_) => true,
            TraverseType::NonTraversable => false,
        }
    }

    fn cost(&self) -> Option<f64> {
        match self.get_type() {
            TraverseType::Traversable(cost) => Some(cost),
            TraverseType::NonTraversable => None,
        }
    }
}
