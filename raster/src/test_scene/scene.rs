use std::sync::Arc;

use engine::camera::CameraModel;
use engine::entity::Entity;
use engine::input::Input;
use engine::mesh::Mesh;
use engine::pose_graph::PoseGraph;
use engine::primitives::{Float3, Quaternion};
use engine::render_buffer::RenderBuffer;
use engine::scene::{Scene, SceneData};
use engine::shader::TextureShader;
use engine::texture::Texture;

use super::cam_controller::CamController;
use crate::raster::rasterize_scene;

#[derive(Debug)]
pub struct TestScene<const WIDTH: usize, const HEIGHT: usize> {
    data: SceneData<WIDTH, HEIGHT>,
    cam_controller: CamController<WIDTH, HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for TestScene<WIDTH, HEIGHT> {
    fn default() -> Self {
        // Setup pose graph
        let root = PoseGraph::root();
        let dagger_pose = PoseGraph::new("dagger", root.clone());
        let cam_pose = PoseGraph::new("cam", root.clone());
        cam_pose
            .borrow_mut()
            .apply_translation(Float3::new(0.0, -5.0, -9.5));
        dagger_pose
            .borrow_mut()
            .apply_rotation(Quaternion::from_z_angle(f32::to_radians(180.0)));

        // Load meshes
        let dagger_mesh = Arc::new(Mesh::from_obj_file("resources/models/dagger.obj").unwrap());

        // Load shaders
        let dagger_texture = Texture::from_png_file("resources/textures/dagger.png").unwrap();
        let dagger_shader = Arc::new(TextureShader::new(dagger_texture));

        // Create entities
        let dagger = Entity::new(dagger_pose, dagger_mesh.clone(), dagger_shader.clone());

        // Assemble scene data
        let mut data = SceneData {
            cam_model: CameraModel::new(60.0, true),
            cam_pose: cam_pose.clone(),
            ..Default::default()
        };
        data.entities.insert("dagger".to_string(), dagger);

        Self {
            data,
            cam_controller: CamController::new(cam_pose),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Scene<WIDTH, HEIGHT> for TestScene<WIDTH, HEIGHT> {
    fn update_state(&mut self, time_delta: f32, input: &mut Input) {
        let dagger = self.data.entities.get_mut("dagger").unwrap();
        dagger
            .pose
            .borrow_mut()
            .apply_rotation(Quaternion::from_y_angle(f32::to_radians(0.5)));

        self.cam_controller.update_camera(time_delta, input);
    }

    fn render(&mut self, buffer: &mut RenderBuffer<WIDTH, HEIGHT>) {
        rasterize_scene(&mut self.data, buffer);
    }
}
