use component::Component;

pub mod component;
pub mod systems;

use std::any::{type_name, Any};
use std::cell::{RefCell, RefMut};

#[derive(Debug)]
pub enum StarryError {
    ComponentNotFound
}

pub type SystemType = fn(world: &World);

#[derive(Clone)]
pub struct World {
    pub components: Vec<(RefCell<Box<dyn Component>>, &'static str)>,
    pub systems: Vec<SystemType>
}

impl World {
    pub fn new() -> Self {
        Self {
            components: vec![],
            systems: vec![]
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.components.push((RefCell::new(Box::new(component)), type_name::<T>()));
        self
    }

    pub fn add_system(&mut self, system: SystemType) -> &mut Self {
        self.systems.push(system);
        self
    }

    pub fn get_components<T: Component>(&self) -> Result<Vec<RefMut<T>>, StarryError> {
        let name = type_name::<T>();
        let comps = self.components.iter().filter(|(_, t)| t == &name).map(|(v, _)| v.borrow_mut()).collect::<Vec<RefMut<Box<dyn Component>>>>();
        if comps.len() == 0 {
            return Err(StarryError::ComponentNotFound);
        }
        let concreted = comps.into_iter().map(|x| RefMut::map(x, |y| {
            <Box<dyn component::Component> as Into<Box<dyn Any>>>::into(*y).downcast_mut::<T>().unwrap()
        })).collect::<Vec<_>>();
        Ok(concreted)
    }

    pub fn run(&mut self) {
        loop {
        }
    }
}