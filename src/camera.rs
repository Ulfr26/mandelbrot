use raylib::prelude::*;
use crate::*;

// Represents the position of the camera, and the dimensions of the viewport
// (in the complex plane, not in pixel space)
pub struct Camera {
    pub dim: [f64; 2],
    pub pos: [f64; 2],
}

impl Camera {
    fn new(dim: [f64; 2], pos: [f64; 2]) -> Camera {
        Camera {
            dim,
            pos,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
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

        // Change zoom level
        let scroll = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) as i32
            - rl.is_key_down(KeyboardKey::KEY_SPACE) as i32;
        if scroll > 0 {
            self.dim[0] *= ZOOM;
            self.dim[1] *= ZOOM;
        } else if scroll < 0 {
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
        )
    }
}