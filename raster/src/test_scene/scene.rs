use std::sync::Arc;

use engine::camera::CameraModel;
use engine::coords::ENGINE;
use engine::entity::Entity;
use engine::input::Input;
use engine::mesh::Mesh;
use engine::pose_graph::{PoseGraph, SharedPGNode};
use engine::primitives::{Float3, Quaternion};
use engine::render_buffer::RenderBuffer;
use engine::scene::{Scene, SceneData};
use engine::shader::{DepthShader, LitTextureShader, NormalShader};
use engine::texture::Texture;

use super::cam_controller::CamController;
use crate::raster::rasterize_scene;

#[derive(Debug)]
pub struct TestScene<const WIDTH: usize, const HEIGHT: usize> {
    data: SceneData<WIDTH, HEIGHT>,
    cam_controller: CamController<WIDTH, HEIGHT>,
    sun_pose: SharedPGNode,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for TestScene<WIDTH, HEIGHT> {
    fn default() -> Self {
        // Setup pose graph
        let root = PoseGraph::root();
        let dagger1_pose = PoseGraph::new("dagger1", root.clone());
        let dagger2_pose = PoseGraph::new("dagger1", root.clone());
        let dave_pose = PoseGraph::new("dave", dagger1_pose.clone());
        let cam_pose = PoseGraph::new("cam", root.clone());

        let sun_pose = PoseGraph::new("sun", root.clone());
        let sun_elev = Quaternion::from_x_angle(-f32::to_radians(45.0));
        let sun_azim = Quaternion::from_y_angle(f32::to_radians(30.0));

        sun_pose.borrow_mut().apply_rotation(sun_elev * sun_azim);
        dagger1_pose
            .borrow_mut()
            .apply_translation(Float3::new(2.5, -4.0, -10.0));
        dagger2_pose
            .borrow_mut()
            .apply_translation(Float3::new(-2.5, -4.0, -10.0));
        dave_pose
            .borrow_mut()
            .apply_translation(Float3::new(0.0, 15.0, 0.0));

        // Load meshes
        let dagger_mesh =
            Arc::new(Mesh::from_obj_file("resources/models/dagger.obj", ENGINE).unwrap());
        let dave_mesh = Arc::new(Mesh::from_obj_file("resources/models/dave.obj", ENGINE).unwrap());

        // Load shaders
        let dagger_texture = Texture::from_file("resources/textures/dagger.png").unwrap();
        let dagger1_shader = Arc::new(LitTextureShader::new(dagger_texture));
        let dagger2_shader = Arc::new(NormalShader());
        let dave_shader = Arc::new(DepthShader());

        // Create entities
        let dagger1 = Entity::new(dagger1_pose, dagger_mesh.clone(), dagger1_shader.clone());
        let dagger2 = Entity::new(dagger2_pose, dagger_mesh.clone(), dagger2_shader.clone());
        let dave = Entity::new(dave_pose, dave_mesh.clone(), dave_shader.clone());

        // Assemble scene data
        let mut data = SceneData {
            cam_model: CameraModel::new(60.0, true),
            cam_pose: cam_pose.clone(),
            ..Default::default()
        };
        data.entities.insert("dagger1".to_string(), dagger1);
        data.entities.insert("dagger2".to_string(), dagger2);
        data.entities.insert("dave".to_string(), dave);

        Self {
            data,
            cam_controller: CamController::new(cam_pose),
            sun_pose,
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Scene<WIDTH, HEIGHT> for TestScene<WIDTH, HEIGHT> {
    fn update_state(&mut self, time_delta: f32, input: &mut Input) {
        let dagger1 = self.data.entities.get_mut("dagger1").unwrap();
        dagger1
            .pose
            .borrow_mut()
            .apply_rotation(Quaternion::from_y_angle(f32::to_radians(0.5)));
        self.sun_pose
            .borrow_mut()
            .apply_rotation(Quaternion::from_y_angle(f32::to_radians(-0.5)));

        self.cam_controller.update_camera(time_delta, input);

        let sun_to_cam = PoseGraph::relative_transform(&self.sun_pose, &self.data.cam_pose);
        self.data.globals.time += time_delta;
        self.data.globals.sun_direction_cam_space = sun_to_cam.forward_vec();
    }

    fn render(&mut self, buffer: &mut RenderBuffer<WIDTH, HEIGHT>) {
        rasterize_scene(&mut self.data, buffer);
    }
}
