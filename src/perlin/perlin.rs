use grid::{
    StaticGrid,
    DefaultGrid,
};
use geometry::Dot;
use geometry::Length;
use geometry::Vector2;
use geometry::Vector3;

use rand;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PerlinWrapType {
    Repeat,
    Regenerate,
}

#[derive(Clone, Copy, Debug)]
struct Perlin3Vector(Vector3<f64>);

impl Default for Perlin3Vector {
    fn default() -> Self {
        // random number from 0 to 15
        let index = rand::thread_rng().gen::<usize>() & GRADIENT_MASK;
        Perlin3Vector(GRADIENTS[index].normalize())
    }
}

#[derive(Debug)]
struct Perlin3Slice {
    grid: StaticGrid<Perlin3Vector>,
    z: f64,
}

#[derive(Debug)]
pub struct Perlin3Grid {
    slices: Vec<Perlin3Slice>,
    grid_width: usize,
    grid_height: usize,
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
    minor_offset: Vector2<f64>,
    major_offset: Vector2<usize>,
    wrap_type: PerlinWrapType,
}

const NUM_CORNERS: usize = 8;

pub fn ease_curve(x: f64) -> f64 {
    6.0 * x.powi(5) - 15.0 * x.powi(4) + 10.0 * x.powi(3)
}

impl Perlin3Grid {
    pub fn new(width: usize, height: usize, wrap_type: PerlinWrapType) -> Perlin3Grid {
        let grid_width = width + 2;
        let grid_height = height + 2;
        Perlin3Grid {
            slices: {
                let mut v = Vec::with_capacity(2 as usize);
                for i in 0..2 as isize {
                    v.push(Perlin3Slice {
                        grid: StaticGrid::new_default(grid_width, grid_height),
                        z: i as f64,
                    });
                }
                v
            },
            grid_width: grid_width,
            grid_height: grid_height,
            width: (width + 1) as f64,
            height: (height + 1) as f64,
            depth: 1.0,
            z: 0.0,
            minor_offset: Vector2::new(0.0, 0.0),
            major_offset: Vector2::new(0, 0),
            wrap_type: wrap_type,
        }
    }

    fn swap_slices(&mut self) {
        self.slices.reverse();
        for (i, slice) in izip!(0..self.slices.len(), self.slices.iter_mut()) {
            slice.z = i as f64;
        }
    }

    pub fn mutate(&mut self, value: f64) {
        self.z += value;
        if self.z > 1.0 && self.z <= 2.0 {
            self.z -= 1.0;
            self.slices[0].grid.reset_all();
            self.swap_slices();
        } else if self.z < 0.0 && self.z >= -1.0 {
            self.z += 1.0;
            self.slices[1].grid.reset_all();
            self.swap_slices();
        } else if self.z > 2.0 || self.z < -1.0 {
            self.z = 0.0;
            for slice in self.slices.iter_mut() {
                slice.grid.reset_all();
            }
        }
    }

    pub fn scroll(&mut self, x: f64, y: f64) {
        self.minor_offset.x += x;
        self.minor_offset.y += y;

        let floor_f = Vector2::new(self.minor_offset.x.floor(),
                                   self.minor_offset.y.floor());

        let floor_i = Vector2::new(floor_f.x as usize,
                                   floor_f.y as usize);

        if floor_i.x != 0 {
            if self.wrap_type == PerlinWrapType::Regenerate {
                if floor_i.x > 0 {
                    for i in (self.major_offset.x)..(self.major_offset.x + floor_i.x) {
                        for j in 0..self.grid_height {
                            self.slices[0].grid[((i + self.grid_width) % self.grid_width, j)] = Default::default();
                            self.slices[1].grid[((i + self.grid_width) % self.grid_width, j)] = Default::default();
                        }
                    }
                } else {
                    for i in (self.major_offset.x + floor_i.x)..(self.major_offset.x) {
                        for j in 0..self.grid_height {
                            self.slices[0].grid[((i + self.grid_width) % self.grid_width, j)] = Default::default();
                            self.slices[1].grid[((i + self.grid_width) % self.grid_width, j)] = Default::default();
                        }
                    }
                }
            }
            self.major_offset.x += floor_i.x;
            self.major_offset.x = (self.major_offset.x + self.grid_width) % self.grid_width;
        }

        if floor_i.y != 0 {
            if self.wrap_type == PerlinWrapType::Regenerate {
                if floor_i.y > 0 {
                    for i in (self.major_offset.y)..(self.major_offset.y + floor_i.y) {
                        for j in 0..self.grid_width {
                            self.slices[0].grid[(j, (i + self.grid_height) % self.grid_height)] = Default::default();
                            self.slices[1].grid[(j, (i + self.grid_height) % self.grid_height)] = Default::default();
                        }
                    }
                } else {
                    for i in (self.major_offset.y + floor_i.y)..(self.major_offset.y) {
                        for j in 0..self.grid_width {
                            self.slices[0].grid[(j, (i + self.grid_height) % self.grid_height)] = Default::default();
                            self.slices[1].grid[(j, (i + self.grid_height) % self.grid_height)] = Default::default();
                        }
                    }
                }
            }
            self.major_offset.y += floor_i.y;
            self.major_offset.y = (self.major_offset.y + self.grid_height) % self.grid_height;
        }

        self.minor_offset -= floor_f;

    }

    pub fn noise(&mut self, global_x: f64, global_y: f64) -> Option<f64> {
        assert!(self.z >= 0.0 && self.z <= self.depth);

        let x = global_x + self.minor_offset.x;
        let y = global_y + self.minor_offset.y;

        if x > self.width || y > self.height {
            return None;
        }

        let global_coord = Vector3::new(x, y, self.z);

        let top_left_f = Vector2::new(x.floor(), y.floor());
        let top_left_i = self.major_offset +
                         Vector2::new(top_left_f.x as usize,
                                      top_left_f.y as usize);

        let mut corner_coords_i = [
            top_left_i,
            top_left_i + Vector2::new(1, 0),
            top_left_i + Vector2::new(0, 1),
            top_left_i + Vector2::new(1, 1),
        ];

        for corner_coord_i in &mut corner_coords_i {
            corner_coord_i.x %= self.grid_width;
            corner_coord_i.y %= self.grid_height;
        }

        let corner_coords_f = [
            top_left_f,
            top_left_f + Vector2::new(1.0, 0.0),
            top_left_f + Vector2::new(0.0, 1.0),
            top_left_f + Vector2::new(1.0, 1.0),
        ];

        let mut dots: [f64; NUM_CORNERS] = [0.0; NUM_CORNERS];

        for (dot, (slice, (corner_coord_i, corner_coord_f))) in
            izip!(&mut dots,
                  iproduct!(&self.slices,
                            izip!(&corner_coords_i,
                                  &corner_coords_f)))
        {
            let gradient = slice.grid[corner_coord_i].0;
            let corner_coord_f3 = Vector3::new(
                corner_coord_f.x,
                corner_coord_f.y,
                slice.z
            );
            let relative = global_coord - corner_coord_f3;
            *dot = gradient.dot(relative);
        }

        let weight_x = ease_curve(x - top_left_f.x);
        let weight_y = ease_curve(y - top_left_f.y);
        let weight_z = ease_curve(self.z.fract());

        let square_avgs = [
            dots[0] + weight_z * (dots[4] - dots[0]),
            dots[1] + weight_z * (dots[5] - dots[1]),
            dots[2] + weight_z * (dots[6] - dots[2]),
            dots[3] + weight_z * (dots[7] - dots[3]),
        ];

        let line_avgs = [
            square_avgs[0] + weight_x * (square_avgs[1] - square_avgs[0]),
            square_avgs[2] + weight_x * (square_avgs[3] - square_avgs[2]),
        ];

        let avg = line_avgs[0] + weight_y * (line_avgs[1] - line_avgs[0]);

        Some(avg.max(-1.0).min(1.0))
    }
}

const NUM_GRADIENTS: usize = 16;
const GRADIENT_MASK: usize = 0xf;
static GRADIENTS: [Vector3<f64>; NUM_GRADIENTS] = [
    Vector3 { x: 1.0, y: 1.0, z: 0.0 },
    Vector3 { x: -1.0, y: 1.0, z: 0.0 },
    Vector3 { x: 1.0, y: -1.0, z: 0.0 },
    Vector3 { x: -1.0, y: -1.0, z: 0.0 },
    Vector3 { x: 1.0, y: 0.0, z: 1.0 },
    Vector3 { x: -1.0, y: 0.0, z: 1.0 },
    Vector3 { x: 1.0, y: 0.0, z: -1.0 },
    Vector3 { x: -1.0, y: 0.0, z: -1.0 },
    Vector3 { x: 0.0, y: 1.0, z: 1.0 },
    Vector3 { x: 0.0, y: -1.0, z: 1.0 },
    Vector3 { x: 0.0, y: 1.0, z: -1.0 },
    Vector3 { x: 0.0, y: -1.0, z: -1.0 },

    // repetition
    Vector3 { x: 1.0, y: 1.0, z: 0.0 },
    Vector3 { x: -1.0, y: 1.0, z: 0.0 },
    Vector3 { x: 0.0, y: -1.0, z: 1.0 },
    Vector3 { x: 0.0, y: -1.0, z: -1.0 },
];
