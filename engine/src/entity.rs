use crate::camera::CameraModel;
use crate::mesh::Mesh;
use crate::pose_graph::{PoseGraph, SharedPGNode};
use crate::primitives::{Float2, Float3, Tri};

use std::path::Path;

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct VertexData2D {
    pub vertices: Tri<Float2>,
    pub depths: Tri<f32>,
    pub normals: Tri<Float3>,
    pub uvs: Tri<Float2>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub mesh: Mesh,
    pub pose: SharedPGNode,
    pub name: String,

    pub screen_tris: Vec<VertexData2D>,
}

impl Entity {
    pub fn new<P: AsRef<Path>>(
        name: &str,
        obj_file: P,
        pose: SharedPGNode,
    ) -> std::io::Result<Self> {
        let mesh = Mesh::from_obj_file(obj_file)?;

        Ok(Self {
            mesh,
            pose,
            name: name.to_string(),
            screen_tris: Vec::new(),
        })
    }

    pub fn update_screen_tris<const WIDTH: usize, const HEIGHT: usize>(
        &mut self,
        cam_model: CameraModel<WIDTH, HEIGHT>,
        cam_pose: SharedPGNode,
    ) {
        let mesh_to_cam = PoseGraph::relative_transform(&self.pose, &cam_pose);

        self.screen_tris = self
            .mesh
            .data
            .par_iter()
            .map(|v| {
                let tri_cam = mesh_to_cam.apply_tri(&v.vertices);
                let tri_screen = cam_model.tri_to_screen(&tri_cam);
                let depths = Tri::new(v.vertices[0].z, v.vertices[1].z, v.vertices[2].z);

                VertexData2D {
                    vertices: tri_screen,
                    depths,
                    normals: v.normals.clone(),
                    uvs: v.uvs.clone(),
                }
            })
            .collect();
    }
}
