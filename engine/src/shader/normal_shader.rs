use super::{PixelShader, ShaderGlobals};
use crate::primitives::{Float2, Float3};

#[derive(Debug)]
pub struct NormalShader();

impl PixelShader for NormalShader {
    fn pixel_color(
        &self,
        _pixel: Float2,
        _uv: Float2,
        normal: Float3,
        _depth: f32,
        _globals: &ShaderGlobals,
    ) -> Float3 {
        normal
    }
}
