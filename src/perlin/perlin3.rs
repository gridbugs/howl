use grid::static_grid::StaticGrid;
use geometry::vector::Dot;
use geometry::vector2::Vector2;
use geometry::vector3::Vector3;
use perlin::perlin::ease_curve;

use rand;
use rand::Rng;

pub type Coord = Vector2<f64>;

#[derive(Clone, Copy, Debug)]
struct Perlin3Vector(Vector3<f64>);

impl Default for Perlin3Vector {
    fn default() -> Self {
        // random number from 0 to 15
        let index: usize = rand::thread_rng().gen::<usize>() & GRADIENT_MASK;
        GRADIENTS[index]
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
    slice_depth: f64,
    width: f64,
    height: f64,
}

const NUM_CORNERS: usize = 8;

impl Perlin3Grid {
    pub fn new(width: f64, height: f64, slice_depth: f64) -> Perlin3Grid {
        let limit_width = width.ceil() as isize + 2;
        let limit_height = height.ceil() as isize + 2;
        Perlin3Grid {
            slices: vec![
                Perlin3Slice {
                    grid: StaticGrid::new_default(limit_width, limit_height),
                    z: 0.0,
                },
                Perlin3Slice {
                    grid: StaticGrid::new_default(limit_width, limit_height),
                    z: 1.0,
                },
            ],
            width: width,
            height: height,
            slice_depth: slice_depth,
        }
    }

    pub fn noise(&self, global_coord_2d: Coord) -> Option<f64> {
        assert!(self.slice_depth >= 0.0 && self.slice_depth <= 1.0);

        let Coord { x, y } = global_coord_2d;
        if x > self.width || y > self.height {
            return None;
        }

        let global_coord = Vector3::new(x, y, self.slice_depth);

        let top_left_f = Vector2::new(x.floor(), y.floor());
        let top_left_i = Vector2::new(top_left_f.x as isize,
                                      top_left_f.y as isize);

        let corner_coords_i = [
            top_left_i,
            top_left_i + Vector2::new(1, 0),
            top_left_i + Vector2::new(0, 1),
            top_left_i + Vector2::new(1, 1),
        ];

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
            let gradient = slice.grid.get(*corner_coord_i).unwrap().0;
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
        let weight_z = ease_curve(self.slice_depth);

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

        Some(avg)
    }
}

const NUM_GRADIENTS: usize = 16;
const GRADIENT_MASK: usize = 0xf;
static GRADIENTS: [Perlin3Vector; NUM_GRADIENTS] = [
    Perlin3Vector(Vector3 { x: 1.0, y: 1.0, z: 0.0 }),
    Perlin3Vector(Vector3 { x: -1.0, y: 1.0, z: 0.0 }),
    Perlin3Vector(Vector3 { x: 1.0, y: -1.0, z: 0.0 }),
    Perlin3Vector(Vector3 { x: -1.0, y: -1.0, z: 0.0 }),
    Perlin3Vector(Vector3 { x: 1.0, y: 0.0, z: 1.0 }),
    Perlin3Vector(Vector3 { x: -1.0, y: 0.0, z: 1.0 }),
    Perlin3Vector(Vector3 { x: 1.0, y: 0.0, z: -1.0 }),
    Perlin3Vector(Vector3 { x: -1.0, y: 0.0, z: -1.0 }),
    Perlin3Vector(Vector3 { x: 0.0, y: 1.0, z: 1.0 }),
    Perlin3Vector(Vector3 { x: 0.0, y: -1.0, z: 1.0 }),
    Perlin3Vector(Vector3 { x: 0.0, y: 1.0, z: -1.0 }),
    Perlin3Vector(Vector3 { x: 0.0, y: -1.0, z: -1.0 }),

    // repetition
    Perlin3Vector(Vector3 { x: 1.0, y: 1.0, z: 0.0 }),
    Perlin3Vector(Vector3 { x: -1.0, y: 1.0, z: 0.0 }),
    Perlin3Vector(Vector3 { x: 0.0, y: -1.0, z: 1.0 }),
    Perlin3Vector(Vector3 { x: 0.0, y: -1.0, z: -1.0 }),
];
