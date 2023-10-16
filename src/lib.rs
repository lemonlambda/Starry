//! # Starry ECS
//!
//! A very crude parallelized entity-component-system API

#![deny(rust_2018_idioms)]
#![feature(type_name_of_val)]
#![feature(thread_spawn_unchecked)]
#![deny(missing_docs)]

use component::Component;
use resources::Resource;
use systems::SystemOrdering;

/// Trait for Components
pub mod component;
/// Trait for resources
pub mod resources;
/// Traits for SystemOrdering and Systems
pub mod systems;


use std::any::{TypeId, type_name};
use std::collections::HashMap;
use std::sync::{Arc};

use parking_lot::{RwLock, RwLockReadGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockWriteGuard};
use thiserror::Error;
use rayon::prelude::*;

/// An error type for the `try_get` functions
#[derive(Error, Debug)]
pub enum StarryError {
    /// Returns when a certain Component is not found in World
    #[error("Component no found of type: `{0}`")]
    ComponentNotFound(&'static str),
    /// Returns when a certain Resource is not found in World
    #[error("Resource no found of type: `{0}`")]
    ResourceNotFound(&'static str)
}

/// A reusable alias to make it easier to change system type signature
pub type SystemType = fn(world: &World);
// Aliases to make the type signature make more sense
/// Type alias to a more confusing type
pub type ResourceWriteGuard<'a, T> = MappedRwLockWriteGuard<'a, T>;
/// Type alias to a more confusing type
pub type ResourceReadGuard<'a, T> = MappedRwLockReadGuard<'a, T>;
/// Type alias to a more confusing type
pub type ComponentWriteGuard<'a, T> = MappedRwLockWriteGuard<'a, T>;
/// Type alias to a more confusing type
pub type ComponentReadGuard<'a, T> = MappedRwLockReadGuard<'a, T>;

/// The main runtime of the ECS api.
///
/// ```no_run
/// use starry_ecs::World;
/// use starry_ecs::resources::Resource;
/// use starry_ecs::component::Component;
/// use starry_ecs::systems::DefaultOrdering;
///
/// #[derive(Clone, Debug)]
/// pub struct TestResource { x: i32 }
/// impl Resource for TestResource {}
/// 
/// #[derive(Clone, Debug)]
/// pub struct TestComponent { x: i32 }
/// impl Component for TestComponent {}
/// 
/// fn test_system(_: &World) {
///     println!("Hello world!");
/// }
///
/// World::new().add_system(DefaultOrdering::Run, test_system).add_component(TestComponent { x: 0 }).add_resource(TestResource { x: 0 }).run();
/// ```
#[derive(Clone)]
pub struct World {
    components: Vec<(Arc<RwLock<dyn Component>>, TypeId)>,
    systems: HashMap<i32, Vec<SystemType>>,
    starting_systems: Vec<SystemType>,
    resources: HashMap<TypeId, Arc<RwLock<dyn Resource>>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
    /// Creates a new world instance
    pub fn new() -> Self {
        Self {
            components: vec![],
            systems: HashMap::new(),
            starting_systems: vec![],
            resources: HashMap::new(),
        }
    }

    /// Adds a component to the world
    ///
    /// ```
    /// use starry_ecs::component::Component;
    /// use starry_ecs::World;
    ///
    /// #[derive(Clone, Debug)]
    /// pub struct TestComponent { x: i32 }
    /// impl Component for TestComponent {}
    /// 
    /// World::new().add_component(TestComponent { x: 0 });
    /// ```
    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.components.push((Arc::new(RwLock::new(component)), TypeId::of::<T>()));
        self
    }

    /// Adds a system with an ordering to the world
    ///
    /// ```
    /// use starry_ecs::systems::DefaultOrdering;
    /// use starry_ecs::World;
    ///
    /// fn example_system(_: &World) {
    ///     println!("Hello, world!");
    /// }
    ///
    /// World::new().add_system(DefaultOrdering::Run, example_system).single_step();
    /// ```
    pub fn add_system<S: SystemOrdering + Copy>(&mut self, system_ordering: S, system: SystemType) -> &mut Self {
        self.systems.entry(system_ordering.into()).or_insert(vec![]);
        self.systems.entry(system_ordering.into()).and_modify(|x| x.push(system));
        self
    }

    /// Adds a staring system
    ///
    /// ```
    /// use starry_ecs::World;
    ///
    /// fn only_ran_once(_: &World) {
    ///     println!("Hello, World!");
    /// }
    ///
    /// World::new().add_startup_system(only_ran_once).start();
    /// ```
    pub fn add_startup_system(&mut self, system: SystemType) -> &mut Self {
        self.starting_systems.push(system);
        self
    }

    /// Adds a resource to the world.
    /// There can only be once instance of each resource.
    /// If an existing resource exists, it will not be replaced.
    ///
    /// ```
    /// use starry_ecs::resources::Resource;
    /// use starry_ecs::World;
    ///
    /// #[derive(Clone, Debug)]
    /// pub struct TestResource { x: i32 }
    /// impl Resource for TestResource {}
    /// 
    /// World::new().add_resource(TestResource { x: 0 });
    /// ```
    pub fn add_resource<T: Resource + 'static>(&mut self, resource: T) -> &mut Self {
        self.resources.entry(TypeId::of::<T>()).or_insert(Arc::new(RwLock::new(resource)));
        self
    }
    
    /// Gets a resource based on a given type `T` and returns a Read guard
    ///
    /// # Errors
    /// Will return a `StarryError::ResourceNotFound` if the resource is not found
    /// # Example
    /// ```rs
    /// use starry_ecs::World;
    /// use starry_ecs::resources::Resource;
    /// use starry_ecs::systems::DefaultOrdering;
    ///
    /// #[derive(Clone, Debug)]
    /// struct TestResource { x: i32 }
    /// impl Resource for TestResource {}
    ///
    /// fn test_system(world: &World) {
    ///     let _resource = world.try_get_resource::<TestResource>().unwrap();
    /// }
    ///
    /// World::new().add_system(DefaultOrdering::Run, test_system).add_resource(TestResource { x: 0 });
    /// ```
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

    /// Same as `try_get_resource` but unwraps the value
    pub fn get_resource<T: Resource + 'static>(&self) -> ResourceReadGuard<'_, T> {
        self.try_get_resource::<T>().unwrap()
    }

    /// Gets a resource based on a given type `T` and returns a Write guard
    ///
    /// # Errors
    /// Will return a `StarryError::ResourceNotFound` if the resource is not found
    /// # Example
    /// ```rs
    /// use starry_ecs::World;
    /// use starry_ecs::resources::Resource;
    /// use starry_ecs::systems::DefaultOrdering;
    ///
    /// #[derive(Clone, Debug)]
    /// struct TestResource { x: i32 }
    /// impl Resource for TestResource {}
    ///
    /// fn test_system(world: &World) {
    ///     let _resource = world.try_get_resource_mut::<TestResource>().unwrap();
    /// }
    ///
    /// World::new().add_system(DefaultOrdering::Run, test_system).add_resource(TestResource { x: 0 });
    /// ```
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

    /// Same as `try_get_resource_mut` but unwraps the value
    pub fn get_resource_mut<T: Resource + 'static>(&self) -> ResourceWriteGuard<'_, T> {
        self.try_get_resource_mut::<T>().unwrap()
    }

    /// Prints out a list of all resources
    pub fn list_resources(&self) {
        for resource in self.resources.iter() {
            println!("{:#?}", resource);
        }
    }

    /// Gets components based on a given type `T` and returns a Read guard
    ///
    /// # Errors
    /// Will return a `StarryError::ComponentNotFound` if components are not found
    /// # Example
    /// ```rs
    /// use starry_ecs::World;
    /// use starry_ecs::component::Component;
    /// use starry_ecs::systems::DefaultOrdering;
    ///
    /// #[derive(Clone, Debug)]
    /// struct TestComponent { x: i32 }
    /// impl Component for TestComponent {}
    ///
    /// fn test_system(world: &World) {
    ///     let _resource = world.try_get_components::<TestResource>().unwrap();
    /// }
    ///
    /// World::new().add_system(DefaultOrdering::Run, test_system).add_component(TestResource { x: 0 }).add_component(TestResource { x: 1 });
    /// ```
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

    /// Same as `try_get_components` but unwraps the value
    pub fn get_components<T: Component + 'static>(&self) -> Vec<ComponentReadGuard<'_, T>> {
        self.try_get_components().unwrap()
    }
    
    /// Gets components based on a given type `T` and returns a Write guard
    ///
    /// # Errors
    /// Will return a `StarryError::ComponentNotFound` if components are not found
    /// # Example
    /// ```rs
    /// use starry_ecs::World;
    /// use starry_ecs::component::Component;
    /// use starry_ecs::systems::DefaultOrdering;
    ///
    /// #[derive(Clone, Debug)]
    /// struct TestComponent { x: i32 }
    /// impl Component for TestComponent {}
    ///
    /// fn test_system(world: &World) {
    ///     let _resource = world.try_get_components_mut::<TestResource>().unwrap();
    /// }
    ///
    /// World::new().add_system(DefaultOrdering::Run, test_system).add_component(TestResource { x: 0 }).add_component(TestResource { x: 1 });
    /// ```
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

    /// Same as `try_get_components_mut` but unwraps the value
    pub fn get_components_mut<T: Component + 'static>(&self) -> Vec<ComponentWriteGuard<'_, T>> {
        self.try_get_components_mut().unwrap()
    }

    /// Runs a single step of the systems
    ///
    /// ```
    /// use starry_ecs::World;
    ///
    /// World::new().single_step();
    /// ```
    pub fn single_step(&mut self) -> &mut Self {
        let mut numbers = self.systems.iter().map(|(i, _)| i).collect::<Vec<_>>();
        numbers.sort();
        
        let _ = numbers.iter().map(|system_group| {
            let _ = self.systems.get(system_group).unwrap().par_iter().map(|system| system(&self)).collect::<Vec<_>>();
        }).collect::<Vec<_>>();
        self
    }

    /// Runs startup systems
    ///
    /// ```
    /// use starry_ecs::World;
    ///
    /// World::new().start();
    /// ```
    pub fn start(&mut self) -> &mut Self {
        let _ = self.starting_systems.par_iter().map(|system| system(&self)).collect::<Vec<_>>();
        self
    }

    /// Runs systems
    ///
    /// ```no_run
    /// use starry_ecs::World;
    ///
    /// World::new().run();
    /// ```
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