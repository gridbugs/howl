use num::Integer;

#[derive(Serialize, Deserialize)]
pub struct LeakyReserver<T: Integer>(T);

impl<T: Integer + Copy> LeakyReserver<T> {
    pub fn new() -> Self {
        LeakyReserver(T::zero())
    }

    pub fn reserve(&mut self) -> T {
        let ret = self.0;
        self.0 = self.0 + T::one();
        ret
    }
}
