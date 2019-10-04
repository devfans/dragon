use std::collections::HashMap;
use std::any::TypeId;

pub trait Component {
}

pub struct ComponentManager {
    coder: u32,
    store: HashMap<TypeId, u32>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            coder: 1,
            store: HashMap::new(),
        }
    }
    pub fn register_component<C: 'static + Component>(&mut self) {
        let id = self.coder;
        self.coder *= 2;
        self.store.insert(TypeId::of::<C>(), id);
    }

    pub fn get_code<C: 'static + Component>(&self) -> Option<&u32> {
        self.store.get(&TypeId::of::<C>())
    }
}
