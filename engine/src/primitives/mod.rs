pub use face::{FaceData2D, FaceData3D};
pub use float2::Float2;
pub use float3::Float3;
pub use quaternion::Quaternion;
pub use transform::Transform;
pub use triangle::Tri;

mod face;
mod float2;
mod float3;
mod quaternion;
mod transform;
mod triangle;

use std::ops::{Add, Div, Mul, Sub};

// Get Float3 from Float2
impl From<Float2> for Float3 {
    fn from(item: Float2) -> Self {
        Float3::new(item.x, item.y, 0.0)
    }
}

// Get Float2 from Float3
impl From<Float3> for Float2 {
    fn from(item: Float3) -> Self {
        Float2::new(item.x, item.y)
    }
}

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

    fn approx_eq(lhs: Self, rhs: Self, eps: f32) -> bool {
        (lhs - rhs).length() < eps
    }
}
