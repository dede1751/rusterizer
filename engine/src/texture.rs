use png::Decoder;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::primitives::{Float2, Float3};

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    scale: Float2,
    data: Vec<Float3>,
}

impl Texture {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let extension = path.as_ref().extension().and_then(|s| s.to_str());

        match extension {
            Some("png") => Self::from_png_file(path),
            Some("bytes") => Self::from_bytes_file(path),
            _ => Err("Unsupported extension".into()),
        }
    }

    fn from_png_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let decoder = Decoder::new(BufReader::new(file));
        let mut reader = decoder.read_info()?;

        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let (width, height) = (info.width as usize, info.height as usize);

        let chunk_size = match info.color_type {
            png::ColorType::Rgb => 3,
            png::ColorType::Rgba => 4,
            _ => return Err("Unsupported color type".into()),
        };

        // UV (0,0) is bottom left, PNG is top left
        // Process the buffer by reversing the order of rows to flip the image vertically.
        let data: Vec<Float3> = buf
            .chunks_exact(width * chunk_size)
            .rev()
            .flat_map(|row_bytes| {
                row_bytes.chunks_exact(chunk_size).map(|chunk| {
                    Float3::new(chunk[0] as f32, chunk[1] as f32, chunk[2] as f32) / 255.0
                })
            })
            .collect();

        if data.len() != width * height {
            return Err("Image data does not match expected dimensions".into());
        }

        Ok(Self {
            width,
            height,
            scale: Float2::new((width - 1) as f32, (height - 1) as f32),
            data,
        })
    }

    fn from_bytes_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        if bytes.len() < 4 {
            return Err("File too short to contain dimensions".into());
        }

        let width = bytes[0] as usize | ((bytes[1] as usize) << 8);
        let height = bytes[2] as usize | ((bytes[3] as usize) << 8);

        let data: Vec<Float3> = bytes[4..]
            .chunks_exact(3)
            .map(|chunk| Float3::new(chunk[0] as f32, chunk[1] as f32, chunk[2] as f32) / 255.0)
            .collect();

        Ok(Self {
            width,
            height,
            scale: Float2::new((width - 1) as f32, (height - 1) as f32),
            data,
        })
    }

    pub fn sample(&self, uv: Float2) -> Float3 {
        let uv_idx = (uv - uv.floor()) * self.scale; // Support wrapping textures.
        let (x, y) = (uv_idx.x as usize, uv_idx.y as usize);

        self.data[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_from_png() {
        let texture = Texture::from_file("../resources/textures/dagger.png");
        assert!(
            texture.is_ok(),
            "Failed to load texture: {:?}",
            texture.err()
        );
        let texture = texture.unwrap();
        assert_eq!(texture.width, 512);
        assert_eq!(texture.height, 512);
        assert!(!texture.data.is_empty(), "Texture data should not be empty");
    }

    #[test]
    fn test_texture_from_bytes() {
        let texture = Texture::from_file("../resources/textures/daveTex.bytes");
        assert!(
            texture.is_ok(),
            "Failed to load texture: {:?}",
            texture.err()
        );
        let texture = texture.unwrap();
        assert_eq!(texture.width, 1024);
        assert_eq!(texture.height, 1024);
        assert!(!texture.data.is_empty(), "Texture data should not be empty");
    }

    #[test]
    fn test_sample() {
        let texture = Texture::from_file("../resources/textures/dagger.png").unwrap();

        // Bottom-right (PNG) corner: DimGray
        let uv = Float2::new(510.9, 0.1) / 511.0;
        assert_eq!(texture.sample(uv), Float3::new(80.0, 57.0, 44.0) / 255.0);

        // Top-right (PNG) random pixel: Maroon
        let uv = Float2::new(443.0, 377.0) / 511.0;
        assert_eq!(texture.sample(uv), Float3::new(79.0, 33.0, 33.0) / 255.0);
    }
}
