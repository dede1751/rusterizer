use crate::primitives::{Float2, Float3, Tri};

// Abstract Right-Handed camera model (fully sync)
// FOV is in radians for perspective cameras, scale for orthographic cameras
#[derive(Debug, Clone, Copy, Default)]
pub struct CameraModel<const WIDTH: usize, const HEIGHT: usize> {
    screen_height: f32,
    pub perspective: bool,
}

impl<const WIDTH: usize, const HEIGHT: usize> CameraModel<WIDTH, HEIGHT> {
    pub fn new(fov_deg: f32, perspective: bool) -> Self {
        let fov_rad = f32::to_radians(fov_deg);
        Self {
            screen_height: 2.0 * f32::tan(fov_rad / 2.0),
            perspective,
        }
    }

    pub fn point_to_screen(&self, p: Float3) -> Float2 {
        let center = Float2::new(WIDTH as f32, HEIGHT as f32) / 2.0;
        let mut pixels_per_world_unit = HEIGHT as f32 / self.screen_height;
        if self.perspective {
            pixels_per_world_unit /= -p.z; // Right-Handed camera
        }

        center + Float2::new(p.x, -p.y) * pixels_per_world_unit
    }

    pub fn tri_to_screen(&self, tri: &Tri<Float3>) -> Tri<Float2> {
        Tri::new(
            self.point_to_screen(tri.vertices[0]),
            self.point_to_screen(tri.vertices[1]),
            self.point_to_screen(tri.vertices[2]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pose_graph::{PoseGraph, SharedPGNode};

    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    fn setup_pg() -> (SharedPGNode, SharedPGNode) {
        let root = PoseGraph::root();
        let cam = PoseGraph::new("cam", root.clone());
        let mesh = PoseGraph::new("mesh", root.clone());

        cam.borrow_mut()
            .apply_translation(Float3::new(0.0, 0.0, 1.0));

        mesh.borrow_mut()
            .apply_translation(Float3::new(0.2, 0.0, 0.0));

        (cam, mesh)
    }

    #[test]
    fn perspective_cam_project_center() {
        let cam = CameraModel::<WIDTH, HEIGHT>::new(45.0, true);
        let cam_pose = PoseGraph::root();
        let world_to_cam = cam_pose.borrow().transform;

        let p = Float3::new(0.0, 0.0, -1.0); // RH: negative z is forward
        let screen = cam.point_to_screen(world_to_cam.apply(p));
        assert_eq!(screen, Float2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0));
    }

    #[test]
    fn orthographic_cam_project_center() {
        let cam = CameraModel::<WIDTH, HEIGHT>::new(45.0, false);
        let cam_pose = PoseGraph::root();
        let world_to_cam = cam_pose.borrow().transform;

        let p = Float3::new(0.0, 0.0, 0.0);
        let screen = cam.point_to_screen(world_to_cam.apply(p));
        assert_eq!(screen, Float2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0));
    }

    #[test]
    fn perspective_cam_project_offset() {
        let cam = CameraModel::<WIDTH, HEIGHT>::new(45.0, true);
        let (cam_pose, mesh) = setup_pg();
        let mesh_to_cam = PoseGraph::relative_transform(&mesh, &cam_pose);

        let p = Float3::new(1.0, 1.0, -1.0);
        let screen = cam.point_to_screen(mesh_to_cam.apply(p));

        assert!(screen.x > WIDTH as f32 / 2.0);
        assert!(screen.y < HEIGHT as f32 / 2.0);
    }

    #[test]
    fn orthographic_cam_project_offset() {
        let (cam_pose, mesh) = setup_pg();
        let cam = CameraModel::<WIDTH, HEIGHT>::new(45.0, false);
        let mesh_to_cam = PoseGraph::relative_transform(&mesh, &cam_pose);

        let p = Float3::new(1.0, 1.0, 0.0);
        let screen = cam.point_to_screen(mesh_to_cam.apply(p));

        assert!(screen.x > WIDTH as f32 / 2.0);
        assert!(screen.y < HEIGHT as f32 / 2.0);
    }
}