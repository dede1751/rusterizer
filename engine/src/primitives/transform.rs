use crate::primitives::{Float3, Quaternion, Tri};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Float3,
    pub scale: Float3,
    pub rotation: Quaternion,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Float3::ZERO,
            scale: Float3::ONE,
            rotation: Quaternion::IDENTITY,
        }
    }
}

impl Transform {
    pub fn inverse(&self) -> Self {
        let inv_rotation = self.rotation.inverse();
        let inv_scale = Float3::ONE / self.scale;
        let inv_position = inv_rotation * (self.position * inv_scale * -1.);
        Self {
            rotation: inv_rotation,
            scale: inv_scale,
            position: inv_position,
        }
    }

    pub fn apply(&self, p: Float3) -> Float3 {
        self.rotation * (p * self.scale) + self.position
    }

    pub fn apply_inv(&self, p: Float3) -> Float3 {
        (self.rotation.inverse() * (p - self.position)) / self.scale
    }

    pub fn apply_tri(&self, t: &Tri<Float3>) -> Tri<Float3> {
        Tri::new(
            self.apply(t.vertices[0]),
            self.apply(t.vertices[1]),
            self.apply(t.vertices[2]),
        )
    }

    pub fn apply_tri_inv(&self, t: &Tri<Float3>) -> Tri<Float3> {
        Tri::new(
            self.apply_inv(t.vertices[0]),
            self.apply_inv(t.vertices[1]),
            self.apply_inv(t.vertices[2]),
        )
    }

    pub fn compose(&self, local: &Transform) -> Transform {
        let scaled_position = local.position * self.scale;
        let rotated_position = self.rotation * scaled_position;

        Transform {
            position: rotated_position + self.position,
            scale: self.scale * local.scale,
            rotation: self.rotation * local.rotation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::VectorOps;

    use std::f32::consts::FRAC_PI_2; // 90 degrees in radians

    #[test]
    fn test_scale_translation() {
        let t = Transform {
            position: Float3::new(1.0, -2.0, 0.0),
            scale: Float3::new(1.0, 3.0, 0.5),
            rotation: Quaternion::IDENTITY,
        };

        let p_local = Float3::new(1.0, 1.0, 1.0);
        let p_world = t.apply(p_local);

        assert!(VectorOps::approx_eq(
            p_world,
            Float3::new(2.0, 1.0, 0.5),
            1e-5
        ));
    }

    #[test]
    fn test_inverse_transform() {
        let t = Transform {
            position: Float3::new(1.0, 2.0, 3.0),
            scale: Float3::new(2.0, 2.0, 2.0),
            rotation: Quaternion::from_z_angle(FRAC_PI_2), // 90° around Z
        };

        let p = Float3::new(1.0, 0.0, 0.0);
        let transformed = t.apply(p);
        let recovered = t.inverse().apply(transformed);

        assert!(VectorOps::approx_eq(p, recovered, 1e-5));
    }

    #[test]
    fn test_compose_transform() {
        let parent = Transform {
            position: Float3::new(0.0, 1.0, 0.0),
            scale: Float3::new(2.0, 2.0, 2.0),
            rotation: Quaternion::from_z_angle(FRAC_PI_2),
        };

        let local = Transform {
            position: Float3::new(1.0, 0.0, 0.0),
            scale: Float3::new(1.0, 1.0, 1.0),
            rotation: Quaternion::IDENTITY,
        };

        let composed = parent.compose(&local);
        let p = Float3::new(0.0, 0.0, 0.0);

        // Should first move (1, 0, 0) by parent's scale, rotate 90 deg to (0, 2, 0), then translate by (0,1,0)
        let expected = Float3::new(0.0, 3.0, 0.0);
        let result = composed.apply(p);

        assert!(VectorOps::approx_eq(result, expected, 1e-5));
    }
}
