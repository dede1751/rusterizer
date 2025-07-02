use engine::scene::Scene;

mod test_scene;

fn main() {
    Scene::<640, 360>::run(&mut test_scene::TestScene::default());
}
