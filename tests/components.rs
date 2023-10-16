use starry_ecs::{component::Component, World, resources::Resource, systems::DefaultOrdering};

#[derive(Clone, Debug)]
struct TestComponent {
    x: i32
}

impl Component for TestComponent {}

#[derive(Clone, Debug)]
struct RunCounter {
    runs: usize,
}
impl Resource for RunCounter {}

fn test_system(world: &World) {
    let test_comp = &world.try_get_components::<TestComponent>().unwrap()[0];

    assert_eq!(test_comp.x, -100);
}

#[test]
fn create_component() {
    let _world = World::new().add_component(TestComponent { x: -100 }).add_system(DefaultOrdering::Run, test_system).start().single_step();
}
