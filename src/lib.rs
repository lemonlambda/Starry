#![deny(rust_2018_idioms)]
#![feature(type_name_of_val)]

use component::Component;
use resources::Resource;
use thread_manager::ThreadManager;

pub mod component;
pub mod resources;
pub mod systems;
pub mod thread_manager;

use std::mem;
use std::any::{TypeId, Any, type_name};
use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::{RwLock, RwLockReadGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockWriteGuard};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StarryError {
    #[error("Component no found of type: `{0}`")]
    ComponentNotFound(&'static str),
    #[error("Resource no found of type: `{0}`")]
    ResourceNotFound(&'static str)
}

pub type SystemType = fn(world: &World);
// Aliases to make the type signature make more sense
pub type ResourceWriteGuard<'a, T> = MappedRwLockWriteGuard<'a, T>;
pub type ResourceReadGuard<'a, T> = MappedRwLockReadGuard<'a, T>;
pub type ComponentWriteGuard<'a, T> = MappedRwLockWriteGuard<'a, T>;
pub type ComponentReadGuard<'a, T> = MappedRwLockReadGuard<'a, T>;

pub struct World {
    pub components: Vec<(Arc<RwLock<dyn Component>>, TypeId)>,
    pub systems: Vec<SystemType>,
    pub starting_systems: Vec<SystemType>,
    pub resources: HashMap<TypeId, Arc<RwLock<dyn Resource>>>,
    pub thread_manager: ThreadManager
}

impl World {
    pub fn new() -> Self {
        Self {
            components: vec![],
            systems: vec![],
            starting_systems: vec![],
            resources: HashMap::new(),
            thread_manager: ThreadManager::new(),
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.components.push((Arc::new(RwLock::new(component)), TypeId::of::<T>()));
        self
    }

    pub fn add_system(&mut self, system: SystemType) -> &mut Self {
        self.systems.push(system);
        self
    }

    pub fn add_startup_system(&mut self, system: SystemType) -> &mut Self {
        self.starting_systems.push(system);
        self
    }

    pub fn add_resource<T: Resource + 'static>(&mut self, resource: T) -> &mut Self {
        self.resources.entry(TypeId::of::<T>()).or_insert(Arc::new(RwLock::new(resource)));
        self
    }

    pub fn try_get_resource<T: Resource + 'static>(&self) -> Result<ResourceReadGuard<'_, T>, StarryError> {
        let name = TypeId::of::<T>();
        let cloned = self.resources.get(&name).expect(format!("{} Resource doesn't exist", type_name::<T>()).as_str());
        Ok(RwLockReadGuard::map(cloned.read(), |r| {
            unsafe { &*(&**r as *const dyn Resource as *const T) }
        }))
    }

    pub fn try_get_resource_mut<T: Resource + 'static>(&self) -> Result<ResourceWriteGuard<'_, T>, StarryError> {
        let name = TypeId::of::<T>();
        let cloned = self.resources.get(&name).expect(format!("{} Resource doesn't exist", type_name::<T>()).as_str());
        Ok(RwLockWriteGuard::map(cloned.write(), |r| {
            unsafe { &mut *(&mut **r as *mut dyn Resource as *mut T) }
        }))
    }

    pub fn list_resources(&self) {
        for resource in self.resources.iter() {
            println!("{:#?}", resource);
        }
    }

    pub fn get_components_read<T: Component + 'static>(&self) -> Result<Vec<ComponentReadGuard<'_, T>>, StarryError> {
        let name = TypeId::of::<T>();
        let comps = self.components.iter().filter(|(_, t)| t == &name).map(|(v, _)| v.clone()).collect::<Vec<Arc<RwLock<Box<dyn Component>>>>>();
        if comps.len() == 0 {
            return Err(StarryError::ComponentNotFound);
        }

        // TODO: Implement the rest of this
        todo!();
        
        // Ok(concreted)
    }

    pub fn single_step(&mut self) -> &mut Self {
        for system in &self.systems {
            system(&self);
        }
        self
    }

    pub fn start(&mut self) -> &mut Self {
        for system in &self.starting_systems {
            system(&self);
        }
        self
    }

    pub fn run(&mut self) -> ! {
        loop {
            for system in &self.systems {
                system(&self);
            }
        }
    }
}