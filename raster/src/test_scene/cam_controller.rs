use engine::input::{Input, Key, MouseKey};
use engine::pose_graph::SharedPGNode;
use engine::primitives::{Float3, Quaternion, VectorOps};

const SENSITIVITY: f32 = 2.0;
const CAM_MOVE_SPEED: f32 = 15.0;
const WASD_MOVE_SPEED: f32 = 5.0;
const MAX_PITCH: f32 = f32::to_radians(85.0);
const MIN_PITCH: f32 = f32::to_radians(-85.0);

#[derive(Debug, Default)]
pub struct CamController<const WIDTH: usize, const HEIGHT: usize> {
    pose: SharedPGNode,
    pitch_tgt: f32,
    yaw_tgt: f32,
}

impl<const WIDTH: usize, const HEIGHT: usize> CamController<WIDTH, HEIGHT> {
    pub fn new(pose: SharedPGNode) -> Self {
        Self {
            pose,
            pitch_tgt: 0.0,
            yaw_tgt: 0.0,
        }
    }

    pub fn update_camera(&mut self, time_delta: f32, input: &mut Input) {
        // Mouse Look
        if input.is_mouse_held(MouseKey::Left) {
            let mouse_delta = (input.get_mouse_delta() * SENSITIVITY) / WIDTH as f32;
            self.yaw_tgt += mouse_delta.x;
            self.pitch_tgt = (self.pitch_tgt - mouse_delta.y).clamp(MIN_PITCH, MAX_PITCH);
            input.lock_cursor();
        } else if input.is_key_down_this_frame(Key::Q) {
            input.unlock_cursor();
        }

        let cam_rot = self.pose.borrow().transform.rotation;
        let tgt_rot =
            Quaternion::from_y_angle(self.yaw_tgt) * Quaternion::from_x_angle(self.pitch_tgt);
        self.pose.borrow_mut().transform.rotation =
            cam_rot.slerp(tgt_rot, time_delta * CAM_MOVE_SPEED);

        // WASD Movement
        let mut move_delta = Float3::ZERO;
        if input.is_key_held(Key::W) {
            move_delta += Float3::FORWARD;
        }
        if input.is_key_held(Key::S) {
            move_delta -= Float3::FORWARD;
        }
        if input.is_key_held(Key::A) {
            move_delta -= Float3::RIGHT;
        }
        if input.is_key_held(Key::D) {
            move_delta += Float3::RIGHT;
        }

        move_delta = self.pose.borrow().transform.rotation * move_delta.normalized();
        move_delta *= WASD_MOVE_SPEED * time_delta;
        self.pose.borrow_mut().apply_translation(move_delta);
    }
}
