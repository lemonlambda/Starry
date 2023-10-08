use component::Component;

pub mod component;

pub struct World {
    pub components: Vec<Box<dyn Component>>
}