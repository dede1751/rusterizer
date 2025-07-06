use super::{PixelShader, ShaderGlobals};
use crate::primitives::{Float2, Float3, VectorOps};
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct LitTextureShader {
    texture: Texture,
}

impl PixelShader for LitTextureShader {
    fn pixel_color(
        &self,
        _pixel: Float2,
        uv: Float2,
        normal: Float3,
        _depth: f32,
        globals: &ShaderGlobals,
    ) -> Float3 {
        let intensity = 0.5 * (1.0 + normal.normalized().dot(globals.sun_direction_cam_space));
        let scaled_intensity = 0.4 + 0.6 * intensity.clamp(0.0, 1.0);
        self.texture.sample(uv) * scaled_intensity
    }
}

impl LitTextureShader {
    pub fn new(texture: Texture) -> Self {
        Self { texture }
    }
}
