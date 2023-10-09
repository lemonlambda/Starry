use starry::{component::Component, World};

use std::io::{stdout, Write};

#[derive(Clone)]
struct TestComponent {
    x: i32
}

impl Component for TestComponent {
    fn run(&mut self, world: &World) {
        self.x ;
        println!("{}", self.x);
    }
}
#[test]
fn create_component() {
    let world = World::new().add_component(TestComponent { x: -100 }).run();
}
