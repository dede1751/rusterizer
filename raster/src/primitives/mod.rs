use std::ops::{Add, Div, Mul, Sub};

pub mod float2;
pub mod float3;
pub mod quat;
pub mod transform;

pub use float2::Float2;
pub use float3::Float3;
pub use quat::Quaternion;
pub use transform::Transform;

// Simple dot-product based vector operations (works for both Float2 and Float3)
pub trait VectorOps:
    Sized
    + Copy
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
{
    fn zero() -> Self;

    fn dot(self, rhs: Self) -> f32;

    fn cross(self, rhs: Self) -> Self;

    fn sqr_magnitude(self) -> f32 {
        self.dot(self)
    }

    fn length(self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    fn normalized(self) -> Self {
        let len = self.length();
        if len == 0.0 { Self::zero() } else { self / len }
    }

    fn lerp(self, rhs: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        self + (rhs - self) * t
    }
}
