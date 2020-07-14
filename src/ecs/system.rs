use std::cell::{Ref, RefMut, RefCell};
use std::any::Any;
use std::collections::BTreeMap;

pub trait System {
    fn tick(&mut self) {}
    fn dispatch(&mut self, _data: Box<dyn Any>) {}
}

/*
pub trait RendererSystem {
    fn render_tick(&self) {
    }

    fn dispatch(&mut self, data: Box<dyn Any>) {}
}
*/

pub struct SystemStoreProto {
    store: BTreeMap<String, RefCell<Box<dyn System>>>,
}

pub type SystemStore = RefCell<SystemStoreProto>;

impl SystemStoreProto {
    pub fn new() -> SystemStore {
        RefCell::new(SystemStoreProto { store: BTreeMap::new() })
    }

    pub fn borrow(&self) -> &BTreeMap<String, RefCell<Box<dyn System>>> {
        &self.store
    }

    pub fn register<S: 'static + System>(&mut self, name: &str, system: S) {
        // if !self.store.contains_key(name) {
        self.store.insert(name.to_string(), RefCell::new(Box::new(system)));
        // }
    }

    pub fn unregister(&mut self, name: &str) {
        self.store.remove(name);
    }

    pub fn get(&self, name: &str) -> Ref<Box<dyn System>> {
        self.store.get(name).unwrap().borrow()
    }

    pub fn get_mut(&self, name: &str) -> RefMut<Box<dyn System>> {
        self.store.get(name).unwrap().borrow_mut()
    }

    pub fn tick(&self) {
        for system in self.store.values() {
            system.borrow_mut().as_mut().tick()
        }
    }
}
