use engine::camera::CameraModel;
use engine::entity::Entity;
use engine::pose_graph::{PoseGraph, SharedPGNode};
use engine::primitives::{Float2, Tri, VertexData2D};
use engine::render_buffer::RenderBuffer;
use engine::scene::SceneData;

use rayon::prelude::*;

use std::sync::Arc;

fn to_screen_space<const WIDTH: usize, const HEIGHT: usize>(
    entity: &Entity,
    cam_model: CameraModel<WIDTH, HEIGHT>,
    cam_pose: SharedPGNode,
) -> Vec<VertexData2D> {
    let mesh_to_cam = PoseGraph::relative_transform(&entity.pose, &cam_pose);

    entity
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
        .collect()
}

pub fn rasterize_scene<const WIDTH: usize, const HEIGHT: usize>(
    data: &mut SceneData<WIDTH, HEIGHT>,
    buffer: &mut RenderBuffer<WIDTH, HEIGHT>,
) {
    for entity in data.entities.values() {
        let screen_tris = to_screen_space(entity, data.cam_model, data.cam_pose.clone());
        let shader = Arc::clone(&entity.shader);
        let shade_fn = move |p, uv, norm, depth| shader.pixel_color(p, uv, norm, depth);

        screen_tris.par_iter().for_each(|d| {
            let (start_x, start_y, end_x, end_y) = d.vertices.bbox::<WIDTH, HEIGHT>();
            let inv_depth = &Tri::new(1.0, 1.0, 1.0) / &d.depths;
            let scaled_uv = &d.uvs * &inv_depth;
            let scaled_norms = &d.normals * &inv_depth;

            for y in start_y..=end_y {
                for x in start_x..=end_x {
                    let p = Float2::new(x as f32, y as f32);
                    let weights = d.vertices.to_barycentric(p);

                    // Check if the point is inside the triangle
                    if let Some(weights) = weights {
                        let depth = 1.0 / (&weights * &inv_depth).sum();
                        let idx = y * WIDTH + x;

                        let mut pixel = buffer.pixels[idx].lock();
                        if pixel.1 > depth {
                            // Only update if unoccluded
                            let uv = ((&scaled_uv * &weights).sum()) * depth;
                            let norm = ((&scaled_norms * &weights).sum()) * depth;
                            *pixel = (shade_fn(p, uv, norm, depth), depth);
                        }
                    }
                }
            }
        });
    }
}
