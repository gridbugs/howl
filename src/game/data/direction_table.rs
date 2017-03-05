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

    pub fn iter(&self) -> DirectionTableIter<T> {
        DirectionTableIter {
            table: self,
            iter: direction::iter(),
        }
    }
}

pub struct DirectionTableIter<'a, T: 'a> {
    table: &'a DirectionTable<T>,
    iter: direction::Iter,
}

impl<'a, T: 'a> Iterator for DirectionTableIter<'a, T> {
    type Item = (direction::Direction, &'a T);
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
