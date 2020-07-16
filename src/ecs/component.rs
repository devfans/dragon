use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};
use std::slice::{ Iter, IterMut };
use crate::ecs::Entity;

pub trait Component {
}

pub type ComponentStore = RefCell<ComponentStoreProto>;
pub struct ComponentStoreProto {
    store: HashMap<TypeId, Box<dyn Any>>,
}

pub struct DenseStore<C> {
    blanks: Vec<u32>,
    data: Vec<(u32, C)>,
    id: u128,
}

impl<C: 'static + Component> DenseStore<C> {
    pub fn new(cap: usize, id: u128) -> Self {
        Self {
            blanks: Vec::with_capacity(10),
            data: Vec::with_capacity(cap),
            id
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<(u32, C)> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<(u32, C)> {
        self.data.iter_mut()
    }

    #[inline]
    pub fn get(&self, entity: &Entity) -> Option<&(u32, C)> {
        if entity.components & self.id > 0 {
            self.data.get(entity.indices[self.id.trailing_zeros() as usize] as usize)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut (u32, C)> {
        if entity.components & self.id > 0 {
            self.data.get_mut(entity.indices[self.id.trailing_zeros() as usize] as usize)
        } else {
            None
        }
    }
    
    #[inline]
    pub fn insert(&mut self, entity: u32, component: C) -> u32 {
        // Find if any blank
        if let Some(index) = self.blanks.pop() {
            self.data[index as usize] = (entity, component);
            index
        } else {
            self.data.push((entity, component));
            (self.data.len() - 1) as u32
        }
    }

    #[inline]
    pub fn remove(&mut self, index: u32) {
        // Set entity as 0 for blank space
        self.data[index as usize].0 = 0;
        self.blanks.push(index);
    }
}


impl ComponentStoreProto {
    pub fn new() -> ComponentStore {
        RefCell::new(Self { store: HashMap::new() })
    }

    #[inline]
    pub fn register<C: 'static + Component>(&mut self) {
        self.store.entry(TypeId::of::<C>()).or_insert(Box::new(RefCell::new(HashMap::new() as HashMap<u32, C>)));
    }

    #[inline]
    pub fn register_as_dense<C: 'static + Component>(&mut self, cap: usize, id: u128) {
        self.store.entry(TypeId::of::<C>())
            .or_insert(Box::new(RefCell::new(DenseStore::<C>::new(cap, id))));
    }

    #[inline]
    pub fn get<C: 'static + Component>(&self) -> Ref<HashMap<u32, C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<HashMap<u32, C>>>().unwrap().borrow()
    }

    #[inline]
    pub fn get_dense<C: 'static + Component>(&self) -> Ref<DenseStore<C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<DenseStore<C>>>().unwrap().borrow()
    }

    #[inline]
    pub fn get_mut<C: 'static + Component>(&self) -> RefMut<HashMap<u32, C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<HashMap<u32, C>>>().unwrap().borrow_mut()
    }

    #[inline]
    pub fn get_dense_mut<C: 'static + Component>(&self) -> RefMut<DenseStore<C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<DenseStore<C>>>().unwrap().borrow_mut()
    }

    #[inline]
    pub fn borrow(&self) -> &HashMap<TypeId, Box<dyn Any>> {
        &self.store
    }

    #[inline]
    pub fn reset(&mut self) {
        self.store.clear();
    }
}

pub struct ComponentManager {
    coder: u128,
    store: HashMap<TypeId, u128>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            coder: 1,
            store: HashMap::new(),
        }
    }
    pub fn register_component<C: 'static + Component>(&mut self) -> u128 {
        let type_id = TypeId::of::<C>();
        if let Some(id) = self.store.get(&type_id) {
            return id.clone();
        }
        let id = self.coder;
        self.coder *= 2;
        self.store.insert(type_id, id);
        id
    }

    #[inline]
    pub fn get_code<C: 'static + Component>(&self) -> Option<&u128> {
        self.store.get(&TypeId::of::<C>())
    }

    #[inline]
    pub fn reset(&mut self) {
        self.coder = 1;
        self.store.clear();
    }
}


