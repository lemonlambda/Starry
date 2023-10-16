use starry_ecs::World;
use starry_ecs::systems::DefaultOrdering;

use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::thread::sleep;

pub fn system_1(_: &World) {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    println!("Hello From System 1: {:?}", since_the_epoch);
    sleep(Duration::from_secs(1));
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    println!("Hello From System 1: {:?}", since_the_epoch);
}

pub fn system_2(_: &World) {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    println!("Hello From System 2: {:?}", since_the_epoch);
    sleep(Duration::from_secs(3));
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    println!("Hello From System 2: {:?}", since_the_epoch);
}

#[test]
pub fn test_parallization() {
    let _world = World::new().add_system(DefaultOrdering::Run, system_1).add_system(DefaultOrdering::Run, system_2).single_step();
}