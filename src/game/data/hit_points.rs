use std::cmp;

pub enum HealthStatus {
    Healthy,
    Wounded,
    Dead,
}

#[derive(Clone, Copy, Debug)]
pub struct HitPoints {
    current: isize,
    max: isize,
}

impl HitPoints {
    pub fn new(max: isize) -> Self {
        HitPoints {
            current: max,
            max: max,
        }
    }

    pub fn current(&self) -> isize {
        self.current
    }

    pub fn max(&self) -> isize {
        self.max
    }

    pub fn dec(&mut self, amount: usize) {
        self.current -= amount as isize;
    }

    pub fn inc(&mut self, amount: usize) {
        self.current = cmp::max(self.current + amount as isize, self.max);
    }

    pub fn is_positive(&self) -> bool {
        self.current > 0
    }

    pub fn status(&self) -> HealthStatus {
        let half = self.max / 2;
        if self.current > half {
            HealthStatus::Healthy
        } else if self.current > 0 {
            HealthStatus::Wounded
        } else {
            HealthStatus::Dead
        }
    }
}
