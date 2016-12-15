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

    pub fn is_empty(&self) -> bool {
        self.any.is_none()
    }

    pub fn insert(&mut self, value: T) {
        if self.inner.is_empty() {
            self.any = Some(value);
        }
        self.inner.insert(value);
    }

    pub fn remove(&mut self, value: T) {
        self.inner.remove(&value);
        self.any = self.inner.iter().next().map(|r| *r);
    }
}
