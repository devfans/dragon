use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, BTreeMap};
use std::any::Any;

use crate::ecs::entity::*;
use crate::ecs::component::*;
use crate::ecs::system::*;
use crate::ecs::components::*;

pub type EntityComponentCollection = HashMap<u32, Box<dyn Any>>;

pub struct WorldState {
    entity_store: Rc<RefCell<HashMap<u32, Entity>>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    component_store: Rc<RefCell<HashMap<u32, HashMap<u32, Box<dyn Any>>>>>,
    component_manager: Rc<RefCell<ComponentManager>>,
    system_store: Rc<RefCell<BTreeMap<String, Box<dyn System>>>>,
    active_camera: Rc<RefCell<(u32, u32)>>,
}

impl WorldState {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            entity_store: Rc::new(RefCell::new(HashMap::new())),
            entity_manager: Rc::new(RefCell::new(EntityManager::new())),
            component_store: Rc::new(RefCell::new(HashMap::new())),
            component_manager: Rc::new(RefCell::new(ComponentManager::new())),
            system_store: Rc::new(RefCell::new(BTreeMap::new())),
            active_camera: Rc::new(RefCell::new((0,0))),
        })
    }

    pub fn register_component<C: 'static + Component>(&self) -> u32 {
        let mut manager = self.component_manager.borrow_mut();
        manager.register_component::<C>()
    }

    pub fn register_system<S: 'static + System>(&self, name: &String, system: S) {
        let mut store = self.system_store.borrow_mut();
        if !store.contains_key(name) {
            store.insert(name.to_string(), Box::new(system));
        }
    }

    pub fn tick(&self) {
        for system in self.system_store.borrow_mut().values_mut() {
            system.tick();
        }
    }

    pub fn create_entity(&self) -> u32 {
        let mut manager = self.entity_manager.borrow_mut();
        let entity = manager.create_entity();
        let id = entity.id;
        let mut store = self.entity_store.borrow_mut();
        store.insert(entity.id, entity);
        id
    }

    pub fn bind_components<C: 'static + Component>(&self, entity_id: &u32, mut components: Vec<C>) {
        let mut entity_store = self.entity_store.borrow_mut();
        let mut component_store = self.component_store.borrow_mut();
        if let Some(entity) = entity_store.get_mut(entity_id) {
            for component in components.drain(..) {
                let comp_id = self.register_component::<C>();
                entity.components &= comp_id;
                let entry = component_store.entry(comp_id).or_insert(HashMap::new());
                entry.insert(entity_id.clone(), Box::new(component));
            }
        }
    }
}

#[derive(Clone)]
pub struct World {
    pub state: Rc<WorldState>,
}

impl World {
    pub fn new() -> Self {
        let state = WorldState::new();
        state.register_component::<MeshComponent>();
        state.register_component::<TransformComponent>();
        state.register_component::<CameraComponent>();

        Self {
            state,
        }
    }
}

