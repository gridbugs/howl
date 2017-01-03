use rand::{StdRng, SeedableRng, Rng, Rand};

pub struct GameRng {
    rng: StdRng,
}

impl GameRng {
    pub fn new(seed: usize) -> Self {
        GameRng {
            rng: StdRng::from_seed(&[seed]),
        }
    }

    pub fn gen<T: Rand>(&mut self) -> T {
        self.rng.gen()
    }

    pub fn gen_usize(&mut self) -> usize {
        self.gen()
    }
}
