use std::fmt::Debug;

use dyn_clone::{DynClone, clone_trait_object};

/// Marker trait for saying what's a Component
pub trait Component: DynClone + Debug {}

clone_trait_object!(Component);
