use component::Component;

pub mod component;

#[derive(Clone)]
pub struct World {
    pub components: Vec<Box<dyn Component>>
}

impl World {
    pub fn new() -> Self {
        Self {
            components: vec![],
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.components.push(Box::new(component));
        self
    }

    pub fn run(&mut self) {
        loop {
            for (i, mut component) in self.components.clone().into_iter().enumerate() {
                component.run(&self);
                self.components[i] = component;
            }
        }
    }
}