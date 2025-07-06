use super::{Float2, Float3, Tri};

#[derive(Debug, Clone)]
pub struct FaceData2D {
    pub vertices: Tri<Float2>,
    pub depths: Tri<f32>,
    pub normals: Tri<Float3>,
    pub uvs: Tri<Float2>,
}

#[derive(Debug, Clone)]
pub struct FaceData3D {
    pub vertices: Tri<Float3>,
    pub normals: Tri<Float3>,
    pub uvs: Tri<Float2>,
}
