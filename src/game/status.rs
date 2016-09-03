use std::cmp;

#[derive(Clone, Copy, Debug)]
pub struct StatusCounter {
    pub max: usize,
    pub current: usize,
}

impl StatusCounter {
    pub fn new(max: usize, current: usize) -> Self {
        StatusCounter {
            max: max,
            current: current,
        }
    }

    pub fn new_max(max: usize) -> Self {
        Self::new(max, max)
    }

    pub fn increase(&mut self, value: usize) {
        self.current = cmp::min(self.current + value, self.max);
    }

    pub fn decrease(&mut self, value: usize) {
        if value >= self.current {
            self.current = 0;
        } else {
            self.current -= value;
        }
    }

    pub fn change(&mut self, value: isize) {
        if value > 0 {
            self.increase(value as usize);
        } else if value < 0 {
            self.decrease((-value) as usize);
        }
    }

    pub fn is_zero(&self) -> bool {
        self.current == 0
    }
}
