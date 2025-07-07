use crate::primitives::Float3;

pub const ENGINE: CoordinateSystem = CoordinateSystem::RightHandedYUp;
pub const BLENDER: CoordinateSystem = CoordinateSystem::RightHandedZUp;

// This engine defaults to Right-Handed/Y-Up
#[derive(Debug, Clone, Copy)]
pub enum CoordinateSystem {
    RightHandedYUp,
    RightHandedZUp,
    LeftHandedYUp,
    LeftHandedZUp,
}

impl CoordinateSystem {
    pub const fn to_engine_coords(self, v: Float3) -> Float3 {
        match self {
            CoordinateSystem::RightHandedYUp => v,
            CoordinateSystem::RightHandedZUp => Float3::new(v.x, v.z, -v.y),
            CoordinateSystem::LeftHandedYUp => Float3::new(v.x, v.y, -v.z),
            CoordinateSystem::LeftHandedZUp => Float3::new(v.x, v.z, v.y),
        }
    }
}
