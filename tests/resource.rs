use starry_ecs::World;
use starry_ecs::resources::Resource;

#[derive(Debug)]
pub struct TestResource {
    x: i32
}
impl Resource for TestResource {}

#[derive(Clone, Debug)]
struct RunCounter {
    runs: usize,
}
impl Resource for RunCounter {}

pub fn test_resource(world: &World) {
    let mut resource = world.get_resource_mut::<TestResource>();
    resource.x += 10;

    let mut run_counter = world.get_resource_mut::<RunCounter>();
    match run_counter.runs {
        0 => assert_eq!(resource.x, 110),
        1 => assert_eq!(resource.x, 120),
        _ => {}
    };
    run_counter.runs += 1;
}

#[test]
pub fn create_resource() {
    let world = World::new().add_system(test_resource).add_resource(TestResource { x: 100 }).add_resource(RunCounter { runs: 0 }).start().single_step().single_step();
}