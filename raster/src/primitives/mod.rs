use std::ops::{Add, Div, Mul, Sub};

pub mod float2;
pub mod float3;
pub mod transform;

pub use float2::Float2;
pub use float3::Float3;
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

    fn dot(self, other: Self) -> f32;

    fn cross(self, other: Self) -> Self;

    fn sqr_magnitude(self) -> f32 {
        self.dot(self)
    }

    fn length(self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 { Self::zero() } else { self / len }
    }

    fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        self + (other - self) * t
    }
}
