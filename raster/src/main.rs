mod raster;
mod test_scene;

use engine::scene::Scene;

use test_scene::TestScene;

fn main() {
    let mut scene = TestScene::<640, 360>::default();
    scene.run();
}
