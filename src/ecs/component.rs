use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::iter;
use std::collections::hash_map::IterMut;

pub trait Component {
}

pub type ComponentStore = HashMap<u32, HashMap<u32, Box<dyn Any>>>;

pub trait ComponentStorage {
    fn get_component<C: 'static + Component>(&mut self, entity: u32, component: u32) -> Option<&mut C>;
}

impl ComponentStorage for ComponentStore {
    fn get_component<C: 'static + Component>(&mut self, entity: u32, component: u32) -> Option<&mut C> {
        if let Some(entry) = self.get_mut(&component) {
            if let Some(comp) =  entry.get_mut(&entity) {
                return comp.downcast_mut::<C>();
            }
        }
        None
    }
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
    pub fn register_component<C: 'static + Component>(&mut self) -> u32 {
        let type_id = TypeId::of::<C>();
        if let Some(id) = self.store.get(&type_id) {
            return id.clone();
        }
        let id = self.coder;
        self.coder *= 2;
        self.store.insert(type_id, id);
        id
    }

    pub fn get_code<C: 'static + Component>(&self) -> Option<&u32> {
        self.store.get(&TypeId::of::<C>())
    }
}
