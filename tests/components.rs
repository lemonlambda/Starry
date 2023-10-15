use starry_ecs::{component::Component, World};

use std::io::{stdout, Write};

#[derive(Clone)]
struct TestComponent {
    x: i32
}

impl Component for TestComponent {}

fn test_system(world: &World) {
    let test_comp = world.get_components::<TestComponent>()[0].clone();
    println!("{}", test_comp.x);
}

#[test]
fn create_component() {
    let world = World::new().add_component(TestComponent { x: -100 }).add_system(test_system).run();
}
