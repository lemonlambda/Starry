#![deny(rust_2018_idioms)]
#![feature(type_name_of_val)]
#![feature(thread_spawn_unchecked)]
#![deny(missing_docs)]

use component::Component;
use resources::Resource;
use systems::SystemOrdering;

pub mod component;
pub mod resources;
pub mod systems;


use std::any::{TypeId, type_name};
use std::collections::HashMap;
use std::sync::{Arc};

use parking_lot::{RwLock, RwLockReadGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockWriteGuard};
use thiserror::Error;
use rayon::prelude::*;

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

#[derive(Clone)]
pub struct World {
    pub components: Vec<(Arc<RwLock<dyn Component>>, TypeId)>,
    pub systems: HashMap<i32, Vec<SystemType>>,
    pub starting_systems: Vec<SystemType>,
    pub resources: HashMap<TypeId, Arc<RwLock<dyn Resource>>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
    pub fn new() -> Self {
        Self {
            components: vec![],
            systems: HashMap::new(),
            starting_systems: vec![],
            resources: HashMap::new(),
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.components.push((Arc::new(RwLock::new(component)), TypeId::of::<T>()));
        self
    }

    pub fn add_system<S: SystemOrdering + Copy>(&mut self, system_ordering: S, system: SystemType) -> &mut Self {
        self.systems.entry(system_ordering.into()).or_insert(vec![]);
        self.systems.entry(system_ordering.into()).and_modify(|x| x.push(system));
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
        let cloned = match self.resources.get(&name) {
            Some(ok) => ok,
            None => return Err(StarryError::ResourceNotFound(type_name::<T>()))
        };
        Ok(RwLockReadGuard::map(cloned.read(), |r| {
            unsafe { &*(&*r as *const dyn Resource as *const T) }
        }))
    }

    pub fn get_resource<T: Resource + 'static>(&self) -> ResourceReadGuard<'_, T> {
        self.try_get_resource::<T>().unwrap()
    }

    pub fn try_get_resource_mut<T: Resource + 'static>(&self) -> Result<ResourceWriteGuard<'_, T>, StarryError> {
        let name = TypeId::of::<T>();
        let cloned = match self.resources.get(&name) {
            Some(ok) => ok,
            None => return Err(StarryError::ResourceNotFound(type_name::<T>()))
        };
        Ok(RwLockWriteGuard::map(cloned.write(), |r| {
            unsafe { &mut *(&mut *r as *mut dyn Resource as *mut T) }
        }))
    }
    
    pub fn get_resource_mut<T: Resource + 'static>(&self) -> ResourceWriteGuard<'_, T> {
        self.try_get_resource_mut::<T>().unwrap()
    }

    pub fn list_resources(&self) {
        for resource in self.resources.iter() {
            println!("{:#?}", resource);
        }
    }

    pub fn try_get_components<T: Component + 'static>(&self) -> Result<Vec<ComponentReadGuard<'_, T>>, StarryError> {
        let id = TypeId::of::<T>();

        let comps = self
            .components
            .iter()
            .filter(|(_, t)| t == &id)
            .map(|(v, _)| RwLockReadGuard::map(v.read(), |r| {
                unsafe { &*(r as *const dyn Component as *const T) }
            }))
            .collect::<Vec<MappedRwLockReadGuard<'_, T>>>();

        if comps.len() == 0 {
            return Err(StarryError::ComponentNotFound(type_name::<T>()));
        }

        Ok(comps)
    }

    pub fn get_components<T: Component + 'static>(&self) -> Vec<ComponentReadGuard<'_, T>> {
        self.try_get_components().unwrap()
    }
    
    pub fn try_get_components_mut<T: Component + 'static>(&self) -> Result<Vec<ComponentWriteGuard<'_, T>>, StarryError> {
        let id = TypeId::of::<T>();

        let comps = self
            .components
            .iter()
            .filter(|(_, t)| t == &id)
            .map(|(v, _)| RwLockWriteGuard::map(v.write(), |r| {
                unsafe { &mut *(r as *mut dyn Component as *mut T) }
            }))
            .collect::<Vec<MappedRwLockWriteGuard<'_, T>>>();

        if comps.len() == 0 {
            return Err(StarryError::ComponentNotFound(type_name::<T>()));
        }

        Ok(comps)
    }

    pub fn get_components_mut<T: Component + 'static>(&self) -> Vec<ComponentWriteGuard<'_, T>> {
        self.try_get_components_mut().unwrap()
    }

    pub fn single_step(&mut self) -> &mut Self {
        let mut numbers = self.systems.iter().map(|(i, _)| i).collect::<Vec<_>>();
        numbers.sort();
        
        let _ = numbers.iter().map(|system_group| {
            let _ = self.systems.get(system_group).unwrap().par_iter().map(|system| system(&self)).collect::<Vec<_>>();
        }).collect::<Vec<_>>();
        self
    }

    pub fn start(&mut self) -> &mut Self {
        let _ = self.starting_systems.par_iter().map(|system| system(&self)).collect::<Vec<_>>();
        self
    }

    pub fn run(&mut self) -> ! {
        loop {
        let mut numbers = self.systems.iter().map(|(i, _)| i).collect::<Vec<_>>();
        numbers.sort();
        
        let _ = numbers.iter().map(|system_group| {
            let _ = self.systems.get(system_group).unwrap().par_iter().map(|system| system(&self)).collect::<Vec<_>>();
        }).collect::<Vec<_>>();
        }
    }
}