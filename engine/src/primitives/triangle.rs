use crate::primitives::Float2;

#[derive(Debug, Clone)]
pub struct Tri<T> {
    pub vertices: [T; 3],
}

// Triangle vertex-wise operations
macro_rules! impl_tri_math_ops {
    ($($trait:ident::$fn:ident),*) => {
        $(
            impl<U, V> std::ops::$trait<&Tri<U>> for &Tri<V>
            where
                V: std::ops::$trait<U, Output = V> + Copy,
                U: Copy,
            {
                type Output = Tri<V>;
                fn $fn(self, rhs: &Tri<U>) -> Self::Output {
                    Tri {
                        vertices: [
                            std::ops::$trait::$fn(self.vertices[0], rhs.vertices[0]),
                            std::ops::$trait::$fn(self.vertices[1], rhs.vertices[1]),
                            std::ops::$trait::$fn(self.vertices[2], rhs.vertices[2]),
                        ],
                    }
                }
            }
        )*
    };
}

impl_tri_math_ops!(Add::add, Sub::sub, Mul::mul, Div::div);

// Immutable indexing
impl<T> std::ops::Index<usize> for Tri<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.vertices[i]
    }
}

// Mutable indexing
impl<T> std::ops::IndexMut<usize> for Tri<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.vertices[i]
    }
}

impl<T> Tri<T> {
    pub fn new(v0: T, v1: T, v2: T) -> Self {
        Self {
            vertices: [v0, v1, v2],
        }
    }
}

impl<T: std::ops::Add<T, Output = T> + Copy> Tri<T> {
    pub fn sum(&self) -> T {
        self.vertices[0] + self.vertices[1] + self.vertices[2]
    }
}

impl Tri<Float2> {
    pub fn bbox<const WIDTH: usize, const HEIGHT: usize>(&self) -> (usize, usize, usize, usize) {
        let [a, b, c] = self.vertices;
        let min_x = (a.x.min(b.x).min(c.x) as usize).clamp(0, WIDTH - 1);
        let min_y = (a.y.min(b.y).min(c.y) as usize).clamp(0, HEIGHT - 1);
        let max_x = (a.x.max(b.x).max(c.x) as usize).clamp(0, WIDTH - 1);
        let max_y = (a.y.max(b.y).max(c.y) as usize).clamp(0, HEIGHT - 1);

        (min_x, min_y, max_x, max_y)
    }

    // Assumes outward-facing polygons are CCW-wound.
    // Culls triangles that are too small or back faces.
    pub fn should_cull(&self) -> bool {
        let area = Float2::signed_area(self.vertices[0], self.vertices[1], self.vertices[2]);
        area < 1e-6
    }

    // Convert a 2d point to barycentric coordinates within the triangle.
    // Returns None if the point is outside the triangle. Indepented of winding order.
    pub fn to_barycentric(&self, p: Float2) -> Option<Tri<f32>> {
        let [a, b, c] = self.vertices;
        let area_abp = Float2::signed_area(a, b, p);
        let area_bcp = Float2::signed_area(b, c, p);
        let area_cap = Float2::signed_area(c, a, p);

        if (area_abp >= 0.0) != (area_bcp >= 0.0) || (area_bcp >= 0.0) != (area_cap >= 0.0) {
            return None;
        }

        let total = area_abp + area_bcp + area_cap;
        let inv_total = 1.0 / total;
        Some(Tri::new(
            area_bcp * inv_total,
            area_cap * inv_total,
            area_abp * inv_total,
        ))
    }
}
