use super::PixelShader;
use crate::primitives::{Float2, Float3};
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct TextureShader {
    texture: Texture,
}

impl PixelShader for TextureShader {
    fn pixel_color(&self, _pixel: Float2, uv: Float2, _normal: Float3, _depth: f32) -> Float3 {
        self.texture.sample(uv)
    }
}

impl TextureShader {
    pub fn new(texture: Texture) -> Self {
        Self { texture }
    }
}
