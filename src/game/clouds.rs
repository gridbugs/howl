use geometry::Vector3;

use perlin::{Perlin3Grid, PerlinWrapType};

#[derive(Debug, Clone)]
pub struct CloudContext {
    perlin: Perlin3Grid,
    perlin_zoom: f64,
    perlin_min: f64,
    perlin_max: f64,
    perlin_change: Vector3<f64>,
}

impl CloudContext {
    pub fn new(width: usize, height: usize) -> Self {
        CloudContext {
            perlin: Perlin3Grid::new(width, height, PerlinWrapType::Regenerate).unwrap(),
            perlin_zoom: 0.05,
            perlin_min: -0.1,
            perlin_max: 0.1,
            perlin_change: Vector3::new(0.005, 0.002, 0.001),
        }
    }

    fn noise(&self, x: isize, y: isize) -> Option<f64> {
        self.perlin.noise((x as f64) * self.perlin_zoom, (y as f64) * self.perlin_zoom)
    }

    pub fn mutate(&mut self, time: u64) {

        let change = self.perlin_change * (time as f64);

        self.perlin.scroll(change.x, change.y);
        self.perlin.mutate(change.z);
    }

    pub fn is_cloud(&self, x: isize, y: isize) -> bool {
        if let Some(noise) = self.noise(x, y) {
            if noise > self.perlin_min && noise < self.perlin_max {
                false
            } else {
                true
            }
        } else {
            true
        }
    }
}
