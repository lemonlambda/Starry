use component::Component;
use resources::Resource;

pub mod component;
pub mod resources;
pub mod systems;

use std::any::{TypeId, Any};
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};

#[derive(Debug)]
pub enum StarryError {
    ComponentNotFound,
    ResourceNotFound
}

pub type SystemType = fn(world: &World);

pub struct World {
    pub components: Vec<(RefCell<Box<dyn Component>>, TypeId)>,
    pub systems: Vec<SystemType>,
    pub resources: HashMap<TypeId, RefCell<Box<dyn Resource>>>
}

impl World {
    pub fn new() -> Self {
        Self {
            components: vec![],
            systems: vec![],
            resources: HashMap::new()
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.components.push((RefCell::new(Box::new(component)), TypeId::of::<T>()));
        self
    }

    pub fn add_system(&mut self, system: SystemType) -> &mut Self {
        self.systems.push(system);
        self
    }

    pub fn add_resource<T: Resource + 'static>(&mut self, resource: T) -> &mut Self {
        self.resources.entry(TypeId::of::<T>()).or_insert(RefCell::new(Box::new(resource)));
        self
    }

    pub fn get_resource<T: Resource + 'static>(&self) -> Result<RefMut<T>, StarryError> {
        let name = TypeId::of::<T>();
        match self.resources.get(&name) {
            Some(resource) => {
                Ok(RefMut::map(resource.borrow_mut(), |thing: &mut Box<dyn Resource>| {
                    let thing: *mut T = &mut **thing as *mut dyn Resource as *mut T;
                    unsafe { &mut *thing }
                }))
            },
            None => Err(StarryError::ResourceNotFound)
        }
    }

    pub fn get_components<T: Component + 'static>(&self) -> Result<Vec<RefMut<T>>, StarryError> {
        let name = TypeId::of::<T>();
        let comps = self.components.iter().filter(|(_, t)| t == &name).map(|(v, _)| v.borrow_mut()).collect::<Vec<RefMut<Box<dyn Component>>>>();
        if comps.len() == 0 {
            return Err(StarryError::ComponentNotFound);
        }
        let concreted = comps.into_iter().map(|x| RefMut::map(x, |thing: &mut Box<dyn Component>| {
            let thing: *mut T = &mut **thing as *mut dyn Component as *mut T;
            unsafe { &mut *thing }
        })).collect::<Vec<_>>();
        Ok(concreted)
    }

    pub fn run(&mut self) {
        loop {
            for system in &self.systems {
                system(&self);
            }
        }
    }
}