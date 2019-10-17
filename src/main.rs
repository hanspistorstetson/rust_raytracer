extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (index, pixel) in buffer.iter_mut().enumerate() {
            let x = index % WIDTH;
            let y = HEIGHT - (index / WIDTH);

            let float_r = x as f64 / WIDTH as f64;
            let float_g = y as f64 / HEIGHT as f64;
            let float_b = 0.2;

            let r = (255.99 * float_r) as u32;
            let g = (255.99 * float_g) as u32;
            let b = (255.99 * float_b) as u32;

            let rgb = (r << 16) | (g << 8) | b;

            *pixel = rgb;
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}
