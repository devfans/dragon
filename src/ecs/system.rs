use std::any::Any;

pub trait System {
    fn tick(&mut self) {
    }

    fn dispatch(&mut self, data: Box<dyn Any>) {}
}
