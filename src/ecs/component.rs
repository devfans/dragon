use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};

pub trait Component {
}

pub type ComponentStore = RefCell<ComponentStoreProto>;
pub struct ComponentStoreProto {
    store: HashMap<TypeId, Box<dyn Any>>
}


impl ComponentStoreProto {
    pub fn new() -> ComponentStore {
        RefCell::new(Self { store: HashMap::new() })
    }

    pub fn register<C: 'static + Component>(&mut self) {
        self.store.entry(TypeId::of::<C>()).or_insert(Box::new(RefCell::new(HashMap::new() as HashMap<u32, C>)));
    }

    pub fn get<C: 'static + Component>(&self) -> Ref<HashMap<u32, C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<HashMap<u32, C>>>().unwrap().borrow()
    }
    pub fn get_mut<C: 'static + Component>(&self) -> RefMut<HashMap<u32, C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<HashMap<u32, C>>>().unwrap().borrow_mut()
    }

    pub fn borrow(&self) -> &HashMap<TypeId, Box<dyn Any>> {
        &self.store
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


