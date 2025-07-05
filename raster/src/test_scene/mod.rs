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

use crate::raster::rasterize_scene;

use std::sync::Arc;

#[derive(Debug)]
pub struct TestScene<const WIDTH: usize, const HEIGHT: usize> {
    data: SceneData<WIDTH, HEIGHT>,
    time: f32,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for TestScene<WIDTH, HEIGHT> {
    fn default() -> Self {
        // Setup pose graph
        let root = PoseGraph::root();
        let dagger_pose = PoseGraph::new("dagger", root.clone());
        let cam_pose = PoseGraph::new("cam", root.clone());
        cam_pose
            .borrow_mut()
            .apply_translation(Float3::new(0.0, -1.0, -3.0));
        dagger_pose
            .borrow_mut()
            .apply_rotation(Quaternion::from_z_angle(f32::to_radians(180.0)));

        // Load meshes
        let dagger_mesh = Arc::new(Mesh::from_obj_file("resources/models/dave.obj").unwrap());

        // Load shaders
        let dagger_texture = Texture::from_bytes_file("resources/textures/daveTex.bytes").unwrap();
        let dagger_shader = Arc::new(TextureShader::new(dagger_texture));

        // Create entities
        let dagger = Entity::new(dagger_pose, dagger_mesh.clone(), dagger_shader.clone());

        // Assemble scene data
        let mut data = SceneData {
            cam_model: CameraModel::perspective(60.0),
            cam_pose,
            ..Default::default()
        };
        data.entities.insert("dagger".to_string(), dagger);

        Self { data, time: 0.0 }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Scene<WIDTH, HEIGHT> for TestScene<WIDTH, HEIGHT> {
    fn update_state(&mut self, time: f32, _input: &mut Input) {
        self.time += time;
        let dagger = self.data.entities.get_mut("dagger").unwrap();
        dagger
            .pose
            .borrow_mut()
            .apply_rotation(Quaternion::from_y_angle(f32::to_radians(0.5)));
    }

    fn render(&mut self, buffer: &mut RenderBuffer<WIDTH, HEIGHT>) {
        rasterize_scene(&mut self.data, buffer);
    }
}
