use crate::primitives::VectorOps;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Scalar/vector math operations
macro_rules! impl_math_ops {
    ($($trait:ident::$fn:ident),*) => {
        $(
            impl std::ops::$trait for Float3 {
                type Output = Self;
                fn $fn(self, other: Self) -> Self::Output {
                    Self::new(
                        std::ops::$trait::$fn(self.x, other.x),
                        std::ops::$trait::$fn(self.y, other.y),
                        std::ops::$trait::$fn(self.z, other.z),
                    )
                }
            }
            impl std::ops::$trait<f32> for Float3 {
                type Output = Self;
                fn $fn(self, other: f32) -> Self::Output {
                    Self::new(
                        std::ops::$trait::$fn(self.x, other),
                        std::ops::$trait::$fn(self.y, other),
                        std::ops::$trait::$fn(self.z, other),
                    )
                }
            }
        )*
    };
}

// Scalar/vector math assignment operations
macro_rules! impl_math_assign_ops {
    ($($trait:ident::$fn:ident),*) => {
        $(
            impl std::ops::$trait for Float3 {
                fn $fn(&mut self, other: Self) {
                    std::ops::$trait::$fn(&mut self.x, other.x);
                    std::ops::$trait::$fn(&mut self.y, other.y);
                    std::ops::$trait::$fn(&mut self.z, other.z);
                }
            }
            impl std::ops::$trait<f32> for Float3 {
                fn $fn(&mut self, other: f32) {
                    std::ops::$trait::$fn(&mut self.x, other);
                    std::ops::$trait::$fn(&mut self.y, other);
                    std::ops::$trait::$fn(&mut self.z, other);
                }
            }
        )*
    };
}

impl_math_ops!(Add::add, Sub::sub, Mul::mul, Div::div);
impl_math_assign_ops!(
    AddAssign::add_assign,
    SubAssign::sub_assign,
    MulAssign::mul_assign,
    DivAssign::div_assign
);

// Immutable indexing
impl std::ops::Index<usize> for Float3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Float3: {}", i),
        }
    }
}

// Mutable indexing
impl std::ops::IndexMut<usize> for Float3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Float3: {}", i),
        }
    }
}

impl std::fmt::Display for Float3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl VectorOps for Float3 {
    fn zero() -> Self {
        Float3::ZERO
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Self) -> Self {
        Float3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Float3 {
    pub const ZERO: Self = Float3::new(0.0, 0.0, 0.0);
    pub const ONE: Self = Float3::new(1.0, 1.0, 1.0);
    pub const RIGHT: Self = Float3::new(1.0, 0.0, 0.0);
    pub const UP: Self = Float3::new(0.0, 1.0, 0.0);
    pub const FORWARD: Self = Float3::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Float3 { x, y, z }
    }

    // Convert from Float3 to minifb RGB format
    // Clamps values to [0.0, 1.0] range and converts to u32
    pub const fn to_minifb_rgb(self) -> u32 {
        let r = (self.x.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.y.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.z.clamp(0.0, 1.0) * 255.0) as u32;

        b | (g << 8) | (r << 16)
    }
}
