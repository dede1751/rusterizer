use crate::primitives::{Float2, Float3};
use super::PixelShader;

#[derive(Debug)]
pub struct NormalShader();

impl PixelShader for NormalShader {
    fn pixel_color(&self, _pixel: Float2, _uv: Float2, normal: Float3, _depth: f32) -> Float3 {
        normal
    }
}