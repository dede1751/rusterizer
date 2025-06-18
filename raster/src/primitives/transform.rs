use crate::primitives::Float3;

#[derive(Debug, Clone)]
pub struct Transform {
    pub parent: Option<Box<Self>>,
    pub position: Float3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
