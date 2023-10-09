use component::Component;

pub mod component;
pub mod systems;

use std::any::type_name;

pub enum StarryError {
    ComponentNotFound
}

pub type SystemType = fn(world: &mut World) -> &mut World;

#[derive(Clone)]
pub struct World {
    pub components: Vec<(Box<dyn Component>, &'static str)>,
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
        self.components.push((Box::new(component), type_name::<T>()));
        self
    }

    pub fn add_system(&mut self, system: SystemType) -> &mut Self {
        self.systems.push(system);
        self
    }

    pub fn get_components<T: Component>(&mut self) -> Result<Vec<&mut Box<dyn Component>>, StarryError> {
        let name = type_name::<T>();
    }

    pub fn run(&mut self) {
        loop {
        }
    }
}