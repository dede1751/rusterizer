use super::{PixelShader, ShaderGlobals};
use crate::primitives::{Float2, Float3};

#[derive(Debug)]
pub struct DepthShader();

impl PixelShader for DepthShader {
    fn pixel_color(
        &self,
        _pixel: Float2,
        _uv: Float2,
        _normal: Float3,
        depth: f32,
        _globals: &ShaderGlobals,
    ) -> Float3 {
        Float3::ONE * (depth / 5.0)
    }
}
