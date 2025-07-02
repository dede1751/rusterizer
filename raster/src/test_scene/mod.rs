use engine::input::Input;
use engine::render_buffer::RenderBuffer;
use engine::scene::Scene;

#[derive(Debug, Default)]
pub struct TestScene {
    time: f32,
}

impl<const WIDTH: usize, const HEIGHT: usize> Scene<WIDTH, HEIGHT> for TestScene {
    fn update_state(&mut self, time: f32, _: &mut Input) {
        self.time += time;
    }

    fn render(&mut self, buffer: &mut RenderBuffer<WIDTH, HEIGHT>) {
        *buffer = RenderBuffer::test_frame(self.time);
    }
}
