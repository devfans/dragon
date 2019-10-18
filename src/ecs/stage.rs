use std::cell::{Ref, RefMut, RefCell};
use std::any::Any;
use std::collections::HashMap;


pub trait Stage {
    fn on_enter(&mut self) {}
    fn tick(&mut self) {}
    fn on_exit(&mut self) {}
    fn dispatch(&mut self, _data: Box<dyn Any>) {}
} 

pub struct DefaultStage {}
impl Stage for DefaultStage {}
impl DefaultStage {
    pub fn new() -> Self { Self {} }
}

pub struct StageStoreProto {
    store: HashMap<String, RefCell<Box<dyn Stage>>>,
}

pub type StageStore = RefCell<StageStoreProto>;

impl StageStoreProto {
    pub fn new() -> StageStore {
        RefCell::new(StageStoreProto { store: HashMap::new() })
    }

    pub fn borrow(&self) -> &HashMap<String, RefCell<Box<dyn Stage>>> {
        &self.store
    }

    pub fn register<S: 'static + Stage>(&mut self, name: &str, stage: S) {
        self.store.insert(name.to_string(), RefCell::new(Box::new(stage)));
    }

    pub fn save_stage(&mut self, name: &str, stage: Box<dyn Stage>) {
        self.store.insert(name.to_string(), RefCell::new(stage));
    }

    pub fn unregister(&mut self, name: &str) -> Option<RefCell<Box<dyn Stage>>> {
        self.store.remove(name)
    }

    pub fn get(&self, name: &str) -> Ref<Box<dyn Stage>> {
        self.store.get(name).unwrap().borrow()
    }

    pub fn get_mut(&self, name: &str) -> RefMut<Box<dyn Stage>> {
        self.store.get(name).unwrap().borrow_mut()
    }
}
