use perlin::*;

pub struct CloudState {
    perlin: PerlinGrid,
}

impl CloudState {
    pub fn new(width: usize, height: usize) -> Self {
        CloudState {
            perlin: PerlinGrid::new(width, height, PerlinWrapType::Regenerate).unwrap(),
        }
    }
}
