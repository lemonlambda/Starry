use dyn_clone::{DynClone, clone_trait_object};

use crate::World;

pub trait Component: DynClone {
    fn run(&mut self, world: &World);
}

clone_trait_object!(Component);
