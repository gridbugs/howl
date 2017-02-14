#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct BestMap<K: Ord + Copy, V: Copy> {
    key: Option<K>,
    value: Option<V>,
}

impl<K: Ord + Copy, V: Copy> BestMap<K, V> {
    pub fn new() -> Self {
        BestMap {
            key: None,
            value: None,
        }
    }

    pub fn clear(&mut self) {
        self.key = None;
        self.value = None;
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(k) = self.key {
            if key > k {
                self.key = Some(key);
                self.value = Some(value);
            }
        } else {
            self.key = Some(key);
            self.value = Some(value);
        }
    }

    pub fn key(&self) -> Option<K> {
        self.key
    }

    pub fn value(&self) -> Option<V> {
        self.value
    }

    pub fn items(&self) -> Option<(K, V)> {
        if let Some(k) = self.key {
            if let Some(v) = self.value {
                return Some((k, v));
            }
        }

        None
    }
}
