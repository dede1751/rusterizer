use raster::frame::Frame;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut last_time = Instant::now();
    let mut frames = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = Frame::test_frame(WIDTH as u32, HEIGHT as u32);
        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = frame.color[i].to_minifb_rgb();
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        frames += 1;
        let now = Instant::now();
        let elapsed = now - last_time;
        if elapsed >= Duration::from_millis(100) {
            let fps = frames as f32 / elapsed.as_secs_f32();
            window.set_title(&format!("Rusterizer - FPS: {:.2}", fps));
            frames = 0;
            last_time = now;
        }
    }
}
