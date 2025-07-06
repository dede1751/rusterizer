mod raster;
mod test_scene;

use engine::scene::Scene;

use test_scene::TestScene;

fn main() {
    let mut scene = TestScene::<960, 540>::default();
    scene.run();
}
