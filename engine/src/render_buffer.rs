use crate::primitives::Float3;

#[derive(Debug, Clone)]
pub struct RenderBuffer<const WIDTH: usize, const HEIGHT: usize> {
    pub color: Vec<Float3>,
    pub depth: Vec<f32>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for RenderBuffer<WIDTH, HEIGHT> {
    fn default() -> Self {
        let color = vec![Float3::ZERO; WIDTH * HEIGHT];
        let depth = vec![f32::INFINITY; WIDTH * HEIGHT];
        RenderBuffer { color, depth }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> RenderBuffer<WIDTH, HEIGHT> {
    pub fn test_frame(t: f32) -> Self {
        let mut buffer: RenderBuffer<WIDTH, HEIGHT> = RenderBuffer::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = y * WIDTH + x;
                buffer.color[idx] = Float3::new((t % 255.0) / 255.0, 0.0, 0.0); // Red color
            }
        }
        buffer
    }

    pub fn to_minifb(&self, minifb_buffer: &mut [u32]) {
        for (i, pixel) in minifb_buffer.iter_mut().enumerate() {
            *pixel = self.color[i].to_minifb_rgb();
        }
    }

    pub fn clear(&mut self, bg: Float3) {
        for i in 0..self.color.len() {
            self.color[i] = bg;
        }

        for i in 0..self.depth.len() {
            self.depth[i] = f32::INFINITY;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = RenderBuffer::<640, 480>::default();
        assert_eq!(buffer.color.len(), 640 * 480);
        assert_eq!(buffer.depth.len(), 640 * 480);
        assert_eq!(buffer.color[0], Float3::ZERO);
        assert_eq!(buffer.depth[0], f32::INFINITY);
    }
}
