use std::cmp;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum HealthStatus {
    Healthy,
    Wounded,
    Dead,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

    pub fn new_with(max: isize, current: isize) -> Self {
        HitPoints {
            max: max,
            current: cmp::min(current, max),
        }
    }

    pub fn current(&self) -> isize {
        self.current
    }

    pub fn max(&self) -> isize {
        self.max
    }

    pub fn ucurrent(&self) -> usize {
        cmp::max(self.current, 0) as usize
    }

    pub fn umax(&self) -> usize {
        cmp::max(self.max, 0) as usize
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

    pub fn is_full(&self) -> bool {
        self.current >= self.max
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
