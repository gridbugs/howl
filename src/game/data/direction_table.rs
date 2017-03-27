use std::mem;
use math::*;

#[derive(Serialize, Deserialize)]
pub struct DirectionTable<T> {
    slots: Vec<Option<T>>,
}

impl<T> DirectionTable<T> {
    pub fn new() -> Self {
        let mut slots = Vec::new();
        for _ in 0..NUM_DIRECTIONS {
            slots.push(None);
        }
        DirectionTable {
            slots: slots,
        }
    }

    pub fn get(&self, direction: Direction) -> Option<&T>{
        self.slots[direction.index()].as_ref()
    }

    pub fn insert(&mut self, direction: Direction, data: T) -> Option<T> {
        mem::replace(&mut self.slots[direction.index()], Some(data))
    }

    pub fn remove(&mut self, direction: Direction) -> Option<T> {
        mem::replace(&mut self.slots[direction.index()], None)
    }

    pub fn iter(&self) -> DirectionTableIter<T> {
        DirectionTableIter {
            table: self,
            iter: direction_iter(),
        }
    }
}

pub struct DirectionTableIter<'a, T: 'a> {
    table: &'a DirectionTable<T>,
    iter: DirectionIter,
}

impl<'a, T: 'a> Iterator for DirectionTableIter<'a, T> {
    type Item = (Direction, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(d) = self.iter.next() {
                if let Some(x) = self.table.get(d) {
                    return Some((d, x));
                }
            } else {
                return None;
            }
        }
    }
}
