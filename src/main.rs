use raylib::prelude::*;

// Default values
const HUE: f32 = 252.;
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 960;
const RANGE_X: (f64, f64) = (-2.0, 0.47);
const RANGE_Y: (f64, f64) = (-1.12, 1.12);
const MAX_ITERATIONS: i32 = 64;
const SPEED: f64 = 200.0;
const ZOOM: f64 = 0.8; // Sheogorath!
const ITER_SPEED: f64 = 10.0; // Cycle iterations at this many cps

// Represents the position of the camera, and the dimensions of the viewport
// (in the complex plane, not in pixel space)
struct Camera {
    dim: [f64; 2],
    pos: [f64; 2],
    iterations: i32,
}

impl Camera {
    fn new(dim: [f64; 2], pos: [f64; 2], iterations: i32) -> Camera {
        Camera {
            dim,
            pos,
            iterations,
        }
    }

    fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time() as f64;
        // Update psoition
        // I know i misspelled that word but im keeping it cause its funni
        let hdir = (rl.is_key_down(KeyboardKey::KEY_D) as i32
            - rl.is_key_down(KeyboardKey::KEY_A) as i32) as f64;

        let vdir = (rl.is_key_down(KeyboardKey::KEY_S) as i32
            - rl.is_key_down(KeyboardKey::KEY_W) as i32) as f64;

        let dx = hdir * SPEED * dt / WIDTH as f64;
        let dy = vdir * SPEED * dt / HEIGHT as f64;

        self.pos[0] += dx * self.dim[0];
        self.pos[1] += dy * self.dim[1];

        // Change iteration level
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.iterations += ((ITER_SPEED * dt) as i32).max(1);
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.iterations -= ((ITER_SPEED * dt) as i32)
                .clamp(1, self.iterations);
        }

        self.iterations = self.iterations.max(1);

        // Change zoom level
        let scroll = rl.get_mouse_wheel_move();
        if scroll > 0.0 {
            self.dim[0] *= ZOOM;
            self.dim[1] *= ZOOM;
        } else if scroll < 0.0 {
            self.dim[0] /= ZOOM;
            self.dim[1] /= ZOOM;
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            [RANGE_X.1 - RANGE_X.0, RANGE_Y.1 - RANGE_Y.0],
            [(RANGE_X.0 + RANGE_X.1) / 2., (RANGE_X.0 + RANGE_X.1) / 2.],
            MAX_ITERATIONS
        )
    }
}

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

    ((iterations as f32 / max_iterations as f32) * 256.0) as u8
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("mandelbrot")
        .build();

    let mut c = Camera::default();

    let pallette: Vec<Color> = (0..256).map(|x| {
        Color::color_from_hsv(HUE, 1., x as f32 / 255.0)
    }).collect();

    while !rl.window_should_close() {
        c.update(&rl);
        let mut d = rl.begin_drawing(&thread);

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let x = ((i - (WIDTH / 2)) as f64) / WIDTH as f64;
                let y = ((j - (HEIGHT / 2)) as f64) / HEIGHT as f64;

                let x = x * c.dim[0] + c.pos[0];
                let y = y * c.dim[1] + c.pos[1];

                let col = pallette[mandelbrot(x, y, c.iterations) as usize];
                d.draw_pixel(i, j, col);
            }
        }

        d.draw_text(&format!("Iterations: {}", c.iterations), 10, 10, 30,
                    Color::WHITE);
        d.draw_fps(WIDTH - 100, 10);
    }
}
