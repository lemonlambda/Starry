use starry_ecs::World;
use starry_ecs::resources::Resource;

#[derive(Debug)]
pub struct TestResource {
    x: i32
}

impl Resource for TestResource {}

pub fn test_resource(world: &World) {
    let mut resource = world.get_resource_mut::<TestResource>();
    println!("{}", resource.x);
    resource.x += 10;
}

#[test]
pub fn create_resource() {
    let world = World::new().add_system(test_resource).add_resource(TestResource { x: 100 }).start().single_step().single_step();
}

pub fn list_resources_system(world: &World) {
    world.list_resources();
}

#[test]
pub fn list_resources_test() {
    World::new().add_system(list_resources_system).add_resource(TestResource { x: 100 }).start().single_step();
}