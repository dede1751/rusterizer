use crate::primitives::Float2;

#[derive(Debug, Clone)]
pub struct Tri<T> {
    pub vertices: [T; 3],
}

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

impl Tri<Float2> {
    pub fn bbox(&self) -> (Float2, Float2) {
        let [a, b, c] = self.vertices;
        let min_x = a.x.min(b.x).min(c.x);
        let min_y = a.y.min(b.y).min(c.y);
        let max_x = a.x.max(b.x).max(c.x);
        let max_y = a.y.max(b.y).max(c.y);
        (Float2::new(min_x, min_y), Float2::new(max_x, max_y))
    }

    pub fn to_barycentric(&self, p: Float2) -> (f32, f32, f32) {
        let [a, b, c] = self.vertices;

        (
            Float2::signed_area(a, b, p),
            Float2::signed_area(b, c, p),
            Float2::signed_area(c, a, p),
        )
    }
}
