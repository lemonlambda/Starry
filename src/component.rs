use dyn_clone::{DynClone, clone_trait_object};

use crate::World;

pub trait Component: DynClone {}

clone_trait_object!(Component);
