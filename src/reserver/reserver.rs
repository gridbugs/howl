pub struct LeakyReserver(u64);

impl LeakyReserver {
    pub fn new() -> Self {
        LeakyReserver(0)
    }

    pub fn reserve(&mut self) -> u64 {
        let ret = self.0;
        self.0 += 1;
        ret
    }
}
