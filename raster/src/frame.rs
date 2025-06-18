use crate::primitives::Float3;

#[derive(Debug, Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub color: Vec<Float3>,
    pub depth: Vec<f32>,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        let color = vec![Float3::ZERO; (width * height) as usize];
        let depth = vec![f32::INFINITY; (width * height) as usize];
        Frame {
            width,
            height,
            color,
            depth,
        }
    }

    pub fn test_frame(width: u32, height: u32) -> Self {
        let mut frame = Frame::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let r = x as f32 / (width - 1) as f32;
                let g = y as f32 / (height - 1) as f32;
                frame.color[idx] = Float3::new(r, g, 0.0);
            }
        }
        frame
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
    fn test_frame_creation() {
        let frame = Frame::new(640, 480);
        assert_eq!(frame.width, 640);
        assert_eq!(frame.height, 480);
        assert_eq!(frame.color.len(), 640 * 480);
        assert_eq!(frame.depth.len(), 640 * 480);
        assert_eq!(frame.color[0], Float3::ZERO);
        assert_eq!(frame.depth[0], f32::INFINITY);
    }

    
}
