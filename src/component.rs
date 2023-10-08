use crate::World;

pub trait Component {
    fn run(&mut self, world: &World);
}
