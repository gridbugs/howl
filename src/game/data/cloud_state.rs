use perlin::*;
use math::{Coord, Vector2};

pub struct CloudState {
    perlin: PerlinGrid,
    zoom: f64,
    min: f64,
    max: f64,
    scroll_rate: Vector2<f64>,
    mutate_rate: f64,
}

impl CloudState {
    pub fn new(width: usize, height: usize, zoom: f64, min: f64, max: f64, scroll_rate: Vector2<f64>, mutate_rate: f64) -> Self {

        let zoomed_width = ((width as f64) * zoom).ceil() as usize;
        let zoomed_height = ((height as f64) * zoom).ceil() as usize;

        CloudState {
            perlin: PerlinGrid::new(zoomed_width, zoomed_height, PerlinWrapType::Regenerate).unwrap(),
            zoom: zoom,
            min: min,
            max: max,
            scroll_rate: scroll_rate,
            mutate_rate: mutate_rate,
        }
    }

    pub fn progress(&mut self, scale: f64) {
        let scroll = self.scroll_rate * scale;
        let mutate = self.mutate_rate * scale;

        self.perlin.scroll(scroll.x, scroll.y);
        self.perlin.mutate(mutate);
    }

    fn noise(&self, x: f64, y: f64) -> Option<f64> {
        self.perlin.noise(x * self.zoom, y * self.zoom)
    }

    pub fn is_cloud(&self, coord: Coord) -> bool {
        self.noise(coord.x as f64, coord.y as f64).map_or(false, |noise| {
            noise >= self.min && noise <= self.max
        })
    }
}
