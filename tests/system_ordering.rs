use starry_ecs::{systems::{DefaultOrdering, SystemOrdering}, World};

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

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CustomOrdering {
    CPreRun = 100,
    CRun = 200,
    CPostRun = 300
}

impl SystemOrdering for CustomOrdering {}

impl Into<i32> for CustomOrdering {
    fn into(self) -> i32 {
        self as i32
    }
}

#[test]
pub fn test_custom_order() {
    World::new().add_system(CustomOrdering::CPreRun, first).add_system(CustomOrdering::CRun, second).single_step().single_step();
}
