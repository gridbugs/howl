use std::collections::HashSet;
use std::hash::Hash;

pub struct AnySet<T: Copy + Hash + Eq> {
    inner: HashSet<T>,
    any: Option<T>,
}

impl<T: Copy + Hash + Eq> AnySet<T> {
    pub fn new() -> Self {
        AnySet {
            inner: HashSet::new(),
            any: None,
        }
    }

    pub fn any(&self) -> Option<T> {
        self.any
    }

    pub fn insert(&mut self, value: T) {
        self.inner.insert(value);
        self.any = Some(value);
    }

    pub fn remove(&mut self, value: T) {
        self.inner.remove(&value);
        if self.inner.is_empty() {
            self.any = None;
        }
    }
}
