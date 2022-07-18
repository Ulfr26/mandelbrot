mod camera;

use itertools::Itertools;
use raylib::prelude::*;
use rayon::prelude::*;
use camera::Camera;

// Default values
const HUE: f32 = 252.;
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 960;
const RANGE_X: (f64, f64) = (-2.0, 0.47);
const RANGE_Y: (f64, f64) = (-1.12, 1.12);
const MAX_ITERATIONS: i32 = 64;
const SPEED: f64 = 200.0;
const ZOOM: f64 = 0.9; // Sheogorath!
const ITER_SPEED: i32 = 64; // Add this many iterations per button press

// does ????
// stolen from wikipedia
fn mandelbrot(re: f64, im: f64, max_iterations: i32) -> u8 {
    let (mut x, mut y, mut x2, mut y2) = (0., 0., 0., 0.);
    let mut iterations = 0;

    while x*x + y*y <= 4.0 && iterations < max_iterations {
        y = 2.0 * x * y + im;
        x = x2 - y2 + re;
        x2 = x * x;
        y2 = y * y;
        iterations += 1;
    }

    ((iterations as f32 / max_iterations as f32) * 255.0) as u8
}

fn pixel_to_real(pixel: (i32, i32), dim: &[f64; 2], pos: &[f64; 2]) -> (f64, f64) {
    let res = (
        ((pixel.0 - (WIDTH / 2)) as f64) / WIDTH as f64,
        ((pixel.1 - (HEIGHT/ 2)) as f64) / HEIGHT as f64
    );

    (res.0 * dim[0] + pos[0], res.1 * dim[1] + pos[1])
}

fn update_iterations(rl: &RaylibHandle, iterations: &mut i32) {
    let change = rl.is_key_down(KeyboardKey::KEY_UP) as i32 
        - rl.is_key_down(KeyboardKey::KEY_DOWN) as i32;

    *iterations += change * ITER_SPEED;
    *iterations = (*iterations).max(0);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("mandelbrot")
        .build();

    let mut camera = Camera::default();

    let pallette: Vec<Color> = (0..256).map(|x| {
        Color::color_from_hsv(HUE, 1., x as f32 / 255.0)
    }).collect();
    let mut iterations = MAX_ITERATIONS;

    while !rl.window_should_close() {
        camera.update(&rl);
        update_iterations(&rl, &mut iterations);

        let canvas = (0..HEIGHT)
            .cartesian_product(0..WIDTH)
            .collect_vec()
            .into_par_iter()
            .map(|(x, y)| pixel_to_real((y, x), &camera.dim, &camera.pos))
            .map(|(x, y)| mandelbrot(x, y, iterations))
            .collect::<Vec<u8>>();

        let mut draw_handle = rl.begin_drawing(&thread);

        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let level = canvas[(j * WIDTH + i) as usize] as usize;
                let col = pallette[level];
                draw_handle.draw_pixel(i, j, col);
            }
        }

        draw_handle.draw_text(&format!("Iterations: {}", iterations), 10, 10, 30,
                    Color::WHITE);
        draw_handle.draw_fps(WIDTH - 100, 10);
    }
}
