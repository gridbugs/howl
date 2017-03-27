use rand::{StdRng, SeedableRng};
use perlin::*;

const ZOOM: usize = 10;
const ZOOM_F: f64 = ZOOM as f64;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

#[test]
fn demo() {
    let mut rng = StdRng::from_seed(&[0]);
    let perlin = PerlinGrid::new(WIDTH, HEIGHT, PerlinWrapType::Regenerate, &mut rng);

    for i in 0..(ZOOM * HEIGHT) {
        for j in 0..(ZOOM * WIDTH) {
            let noise = perlin.noise(j as f64 / ZOOM_F,
                                     i as f64 / ZOOM_F).unwrap();

            if noise < -0.2 || noise > 0.2 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
