use std::cell::RefCell;
use std::rc::Rc;

use crate::primitives::{Float3, Quaternion, Transform};

// We need this for more complex hierarchies of transforms.
pub type SharedPGNode = Rc<RefCell<PoseGraph>>;
#[derive(Debug, Clone, Default)]
pub struct PoseGraph {
    parent: Option<SharedPGNode>,
    pub transform: Transform,
    pub name: String,
}

impl PoseGraph {
    pub fn root() -> SharedPGNode {
        let root = PoseGraph {
            name: "root".to_string(),
            ..Default::default()
        };
        Rc::new(RefCell::new(root))
    }

    pub fn new(name: &str, parent: SharedPGNode) -> SharedPGNode {
        let mut node = PoseGraph {
            name: name.to_string(),
            ..Default::default()
        };
        node.set_parent(parent);
        Rc::new(RefCell::new(node))
    }

    // Sets the parent. There is no explicit cycle avoidance here!
    pub fn set_parent(&mut self, parent: SharedPGNode) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    pub fn apply_scale(&mut self, scale: Float3) -> &mut Self {
        self.transform.scale *= scale;
        self
    }

    pub fn apply_rotation(&mut self, rotation: Quaternion) -> &mut Self {
        self.transform.rotation *= rotation;
        self
    }

    pub fn apply_translation(&mut self, translation: Float3) -> &mut Self {
        self.transform.position += translation;
        self
    }

    /// Compute world transform by traversing up the tree
    fn to_world(&self) -> Transform {
        let mut current = self.parent.clone();
        let mut acc = self.transform;
        let mut stack = vec![];

        while let Some(node) = current {
            let borrowed = node.borrow(); // Explicitly clone node contents
            stack.push(borrowed.clone());
            current = borrowed.parent.clone();
        }

        while let Some(node) = stack.pop() {
            acc = node.transform.compose(&acc);
        }

        acc
    }

    /// Returns a transform that maps a point in `from`'s local space to `to`'s local space
    pub fn relative_transform(from: &SharedPGNode, to: &SharedPGNode) -> Transform {
        let from_world = from.borrow().to_world();
        let to_world_inv = to.borrow().to_world().inverse();
        to_world_inv.compose(&from_world)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::VectorOps;

    use std::f32::consts::FRAC_PI_2; // 90 degrees in radians

    #[test]
    fn test_round_trip() {
        let root = PoseGraph::root();

        let t1 = PoseGraph::new("t1", root.clone());
        t1.borrow_mut()
            .apply_scale(Float3::new(1.0, 3.0, 0.5))
            .apply_rotation(Quaternion::from_x_angle(FRAC_PI_2));

        let t2 = PoseGraph::new("t2", t1.clone());
        t2.borrow_mut()
            .apply_translation(Float3::new(-2.0, 0.0, 3.2))
            .apply_rotation(Quaternion::from_z_angle(-FRAC_PI_2));

        let p_local = Float3::new(5.5, 2.0, -1.7);
        let t2_to_world = PoseGraph::relative_transform(&t2, &root);
        let p_world = t2_to_world.apply(p_local);
        let p_local_back = t2_to_world.apply_inv(p_world);

        assert!(VectorOps::approx_eq(p_local, p_local_back, 1e-5))
    }

    #[test]
    fn test_deep_hierarchy() {
        let root = PoseGraph::root();

        // The parent is rotated 90 degrees around the Y axis.
        let parent = PoseGraph::new("parent", root.clone());
        parent
            .borrow_mut()
            .apply_rotation(Quaternion::from_y_angle(FRAC_PI_2));

        // The child is translated by (1, 0, 0) relative to the parent.
        let child = PoseGraph::new("child", parent.clone());
        child
            .borrow_mut()
            .apply_translation(Float3::new(1.0, 0.0, 0.0));

        let p_local = Float3::ZERO;

        // - The child's transform moves the point to (1, 0, 0) in the parent's space.
        // - The parent's transform rotates this point 90 degrees around Y, moving it to (0, 0, -1).
        // - The root is identity, so the final world position is (0, 0, -1).
        let expected_world_pos = Float3::new(0.0, 0.0, -1.0);

        let child_to_world = PoseGraph::relative_transform(&child, &root);
        let actual_world_pos = child_to_world.apply(p_local);

        assert!(
            VectorOps::approx_eq(actual_world_pos, expected_world_pos, 1e-5),
            "Deep hierarchy transform failed. Expected {:?}, but got {:?}",
            expected_world_pos,
            actual_world_pos
        );
    }


    #[test]
    fn test_mesh_to_cam() {
        let root = PoseGraph::root();

        let mesh = PoseGraph::new("mesh", root.clone());
        mesh.borrow_mut()
            .apply_scale(Float3::new(2.0, 2.0, 2.0))
            .apply_translation(Float3::new(1.0, 0.0, 0.0))
            .apply_rotation(Quaternion::from_y_angle(FRAC_PI_2));

        let cam = PoseGraph::new("cam", root.clone());
        cam.borrow_mut()
            .apply_translation(Float3::new(-1.0, 0.0, 0.0));

        let p_mesh = Float3::new(0.1, 0.0, 0.0);
        let mesh_to_cam = PoseGraph::relative_transform(&mesh, &cam);
        let p_cam = mesh_to_cam.apply(p_mesh);

        assert!(VectorOps::approx_eq(
            p_cam,
            Float3::new(2.0, 0.0, -0.2),
            1e-5
        ))
    }
}
