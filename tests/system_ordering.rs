use starry_ecs::{systems::DefaultOrdering, World};

pub fn first(_: &World) {
    println!("First");
}

pub fn second(_: &World) {
    println!("Second");
}

#[test]
pub fn test_order() {
    World::new().add_system(DefaultOrdering::PreRun, first).add_system(DefaultOrdering::Run, second).single_step().single_step();
}
