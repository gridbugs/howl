use rand::Rng;
use perlin::*;
use math::Vector2;
use coord::Coord;

#[derive(Serialize, Deserialize)]
pub struct CloudState {
    perlin: PerlinGrid,
    x_zoom: f64,
    y_zoom: f64,
    size: f64,
    scroll_rate: Vector2<f64>,
    mutate_rate: f64,
    width: usize,
    height: usize,
}

impl CloudState {
    pub fn new<R: Rng>(width: usize,
                       height: usize,
                       x_zoom: f64,
                       y_zoom: f64,
                       size: f64,
                       scroll_rate: Vector2<f64>,
                       mutate_rate: f64,
                       r: &mut R) -> Self {

        let zoomed_width = ((width as f64) * x_zoom).ceil() as usize;
        let zoomed_height = ((height as f64) * y_zoom).ceil() as usize;

        CloudState {
            perlin: PerlinGrid::new(zoomed_width, zoomed_height, PerlinWrapType::Regenerate, r),
            x_zoom: x_zoom,
            y_zoom: y_zoom,
            size: size,
            scroll_rate: scroll_rate,
            mutate_rate: mutate_rate,
            width: width,
            height: height,
        }
    }

    pub fn progress<R: Rng>(&mut self, r: &mut R, scale: f64) {
        let scroll = self.scroll_rate * scale;
        let mutate = self.mutate_rate * scale;

        self.perlin.scroll(r, scroll.x, scroll.y);
        self.perlin.mutate(r, mutate);
    }

    fn noise(&self, x: f64, y: f64) -> Option<f64> {
        self.perlin.noise(x * self.x_zoom, y * self.y_zoom)
    }

    pub fn is_cloud(&self, coord: Coord) -> bool {
        self.noise(coord.x as f64, coord.y as f64).map_or(false, |noise| {
            let min = 0.0;
            let max = 0.1;
            noise <= min || noise >= max
        })
    }

    pub fn iter(&self) -> CloudStateIter {
        CloudStateIter {
            state: self,
            coord: Coord::new(0, 0),
        }
    }
}

pub struct CloudStateIter<'a> {
    state: &'a CloudState,
    coord: Coord,
}

impl<'a> Iterator for CloudStateIter<'a> {
    type Item = (Coord, bool);
    fn next(&mut self) -> Option<Self::Item> {

        if self.coord.y >= self.state.height as isize {
            return None;
        }

        let coord = self.coord;
        self.coord.x += 1;
        if self.coord.x >= self.state.width as isize {
            self.coord.x = 0;
            self.coord.y += 1;
        }

        Some((coord, self.state.is_cloud(coord)))
    }
}
