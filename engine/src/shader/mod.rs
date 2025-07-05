use crate::primitives::{Float2, Float3};

pub mod texture_shader;

pub use texture_shader::TextureShader;

pub trait PixelShader: std::fmt::Debug + Sync + Send {
    fn pixel_color(&self, pixel: Float2, uv: Float2, normal: Float3, depth: f32) -> Float3;
}
