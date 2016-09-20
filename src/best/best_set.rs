#[derive(Debug)]
pub struct BestSet<T: Ord + Copy> {
    value: Option<T>,
}

impl<T: Ord + Copy> BestSet<T> {
    pub fn new() -> Self {
        BestSet { value: None }
    }

    pub fn clear(&mut self) {
        self.value = None;
    }

    pub fn insert(&mut self, value: T) {
        if let Some(v) = self.value {
            if value > v {
                self.value = Some(value);
            }
        } else {
            self.value = Some(value);
        }
    }

    pub fn best(&self) -> Option<T> {
        self.value
    }
}
