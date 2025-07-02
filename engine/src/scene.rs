use crate::camera::CameraModel;
use crate::entity::Entity;
use crate::input::Input;
use crate::pose_graph::SharedPGNode;
use crate::primitives::Float3;
use crate::render_buffer::RenderBuffer;

use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct SceneData<const WIDTH: usize, const HEIGHT: usize> {
    pub entities: Vec<Entity>,
    pub cam_model: CameraModel<WIDTH, HEIGHT>,
    pub cam_pose: SharedPGNode,
}

pub trait Scene<const WIDTH: usize, const HEIGHT: usize> {
    fn update_state(&mut self, delta: f32, rl: &mut Input);

    fn render(&mut self, buffer: &mut RenderBuffer<WIDTH, HEIGHT>);

    fn run(&mut self) {
        let (mut rl, thread) = raylib::init()
            .size(WIDTH as i32, HEIGHT as i32)
            .title("Rusterizer")
            .build();

        let image = Image::gen_image_color(WIDTH as i32, HEIGHT as i32, Color::BLACK);
        let mut texture = rl.load_texture_from_image(&thread, &image).unwrap();
        let mut render_buffer = RenderBuffer::<WIDTH, HEIGHT>::default();
        let mut frame_buffer = vec![0u8; 4 * WIDTH * HEIGHT];

        while !rl.window_should_close() {
            let delta = rl.get_frame_time();
            let mut input = Input::new(&mut rl);
            self.update_state(delta, &mut input);
            self.render(&mut render_buffer);

            render_buffer.to_rgba_bytes(&mut frame_buffer);
            texture.update_texture(&frame_buffer).unwrap();

            let mut d = rl.begin_drawing(&thread);
            let src = Rectangle {
                x: 0.0,
                y: 0.0,
                width: WIDTH as f32,
                height: HEIGHT as f32,
            };
            let dest = Rectangle {
                x: 0.0,
                y: 0.0,
                width: d.get_screen_width() as f32,
                height: d.get_screen_height() as f32,
            };

            d.draw_texture_pro(&texture, src, dest, Vector2::zero(), 0.0, Color::WHITE);
            d.draw_fps(10, 10);
            render_buffer.clear(Float3::ZERO);
        }
    }
}
