use std::fmt::Debug;

use dyn_clone::{DynClone, clone_trait_object};



pub trait Component: DynClone + Debug {}

clone_trait_object!(Component);
