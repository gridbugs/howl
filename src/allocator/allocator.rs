use std::collections::HashMap;

pub struct Allocator<T> {
    next_id: u64,
    objects: HashMap<u64, T>,
}

impl<T> Allocator<T> {
    pub fn new() -> Allocator<T> {
        Allocator {
            next_id: 0,
            objects: HashMap::new(),
        }
    }

    pub fn allocate(&mut self, object: T) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(id, object);
        return id;
    }

    pub fn get(&self, id: u64) -> Option<&T> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut T> {
        self.objects.get_mut(&id)
    }
}
