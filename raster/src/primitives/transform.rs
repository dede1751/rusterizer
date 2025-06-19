use crate::primitives::{Float3, Quaternion};

#[derive(Debug, Clone)]
pub struct Transform {
    pub parent: Option<Box<Self>>,
    pub position: Float3,
    pub scale: Float3,
    pub rotation: Quaternion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
