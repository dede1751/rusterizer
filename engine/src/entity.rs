use std::sync::Arc;

use crate::mesh::Mesh;
use crate::pose_graph::SharedPGNode;
use crate::shader::PixelShader;

#[derive(Debug, Clone)]
pub struct Entity {
    pub pose: SharedPGNode,
    pub mesh: Arc<Mesh>,
    pub shader: Arc<dyn PixelShader + Sync + Send>,
}

impl Entity {
    pub fn new(pose: SharedPGNode, mesh: Arc<Mesh>, shader: Arc<dyn PixelShader>) -> Self {
        Self { pose, mesh, shader }
    }
}
