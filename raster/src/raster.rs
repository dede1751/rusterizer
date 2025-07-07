use rayon::prelude::*;

use std::sync::Arc;

use engine::camera::CameraModel;
use engine::entity::Entity;
use engine::pose_graph::{PoseGraph, SharedPGNode};
use engine::primitives::{FaceData2D, Float2, Transform, Tri};
use engine::render_buffer::RenderBuffer;
use engine::scene::SceneData;

const NEAR_CLIP: f32 = -0.01;

fn to_screen_space<const WIDTH: usize, const HEIGHT: usize>(
    entity: &Entity,
    cam_model: CameraModel<WIDTH, HEIGHT>,
    cam_pose: SharedPGNode,
) -> Vec<FaceData2D> {
    let vert_to_cam = PoseGraph::relative_transform(&entity.pose, &cam_pose);
    let norm_to_cam = Transform {
        rotation: vert_to_cam.rotation,
        ..Default::default()
    };

    entity
        .mesh
        .data
        .par_iter()
        .filter_map(|v| {
            let vert_cam = vert_to_cam.apply_tri(&v.vertices);
            if vert_cam.vertices.iter().any(|vert| vert.z >= NEAR_CLIP) {
                return None;
            }

            let vert_screen = cam_model.tri_to_screen(&vert_cam);
            if vert_screen.should_cull() {
                return None;
            }

            Some(FaceData2D {
                vertices: vert_screen,
                depths: Tri::new(
                    -vert_cam.vertices[0].z,
                    -vert_cam.vertices[1].z,
                    -vert_cam.vertices[2].z,
                ),
                normals: norm_to_cam.apply_tri(&v.normals),
                uvs: v.uvs.clone(),
            })
        })
        .collect()
}

pub fn rasterize_scene<const WIDTH: usize, const HEIGHT: usize>(
    data: &mut SceneData<WIDTH, HEIGHT>,
    buffer: &mut RenderBuffer<WIDTH, HEIGHT>,
) {
    let globals = &data.globals;

    for entity in data.entities.values() {
        let screen_tris = to_screen_space(entity, data.cam_model, data.cam_pose.clone());
        let shader = Arc::clone(&entity.shader);
        let shade_fn = move |p, uv, norm, depth| shader.pixel_color(p, uv, norm, depth, globals);

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
                        if depth < pixel.1 {
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
