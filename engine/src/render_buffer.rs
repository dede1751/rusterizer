use parking_lot::Mutex;

use crate::primitives::Float3;

#[derive(Debug)]
pub struct RenderBuffer<const WIDTH: usize, const HEIGHT: usize> {
    pub pixels: Vec<Mutex<(Float3, f32)>>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for RenderBuffer<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self {
            pixels: (0..WIDTH * HEIGHT)
                .map(|_| Mutex::new((Float3::ZERO, f32::INFINITY)))
                .collect(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> RenderBuffer<WIDTH, HEIGHT> {
    pub fn test_frame(t: f32) -> Self {
        let buffer: RenderBuffer<WIDTH, HEIGHT> = RenderBuffer::default();
        let color = Float3::new(((t * 50.0) % 255.0) / 255.0, 0.0, 0.0);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = y * WIDTH + x;
                let mut pixel = buffer.pixels[idx].lock();
                pixel.0 = color;
            }
        }
        buffer
    }

    pub fn to_rgba_buffer(&self, buffer: &mut [u8]) {
        for (i, pixel) in buffer.chunks_exact_mut(4).enumerate() {
            let (r, g, b, a) = self.pixels[i].lock().0.to_rgba_bytes();
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
            pixel[3] = a;
        }
    }

    pub fn clear(&mut self, bg: Float3) {
        for i in self.pixels.iter_mut() {
            let mut pixel = i.lock();
            *pixel = (bg, f32::INFINITY);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = RenderBuffer::<640, 480>::default();
        assert_eq!(buffer.pixels.len(), 640 * 480);
        assert_eq!(buffer.pixels[0].lock().0, Float3::ZERO);
        assert_eq!(buffer.pixels[0].lock().1, f32::INFINITY);
    }
}
