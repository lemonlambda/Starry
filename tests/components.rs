use starry::{component::Component, World};

use std::io::{stdout, Write};

#[derive(Clone)]
struct TestComponent {
    x: i32
}

#[test]
fn create_component() {
    let world = World::new().add_component(TestComponent { x: -100 }).run();
}
