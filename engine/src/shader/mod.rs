pub use depth_shader::DepthShader;
pub use lit_texture_shader::LitTextureShader;
pub use normal_shader::NormalShader;
pub use texture_shader::TextureShader;

mod depth_shader;
mod lit_texture_shader;
mod normal_shader;
mod texture_shader;

use crate::primitives::{Float2, Float3};

// Global scene information which can be used by the shader.
#[derive(Debug, Default, Clone)]
pub struct ShaderGlobals {
    pub sun_direction_cam_space: Float3,
    pub time: f32,
}

pub trait PixelShader: std::fmt::Debug + Sync + Send {
    fn pixel_color(
        &self,
        pixel: Float2,
        uv: Float2,
        normal: Float3,
        depth: f32,
        globals: &ShaderGlobals,
    ) -> Float3;
}
