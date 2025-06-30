use crate::primitives::{Float3, VectorOps};

// Rotation quaternions, assumed to always be of unit length.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    w: f32,
    i: f32,
    j: f32,
    k: f32,
}

// Quaternion multiplication (composition of rotations)
impl std::ops::Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
            j: self.w * rhs.j - self.i * rhs.k + self.j * rhs.w + self.k * rhs.i,
            k: self.w * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.w,
        }
    }
}

impl std::ops::MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// Float3 multiplication (rotation of a vector)
impl std::ops::Mul<Float3> for Quaternion {
    type Output = Float3;
    fn mul(self, rhs: Float3) -> Self::Output {
        let p: Quaternion = rhs.into();
        (self * p * self.inverse()).into()
    }
}

// Convert 3D points into quaternions (w = 0)
impl From<Float3> for Quaternion {
    fn from(item: Float3) -> Self {
        Quaternion::new(0.0, item.x, item.y, item.z)
    }
}

// Convert Quaternion back into 3D points (i,j,k)
impl From<Quaternion> for Float3 {
    fn from(item: Quaternion) -> Self {
        Float3::new(item.i, item.j, item.k)
    }
}

impl std::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}i, {}j, {}k)", self.w, self.i, self.j, self.k)
    }
}

impl Quaternion {
    pub const IDENTITY: Self = Quaternion::new(1.0, 0.0, 0.0, 0.0);

    // Convenience method, not exposed to guarantee unit length
    const fn new(w: f32, i: f32, j: f32, k: f32) -> Self {
        Quaternion { w, i, j, k }
    }

    pub fn from_axis_angle(axis: Float3, angle_rad: f32) -> Self {
        let half_angle = angle_rad * 0.5;
        let (s, c) = half_angle.sin_cos();
        let axis = axis.normalized();

        Self::new(c, axis.x * s, axis.y * s, axis.z * s)
    }

    pub fn from_x_angle(angle_rad: f32) -> Self {
        Self::from_axis_angle(Float3::RIGHT, angle_rad)
    }

    pub fn from_y_angle(angle_rad: f32) -> Self {
        Self::from_axis_angle(Float3::UP, angle_rad)
    }

    pub fn from_z_angle(angle_rad: f32) -> Self {
        Self::from_axis_angle(Float3::FORWARD, angle_rad)
    }

    fn pairwise_mul(self, rhs: Self) -> f32 {
        self.w * rhs.w + self.i * rhs.i + self.j * rhs.j + self.k * rhs.k
    }

    fn length(self) -> f32 {
        self.pairwise_mul(self).sqrt()
    }

    // Not normally needed, account for drift
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len != 0.0 {
            Self::new(self.w / len, self.i / len, self.j / len, self.k / len)
        } else {
            Self::IDENTITY
        }
    }

    // With unit-length assumption, conjugate is inverse
    pub fn inverse(self) -> Self {
        Self::new(self.w, -self.i, -self.j, -self.k)
    }

    fn lerp(self, rhs: Self, t: f32) -> Self {
        let w = self.w * (1.0 - t) + rhs.w * t;
        let i = self.i * (1.0 - t) + rhs.i * t;
        let j = self.j * (1.0 - t) + rhs.j * t;
        let k = self.k * (1.0 - t) + rhs.k * t;
        Quaternion::new(w, i, j, k).normalized()
    }

    pub fn slerp(self, rhs: Self, t: f32) -> Self {
        assert!(t == t.clamp(0.0, 1.0));
        let mut cos_theta = self.pairwise_mul(rhs);

        // If dot product is negative, slerp won't take shortest path.
        let mut target = rhs;
        if cos_theta < 0.0 {
            cos_theta = -cos_theta;
            target = Quaternion::new(-rhs.w, -rhs.i, -rhs.j, -rhs.k);
        }

        // If the angle is small, use lerp to avoid numerical issues
        if cos_theta > 0.9995 {
            return self.lerp(target, t);
        }

        let theta = cos_theta.acos();
        let sin_theta = theta.sin();

        let w1 = ((1.0 - t) * theta).sin() / sin_theta;
        let w2 = (t * theta).sin() / sin_theta;

        Quaternion::new(
            self.w * w1 + target.w * w2,
            self.i * w1 + target.i * w2,
            self.j * w1 + target.j * w2,
            self.k * w1 + target.k * w2,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2; // 90 degrees in radians

    #[test]
    fn test_identity() {
        let q = Quaternion::IDENTITY;
        let v = Float3::new(1.0, 2.0, 3.0);
        let rotated = q * v;
        assert!(VectorOps::approx_eq(rotated, v, 1e-5));
    }

    #[test]
    fn test_inverse() {
        let q = Quaternion::from_y_angle(FRAC_PI_2);
        let q_inv = q.inverse();
        let identity = (q * q_inv).normalized();
        assert!(
            VectorOps::approx_eq(identity.into(), Float3::ZERO, 1e-5)
                || (identity.w - 1.0).abs() < 1e-5
        );
    }

    #[test]
    fn test_associativity() {
        let qx = Quaternion::from_x_angle(FRAC_PI_2);
        let qy = Quaternion::from_y_angle(FRAC_PI_2);
        let v = Float3::new(0.0, 1.0, 0.0);

        let r1 = (qy * qx) * v;
        let r2 = qy * (qx * v);
        assert!(VectorOps::approx_eq(r1, r2, 1e-5));
    }

    #[test]
    fn test_composition() {
        let p = Float3::new(2.0, 0.0, 0.0);
        let qz = Quaternion::from_z_angle(FRAC_PI_2);
        let rotated = qz * p;
        assert!(VectorOps::approx_eq(
            rotated,
            Float3::new(0.0, 2.0, 0.0),
            1e-5
        ));

        let qx = Quaternion::from_x_angle(FRAC_PI_2);
        let rotated = qx * rotated;
        assert!(VectorOps::approx_eq(
            rotated,
            Float3::new(0.0, 0.0, 2.0),
            1e-5
        ));

        let qy = Quaternion::from_y_angle(-FRAC_PI_2);
        let rotated = qy * rotated;
        assert!(VectorOps::approx_eq(
            rotated,
            Float3::new(-2.0, 0.0, 0.0),
            1e-5
        ));
    }

    #[test]
    fn test_normalization() {
        let q = Quaternion::new(2.0, 0.0, 0.0, 0.0);
        let normalized = q.normalized();
        assert!((normalized.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_from_and_to_float3() {
        let v = Float3::new(1.0, 2.0, 3.0);
        let q: Quaternion = v.into();
        assert_eq!(q.w, 0.0);
        let v_back: Float3 = q.into();
        assert_eq!(v, v_back);
    }

    #[test]
    fn test_slerp_endpoints() {
        let q1 = Quaternion::from_z_angle(0.0);
        let q2 = Quaternion::from_z_angle(FRAC_PI_2);

        let p = Float3::new(1.0, 0.0, 0.0);

        let r0 = q1.slerp(q2, 0.0) * p;
        let r1 = q1.slerp(q2, 1.0) * p;

        assert!(VectorOps::approx_eq(r0, q1 * p, 1e-5));
        assert!(VectorOps::approx_eq(r1, q2 * p, 1e-5));
    }

    #[test]
    fn test_slerp_halfway() {
        let q1 = Quaternion::from_z_angle(0.0);
        let q2 = Quaternion::from_z_angle(FRAC_PI_2);

        let p = Float3::new(1.0, 0.0, 0.0);
        let rotated = q1.slerp(q2, 0.5) * p;

        // Expected: rotation of 45 degrees around Z
        let sqrt2_over_2 = (2.0f32).sqrt() / 2.0;
        let expected = Float3::new(sqrt2_over_2, sqrt2_over_2, 0.0);

        assert!(VectorOps::approx_eq(rotated, expected, 1e-5));
    }

    #[test]
    fn test_slerp_unit_length() {
        let q1 = Quaternion::from_z_angle(0.0);
        let q2 = Quaternion::from_z_angle(FRAC_PI_2);

        let interp = q1.slerp(q2, 0.3);
        assert!((interp.length() - 1.0).abs() < 1e-5);
    }
}
