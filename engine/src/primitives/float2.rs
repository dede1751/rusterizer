use crate::primitives::VectorOps;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

// Scalar/vector math operations
macro_rules! impl_math_ops {
    ($($trait:ident::$fn:ident),*) => {
        $(
            impl std::ops::$trait for Float2 {
                type Output = Self;
                fn $fn(self, rhs: Self) -> Self::Output {
                    Self::new(
                        std::ops::$trait::$fn(self.x, rhs.x),
                        std::ops::$trait::$fn(self.y, rhs.y),
                    )
                }
            }
            impl std::ops::$trait<f32> for Float2 {
                type Output = Self;
                fn $fn(self, rhs: f32) -> Self::Output {
                    Self::new(
                        std::ops::$trait::$fn(self.x, rhs),
                        std::ops::$trait::$fn(self.y, rhs),
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
            impl std::ops::$trait for Float2 {
                fn $fn(&mut self, rhs: Self) {
                    std::ops::$trait::$fn(&mut self.x, rhs.x);
                    std::ops::$trait::$fn(&mut self.y, rhs.y);
                }
            }
            impl std::ops::$trait<f32> for Float2 {
                fn $fn(&mut self, rhs: f32) {
                    std::ops::$trait::$fn(&mut self.x, rhs);
                    std::ops::$trait::$fn(&mut self.y, rhs);
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
impl std::ops::Index<usize> for Float2 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Float2: {}", i),
        }
    }
}

// Mutable indexing
impl std::ops::IndexMut<usize> for Float2 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds for Float2: {}", i),
        }
    }
}

impl std::fmt::Display for Float2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl VectorOps for Float2 {
    fn zero() -> Self {
        Float2::ZERO
    }

    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Float2 {
    pub const ZERO: Self = Float2::new(0.0, 0.0);
    pub const ONE: Self = Float2::new(1.0, 1.0);
    pub const RIGHT: Self = Float2::new(1.0, 0.0);
    pub const UP: Self = Float2::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Float2 { x, y }
    }

    pub fn floor(self) -> Self {
        Float2::new(self.x.floor(), self.y.floor())
    }

    pub const fn signed_area(a: Float2, b: Float2, c: Float2) -> f32 {
        (c.x - a.x) * (b.y - a.y) + (c.y - a.y) * (a.x - b.x)
    }
}
