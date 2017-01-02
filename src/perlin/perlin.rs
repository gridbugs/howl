use grid::{Grid, StaticGrid, IterGrid};
use math::{Dot, Vector2, Vector3};
use coord::Coord;

use rand::{Rng, StdRng, SeedableRng};

use std::fmt;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PerlinWrapType {
    Repeat,
    Regenerate,
}

#[derive(Clone, Copy, Debug)]
struct PerlinVector(Vector3<f64>);

impl PerlinVector {
    fn new(rng: &mut StdRng) -> Self {
        // random number from 0 to 15
        let index = rng.gen::<usize>() & GRADIENT_MASK;
        PerlinVector(GRADIENTS[index].normalize())
    }
}

#[derive(Debug, Clone)]
struct PerlinSlice {
    grid: StaticGrid<PerlinVector>,
    z: f64,
}

impl PerlinSlice {
    fn reset(&mut self, rng: &mut StdRng) {
        for cell in self.grid.iter_mut() {
            *cell = PerlinVector::new(rng);
        }
    }
}

#[derive(Clone)]
pub struct PerlinGrid {
    slices: Vec<PerlinSlice>,
    grid_width: usize,
    grid_height: usize,
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
    minor_offset: Vector2<f64>,
    major_offset: Vector2<usize>,
    wrap_type: PerlinWrapType,
    rng: StdRng,
}

const NUM_CORNERS: usize = 8;
const NUM_SLICE_CORNERS: usize = 4;
const NUM_SLICES: usize = 2;

pub fn ease_curve(x: f64) -> f64 {
    6.0 * x.powi(5) - 15.0 * x.powi(4) + 10.0 * x.powi(3)
}

impl PerlinGrid {
    pub fn new(width: usize, height: usize, wrap_type: PerlinWrapType) -> io::Result<Self> {
        let rng = try!(StdRng::new());
        Ok(PerlinGrid::new_with_rng(width, height, wrap_type, rng))
    }

    pub fn new_from_seed(width: usize,
                         height: usize,
                         wrap_type: PerlinWrapType,
                         seed: usize)
                         -> Self {
        let rng = StdRng::from_seed(&[seed]);
        PerlinGrid::new_with_rng(width, height, wrap_type, rng)
    }

    fn new_with_rng(width: usize,
                    height: usize,
                    wrap_type: PerlinWrapType,
                    mut rng: StdRng)
                    -> PerlinGrid {
        let grid_width = width + 2;
        let grid_height = height + 2;
        PerlinGrid {
            slices: {
                let mut v = Vec::with_capacity(NUM_SLICES);
                for i in 0..2 as isize {
                    v.push(PerlinSlice {
                        grid: StaticGrid::new_call(grid_width,
                                                   grid_height,
                                                   |_, _| PerlinVector::new(&mut rng)),
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
            rng: rng,
        }
    }

    fn make_vector(&mut self) -> PerlinVector {
        PerlinVector::new(&mut self.rng)
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
            self.slices[0].reset(&mut self.rng);
            self.swap_slices();
        } else if self.z < 0.0 && self.z >= -1.0 {
            self.z += 1.0;
            self.slices[1].reset(&mut self.rng);
            self.swap_slices();
        } else if self.z > 2.0 || self.z < -1.0 {
            self.z = 0.0;
            for slice in self.slices.iter_mut() {
                slice.reset(&mut self.rng);
            }
        }
    }

    pub fn scroll(&mut self, x: f64, y: f64) {
        self.minor_offset.x += x;
        self.minor_offset.y += y;

        let floor_f = Vector2::new(self.minor_offset.x.floor(), self.minor_offset.y.floor());

        let floor_i = Vector2::new(floor_f.x as usize, floor_f.y as usize);

        if floor_i.x != 0 {
            if self.wrap_type == PerlinWrapType::Regenerate {
                if floor_i.x > 0 {
                    for i in (self.major_offset.x)..(self.major_offset.x + floor_i.x) {
                        let x = ((i + self.grid_width) % self.grid_width) as isize;
                        for j in 0..self.grid_height {
                            let coord = Coord::new(x, j as isize);
                            unsafe {
                                for z in 0..NUM_SLICES {
                                    *self.slices[z].grid.get_unchecked_mut(coord) = self.make_vector();
                                }
                            }
                        }
                    }
                } else {
                    for i in (self.major_offset.x + floor_i.x)..(self.major_offset.x) {
                        let x = ((i + self.grid_width) % self.grid_width) as isize;
                        for j in 0..self.grid_height {
                            let coord = Coord::new(x, j as isize);
                            unsafe {
                                for z in 0..NUM_SLICES {
                                    *self.slices[z].grid.get_unchecked_mut(coord) = self.make_vector();
                                }
                            }
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
                        let y = ((i + self.grid_height) % self.grid_height) as isize;
                        for j in 0..self.grid_width {
                            let coord = Coord::new(j as isize, y);
                            unsafe {
                                for z in 0..NUM_SLICES {
                                    *self.slices[z].grid.get_unchecked_mut(coord) = self.make_vector();
                                }
                            }
                        }
                    }
                } else {
                    for i in (self.major_offset.y + floor_i.y)..(self.major_offset.y) {
                        let y = ((i + self.grid_height) % self.grid_height) as isize;
                        for j in 0..self.grid_width {
                            let coord = Coord::new(j as isize, y);
                            unsafe {
                                for z in 0..NUM_SLICES {
                                    *self.slices[z].grid.get_unchecked_mut(coord) = self.make_vector();
                                }
                            }
                        }
                    }
                }
            }
            self.major_offset.y += floor_i.y;
            self.major_offset.y = (self.major_offset.y + self.grid_height) % self.grid_height;
        }

        self.minor_offset -= floor_f;

    }

    pub fn noise(&self, global_x: f64, global_y: f64) -> Option<f64> {
        assert!(self.z >= 0.0 && self.z <= self.depth);

        let x = global_x + self.minor_offset.x;
        let y = global_y + self.minor_offset.y;

        if x > self.width || y > self.height {
            return None;
        }

        let global_coord = Vector3::new(x, y, self.z);

        let top_left_f = Vector2::new(x.floor(), y.floor());
        let top_left_i = Vector2::<isize>::from(self.major_offset) +
                         Vector2::new(top_left_f.x as isize, top_left_f.y as isize);

        let mut corner_coords_i = [top_left_i,
                                   top_left_i + Vector2::new(1, 0),
                                   top_left_i + Vector2::new(0, 1),
                                   top_left_i + Vector2::new(1, 1)];

        for corner_coord_i in &mut corner_coords_i {
            corner_coord_i.x %= self.grid_width as isize;
            corner_coord_i.y %= self.grid_height as isize;
        }

        let corner_coords_f = [top_left_f,
                               top_left_f + Vector2::new(1.0, 0.0),
                               top_left_f + Vector2::new(0.0, 1.0),
                               top_left_f + Vector2::new(1.0, 1.0)];

        let mut dots: [f64; NUM_CORNERS] = [0.0; NUM_CORNERS];

        let mut i = 0;
        while i != NUM_CORNERS {
            let slice = &self.slices[i / NUM_SLICE_CORNERS];
            let corner_coord_i = corner_coords_i[i % NUM_SLICE_CORNERS];
            let corner_coord_f = corner_coords_f[i % NUM_SLICE_CORNERS];

            let gradient = unsafe { slice.grid.get_unchecked(corner_coord_i).0 };
            let corner_coord_f3 = Vector3::new(corner_coord_f.x, corner_coord_f.y, slice.z);
            let relative = global_coord - corner_coord_f3;
            dots[i] = gradient.dot(relative);

            i += 1;
        }

        let weight_x = ease_curve(x - top_left_f.x);
        let weight_y = ease_curve(y - top_left_f.y);
        let weight_z = ease_curve(self.z.fract());

        let square_avgs = [dots[0] + weight_z * (dots[4] - dots[0]),
                           dots[1] + weight_z * (dots[5] - dots[1]),
                           dots[2] + weight_z * (dots[6] - dots[2]),
                           dots[3] + weight_z * (dots[7] - dots[3])];

        let line_avgs = [square_avgs[0] + weight_x * (square_avgs[1] - square_avgs[0]),
                         square_avgs[2] + weight_x * (square_avgs[3] - square_avgs[2])];

        let avg = line_avgs[0] + weight_y * (line_avgs[1] - line_avgs[0]);

        Some(avg.max(-1.0).min(1.0))
    }
}

impl fmt::Debug for PerlinGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PerlinGrid")
    }
}

const NUM_GRADIENTS: usize = 16;
const GRADIENT_MASK: usize = 0xf;
static GRADIENTS: [Vector3<f64>; NUM_GRADIENTS] = [
    Vector3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    },
    Vector3 {
        x: -1.0,
        y: 1.0,
        z: 0.0,
    },
    Vector3 {
        x: 1.0,
        y: -1.0,
        z: 0.0,
    },
    Vector3 {
        x: -1.0,
        y: -1.0,
        z: 0.0,
    },
    Vector3 {
        x: 1.0,
        y: 0.0,
        z: 1.0,
    },
    Vector3 {
        x: -1.0,
        y: 0.0,
        z: 1.0,
    },
    Vector3 {
        x: 1.0,
        y: 0.0,
        z: -1.0,
    },
    Vector3 {
        x: -1.0,
        y: 0.0,
        z: -1.0,
    },
    Vector3 {
        x: 0.0,
        y: 1.0,
        z: 1.0,
    },
    Vector3 {
        x: 0.0,
        y: -1.0,
        z: 1.0,
    },
    Vector3 {
        x: 0.0,
        y: 1.0,
        z: -1.0,
    },
    Vector3 {
        x: 0.0,
        y: -1.0,
        z: -1.0,
    },

    // repetition
    Vector3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    },
    Vector3 {
        x: -1.0,
        y: 1.0,
        z: 0.0,
    },
    Vector3 {
        x: 0.0,
        y: -1.0,
        z: 1.0,
    },
    Vector3 {
        x: 0.0,
        y: -1.0,
        z: -1.0,
    }
];
