
use std::slice::{ Iter, IterMut };
use super::component::{ Store, Component, ComponentStorage };
use std::cell::{Ref, RefMut};

#[derive(Clone)]
pub struct Entity {
    pub id: u32,
    pub components: u128,
    pub indices: [u32; 128],
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            components: 0,
            indices: [0;128],
        }
    }

    pub fn reset (&mut self, id: u32) {
        self.id = id;
        self.components = 0;
        self.indices = [0;128];
    }

    #[inline]
    pub fn set_component(&mut self, id: u8, index: u32) {
        self.indices[id as usize] = index;
        self.components |= 1u128 << id;
    }

    #[inline]
    pub fn has_component(&self, id: u8) -> bool {
        self.components & (1u128 << id) > 0
    }
}

pub type Entities<'a> = RefMut<'a, EntityStore>;
pub type EntitiesRef<'a> = Ref<'a, EntityStore>;
pub struct EntityStore {
    blanks: Vec<usize>,
    data: Vec<Entity>,
}

impl EntityStore {
    pub fn new(cap: usize) -> Self {
        Self {
            blanks: Vec::with_capacity(10),
            data: Vec::with_capacity(cap),
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<Entity> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<Entity> {
        self.data.iter_mut()
    }

    #[inline]
    pub fn get(&self, index: u32) -> Option<&Entity> {
        match self.data.get(index as usize) {
            Some(entity) => {
                if entity.id == index {
                    Some(entity)
                } else {
                    None
                }
            },
            None => None,
        }
    }

    #[inline]
    pub fn get_mut(&mut self, index: u32) -> Option<&mut Entity> {
        match self.data.get_mut(index as usize) {
            Some(entity) => {
                if entity.id == index {
                    Some(entity)
                } else {
                    None
                }
            },
            None => None,
        }
    }

    #[inline]
    pub fn bind_component<C: 'static + Component>(
        &mut self, store: &Store, entity: u32, component: C
    ) {
        if let Some(entity) = self.get_mut(entity) {
            if C::dense() {
                store.get_dense_mut::<C>().insert(entity, component);
            } else {
                store.get_mut::<C>().insert(entity, component);
            }
        }
    }
    
    #[inline]
    pub fn create_entity(&mut self) -> u32 {
        // Find if any blank
        if let Some(index) = self.blanks.pop() {
            self.data[index].reset(index as u32);
            index as u32
        } else {
            let index = self.data.len() as u32;
            self.data.push(Entity::new(index));
            index
        }
    }

    #[inline]
    pub fn remove(&mut self, entity: u32) {
        // Set entity as 0 for blank space
        if self.data[entity as usize].id != 0 {
            self.data[entity as usize].id = 0;
            self.blanks.push(entity as usize);
        }
    }

    pub fn reset(&mut self) {
        self.blanks.clear();
        self.data.clear();
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.data.len() - self.blanks.len()
    }

    #[inline]
    pub fn create_first_entity(&mut self) -> u32 {
        if self.data.len() < 1 {
            self.data.push(Entity::new(0));
        }
        0
    }
}



