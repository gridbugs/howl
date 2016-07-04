use grid::static_grid::StaticGrid;
use geometry::vector2::Vector2;
use geometry::vector::Dot;

pub type Coord = Vector2<f64>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PerlinWrapType {
    Repeat,
    Regenerate,
}

#[derive(Debug)]
struct Perlin2Vector(Vector2<f64>);

impl Default for Perlin2Vector {
    fn default() -> Self {
        Perlin2Vector(Vector2::random_unit_vector())
    }
}

#[derive(Debug)]
pub struct Perlin2Grid {
    grid: StaticGrid<Perlin2Vector>,
    width: f64,
    height: f64,
    minor_offset: Vector2<f64>,
    major_offset: Vector2<isize>,
    wrap_type: PerlinWrapType,
}

const NUM_CORNERS: usize = 4;

fn ease_curve(x: f64) -> f64 {
    6.0 * x.powi(5) - 15.0 * x.powi(4) + 10.0 * x.powi(3)
}

impl Perlin2Grid {
    pub fn new(width: f64, height: f64, wrap_type: PerlinWrapType)
        -> Perlin2Grid
    {
        // 2 is added to the dimensions of the grid.
        // One increase comes from the fact that we store a vector for each
        // corner in the grid.
        // The other comes from the fact that sliding the window requires a
        // buffer around the bottom and right sides of the grid.
        let limit_width = width.ceil() as isize + 2;
        let limit_height = height.ceil() as isize + 2;
        Perlin2Grid {
            grid: StaticGrid::new_default(limit_width, limit_height),
            // Dimensions are inceremented by 1 to account for the buffer
            // around the bottom and right.
            width: width + 1.0,
            height: height + 1.0,
            minor_offset: Vector2::new(0.0, 0.0),
            major_offset: Vector2::new(0, 0),
            wrap_type: wrap_type,
        }
    }

    pub fn scroll(&mut self, x: f64, y: f64) {
        self.minor_offset.x += x;
        self.minor_offset.y += y;

        let floor_f = Vector2::new(self.minor_offset.x.floor(),
                                   self.minor_offset.y.floor());

        let floor_i = Vector2::new(floor_f.x as isize,
                                   floor_f.y as isize);

        let width = self.grid.width;
        let height = self.grid.height;

        if floor_i.x != 0 {
            if self.wrap_type == PerlinWrapType::Regenerate {
                if floor_i.x > 0 {
                    for i in (self.major_offset.x)..(self.major_offset.x + floor_i.x) {
                        for j in 0..self.grid.height {
                            self.grid[((i + width) % width, j)] = Default::default();
                        }
                    }
                } else {
                    for i in (self.major_offset.x + floor_i.x)..(self.major_offset.x) {
                        for j in 0..self.grid.height {
                            self.grid[((i + width) % width, j)] = Default::default();
                        }
                    }
                }
            }
            self.major_offset.x += floor_i.x;
            self.major_offset.x = (self.major_offset.x + self.grid.width) % self.grid.width;
        }

        if floor_i.y != 0 {
            if self.wrap_type == PerlinWrapType::Regenerate {
                if floor_i.y > 0 {
                    for i in (self.major_offset.y)..(self.major_offset.y + floor_i.y) {
                        for j in 0..self.grid.width {
                            self.grid[(j, (i + height) % height)] = Default::default();
                        }
                    }
                } else {
                    for i in (self.major_offset.y + floor_i.y)..(self.major_offset.y) {
                        for j in 0..self.grid.width {
                            self.grid[(j, (i + height) % height)] = Default::default();
                        }
                    }
                }
            }
            self.major_offset.y += floor_i.y;
            self.major_offset.y = (self.major_offset.y + self.grid.height) % self.grid.height;
        }

        self.minor_offset -= floor_f;
    }

    pub fn noise(&self, global_coord: Coord) -> Option<f64> {

        let coord = global_coord + self.minor_offset;
        let Coord { x, y } = coord;

        if x > self.width || y > self.height {
            return None;
        }

        let top_left_f = Vector2::new(x.floor(), y.floor());
        let top_left_i = self.major_offset +
                         Vector2::new(top_left_f.x as isize,
                                      top_left_f.y as isize);

        let mut corner_coords_i = [
            top_left_i,
            top_left_i + Vector2::new(1, 0),
            top_left_i + Vector2::new(0, 1),
            top_left_i + Vector2::new(1, 1),
        ];

        for corner_coord_i in &mut corner_coords_i {
            corner_coord_i.x %= self.grid.width;
            corner_coord_i.y %= self.grid.height;
        }

        let corner_coords_f = [
            top_left_f,
            top_left_f + Vector2::new(1.0, 0.0),
            top_left_f + Vector2::new(0.0, 1.0),
            top_left_f + Vector2::new(1.0, 1.0),
        ];

        let mut dots: [f64; NUM_CORNERS] = [0.0; NUM_CORNERS];

        for (corner_coord_i, corner_coord_f, dot) in
            izip!(&corner_coords_i, &corner_coords_f, &mut dots)
        {
            let gradient = self.grid.get(*corner_coord_i).unwrap().0;
            let relative = coord - *corner_coord_f;
            *dot = gradient.dot(relative);
        }

        let weight_x = ease_curve(x - top_left_f.x);
        let weight_y = ease_curve(y - top_left_f.y);
        let avg0 = dots[0] + weight_x * (dots[1] - dots[0]);
        let avg1 = dots[2] + weight_x * (dots[3] - dots[2]);
        let avg = avg0 + weight_y * (avg1 - avg0);

        Some(avg)
    }
}
