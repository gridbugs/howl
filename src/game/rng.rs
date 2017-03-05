use std::cell::{RefCell, RefMut};
use rand::{StdRng, SeedableRng, Rng, Rand};

pub struct GameRng {
    rng: RefCell<StdRng>,
}

impl GameRng {
    pub fn new(seed: usize) -> Self {
        GameRng {
            rng: RefCell::new(StdRng::from_seed(&[seed])),
        }
    }

    pub fn gen<T: Rand>(&self) -> T {
        self.rng.borrow_mut().gen()
    }

    pub fn gen_usize(&self) -> usize {
        self.gen()
    }

    pub fn gen_usize_below(&self, value: usize) -> usize {
        self.gen_usize() % value
    }

    pub fn gen_f64(&self) -> f64 {
        self.rng.borrow_mut().next_f64()
    }

    pub fn count_failures(&self, value: f64, max: usize) -> usize {
        for count in 0..max {
            if self.gen_f64() < value {
                return count;
            }
        }

        return max;
    }

    pub fn inner_mut(&self) -> RefMut<StdRng> {
        self.rng.borrow_mut()
    }

    pub fn select_uniform<'a, T>(&self, items: &'a [T]) -> &'a T {
        &items[self.gen_usize_below(items.len())]
    }

    pub fn select_or_select_uniform<'a, T>(&self, first_probability: f64, first: &'a T, rest: &'a [T]) -> &'a T {
        if rest.is_empty() || self.gen_f64() < first_probability {
            first
        } else {
            self.select_uniform(rest)
        }
    }
}
