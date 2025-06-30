use engine::render_buffer::RenderBuffer;
use engine::scene::Scene;

#[derive(Debug,Default)]
struct Rasterizer{
    time: i32,
}

impl<const WIDTH: usize, const HEIGHT: usize> Scene<WIDTH, HEIGHT> for Rasterizer {
    fn update_state(&mut self, time: i32) {
        self.time = time;
    }

    fn render(&mut self, buffer: &mut RenderBuffer<WIDTH,HEIGHT>) {
        *buffer = RenderBuffer::test_frame(self.time as f32);
    }
}

fn main() {
    Scene::<640, 360>::run(&mut Rasterizer::default());
}
