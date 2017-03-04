use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct FirstWeightedProbabilisticChoice<T> {
    first_weight: f64,
    first: T,
    rest: Vec<T>,
}

impl<T> FirstWeightedProbabilisticChoice<T> {
    pub fn new(first_weight: f64, first: T, rest: Vec<T>) -> Self {
        FirstWeightedProbabilisticChoice {
            first_weight: first_weight,
            first: first,
            rest: rest,
        }
    }

    pub fn choose<'a, R: Rng>(&self, rng: &'a mut R) -> &T {
        if rng.next_f64() < self.first_weight {
            &self.first
        } else {
            &self.rest[rng.gen::<usize>() % self.rest.len()]
        }
    }
}
