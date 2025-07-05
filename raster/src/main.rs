use engine::scene::Scene;

use crate::test_scene::TestScene;

mod raster;
mod test_scene;

fn main() {
    let mut scene = TestScene::<640, 360>::default();
    scene.run();
}
