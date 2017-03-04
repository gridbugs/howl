use std::mem;
use direction;

#[derive(Serialize, Deserialize)]
pub struct DirectionTable<T> {
    slots: Vec<Option<T>>,
}

impl<T> DirectionTable<T> {
    pub fn new() -> Self {
        let mut slots = Vec::new();
        for _ in 0..direction::NUM_DIRECTIONS {
            slots.push(None);
        }
        DirectionTable {
            slots: slots,
        }
    }

    pub fn get(&self, direction: direction::Direction) -> Option<&T>{
        self.slots[direction.index()].as_ref()
    }

    pub fn insert(&mut self, direction: direction::Direction, data: T) -> Option<T> {
        mem::replace(&mut self.slots[direction.index()], Some(data))
    }

    pub fn remove(&mut self, direction: direction::Direction) -> Option<T> {
        mem::replace(&mut self.slots[direction.index()], None)
    }
}
