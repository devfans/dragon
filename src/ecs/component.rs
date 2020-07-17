use std::collections::{ hash_map, HashMap };
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};
use std::slice;
use crate::ecs::Entity;
use std::iter::Iterator;

pub trait Component {
    fn dense() -> bool { false }
}

pub type ComponentStore = RefCell<ComponentStoreProto>;
pub struct ComponentStoreProto {
    store: HashMap<TypeId, Box<dyn Any>>,
}

pub struct MapStore<C> {
    data: HashMap<u32, C>,
    id: u8,
}

pub struct DenseStore<C> {
    blanks: Vec<u32>,
    data: Vec<(u32, C)>,
    id: u8,
}

pub trait ComponentStorage<'a, C> {
    type Item;
    type ItemMut;
    type Iter: Iterator<Item = Self::Item>;
    type IterMut: Iterator<Item = Self::ItemMut>;

    fn new(cap: usize, id: u128) -> Self;
    fn iter(&'a self) -> Self::Iter;
    fn iter_mut(&'a mut self) -> Self::IterMut;
    fn get(&'a self, entity: &Entity) -> Option<&'a C>;
    fn get_mut(&'a mut self, entity: &Entity) -> Option<&'a mut C>;
    fn insert(&mut self, entity: &mut Entity, component: C);
    fn remove(&mut self, entity: &mut Entity);
    fn reset(&mut self);
}

impl<'a, C: 'static + Component>ComponentStorage<'a, C> for MapStore<C> {
    type Item = (&'a u32, &'a C);
    type ItemMut = (&'a u32, &'a mut C);
    type Iter= hash_map::Iter<'a, u32, C>;
    type IterMut = hash_map::IterMut<'a, u32, C>;

    fn new(cap: usize, id: u128) -> Self {
        Self {
            data: HashMap::with_capacity(cap),
            id: id.trailing_zeros() as u8,
        }
    }

    #[inline]
    fn iter<'b>(&'a self) -> Self::Iter {
        self.data.iter()
    }

    #[inline]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.data.iter_mut()
    }

    #[inline]
    fn get(&'a self, entity: &Entity) -> Option<&'a C> {
        if (entity.components >> self.id) & 1 > 0 {
            self.data.get(&entity.id)
        } else {
            None
        }
    }

    #[inline]
    fn get_mut(&'a mut self, entity: &Entity) -> Option<&'a mut C> {
        if (entity.components >> self.id) & 1 > 0 {
            self.data.get_mut(&entity.id)
        } else {
            None
        }
    }
    
    #[inline]
    fn insert(&mut self, entity: &mut Entity, component: C) {
        self.data.insert(entity.id, component);
        entity.components |= 1u128 << self.id;
    }

    #[inline]
    fn remove(&mut self, entity: &mut Entity) {
        self.data.remove(&entity.id);
        entity.components &= !(1u128 << self.id);
    }

    #[inline]
    fn reset(&mut self) {
        self.data.clear();
    }
}



impl<'a, C: 'static + Component> ComponentStorage<'a, C> for DenseStore<C> {
    type Item = &'a (u32, C);
    type ItemMut = &'a mut (u32, C);
    type Iter = slice::Iter<'a, (u32, C)>;
    type IterMut = slice::IterMut<'a, (u32, C)>;
    fn new(cap: usize, id: u128) -> Self {
        Self {
            blanks: Vec::with_capacity(10),
            data: Vec::with_capacity(cap),
            id: id.trailing_zeros() as u8,
        }
    }

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }

    #[inline]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.data.iter_mut()
    }

    #[inline]
    fn get(&self, entity: &Entity) -> Option<&C> {
        if (entity.components >> self.id) & 1 > 0 {
            self.data.get(entity.indices[self.id.trailing_zeros() as usize] as usize).map(|(_, v)| v)
        } else {
            None
        }
    }

    #[inline]
    fn get_mut(&'a mut self, entity: &Entity) -> Option<&'a mut C> {
        if (entity.components >> self.id) & 1 > 0 {
            self.data.get_mut(entity.indices[self.id.trailing_zeros() as usize] as usize).map(|(_, v)| v)
        } else {
            None
        }
    }
    
    #[inline]
    fn insert(&mut self, entity: &mut Entity, component: C) {
        // Find if any blank
        entity.indices[self.id as usize] = if let Some(index) = self.blanks.pop() {
            self.data[index as usize] = (entity.id, component);
            index
        } else {
            self.data.push((entity.id, component));
            (self.data.len() - 1) as u32
        };
        entity.components |= 1u128 << self.id;
    }

    #[inline]
    fn remove(&mut self, entity: &mut Entity) {
        // Set entity as 0 for blank space
        let index = entity.indices[self.id as usize];
        self.data[index as usize].0 = 0;
        self.blanks.push(index);
        entity.components &= !(1u128 << self.id);
    }

    #[inline]
    fn reset(&mut self) {
        self.data.clear();
        self.blanks.clear();
    }
}


impl ComponentStoreProto {
    pub fn new() -> ComponentStore {
        RefCell::new(Self { store: HashMap::new() })
    }

    #[inline]
    pub fn register<C: 'static + Component>(&mut self, cap: usize, id: u128) {
        if C::dense() {
            self.store.entry(TypeId::of::<C>())
                .or_insert(Box::new(RefCell::new(DenseStore::<C>::new(cap, id))));
        } else {
            self.store.entry(TypeId::of::<C>())
                .or_insert(Box::new(RefCell::new(MapStore::<C>::new(cap, id))));
        }
    }

    #[inline]
    pub fn get<C: 'static + Component>(&self) -> Ref<MapStore<C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<MapStore<C>>>().unwrap().borrow()
    }

    #[inline]
    pub fn get_dense<C: 'static + Component>(&self) -> Ref<DenseStore<C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<DenseStore<C>>>().unwrap().borrow()
    }

    #[inline]
    pub fn get_mut<C: 'static + Component>(&self) -> RefMut<MapStore<C>> {
        self.store.get(&TypeId::of::<C>()).unwrap().downcast_ref::<RefCell<MapStore<C>>>().unwrap().borrow_mut()
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
    indices: [(TypeId, bool); 128]
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            coder: 1,
            store: HashMap::new(),
            indices: [(TypeId::of::<bool>(), false); 128],
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
        self.indices[id.trailing_zeros() as usize] = (type_id, C::dense());
        id
    }

    #[inline]
    pub fn get_code<C: 'static + Component>(&self) -> Option<&u128> {
        self.store.get(&TypeId::of::<C>())
    }

    #[inline]
    pub fn get_component(&self, id: u128) -> (TypeId, bool) {
        self.indices[id as usize]
    }

    #[inline]
    pub fn reset(&mut self) {
        self.coder = 1;
        self.store.clear();
        self.indices = [(TypeId::of::<bool>(), false);128];
    }
}


