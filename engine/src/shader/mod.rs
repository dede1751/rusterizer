pub use texture_shader::TextureShader;
pub use normal_shader::NormalShader;

mod texture_shader;
mod normal_shader;

use crate::primitives::{Float2, Float3};

pub trait PixelShader: std::fmt::Debug + Sync + Send {
    fn pixel_color(&self, pixel: Float2, uv: Float2, normal: Float3, depth: f32) -> Float3;
}
