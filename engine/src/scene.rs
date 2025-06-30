use crate::primitives::Float3;
use crate::entity::Entity;
use crate::camera::CameraModel;
use crate::pose_graph::SharedPGNode;
use crate::render_buffer::RenderBuffer;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

#[derive(Debug,Clone)]
pub struct SceneData<const WIDTH: usize, const HEIGHT: usize> {
    pub entities: Vec<Entity>,
    pub cam_model: CameraModel<WIDTH,HEIGHT>,
    pub cam_pose: SharedPGNode,
}

pub trait Scene<const WIDTH: usize, const HEIGHT: usize> {
    fn update_state(&mut self, time: i32);

    fn render(&mut self, buffer: &mut RenderBuffer<WIDTH,HEIGHT>);

    fn run(&mut self) {
        let mut minifb_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut render_buffer: RenderBuffer<WIDTH,HEIGHT> = RenderBuffer::default();
        let mut window =
            Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        let mut last_time = Instant::now();
        let mut frames = 0;
        let mut time = 0;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.update_state(time);
            self.render(&mut render_buffer);
            render_buffer.to_minifb(&mut minifb_buffer);
            //render_buffer.clear(Float3::ZERO);

            window.update_with_buffer(&minifb_buffer, WIDTH, HEIGHT).unwrap();

            frames += 1;
            let now = Instant::now();
            let elapsed = now - last_time;
            if elapsed >= Duration::from_millis(100) {
                let fps = frames as f32 / elapsed.as_secs_f32();
                window.set_title(&format!("Rusterizer - FPS: {:.2}", fps));
                frames = 0;
                last_time = now;
            }
            time += 1;
        }
    }
}
