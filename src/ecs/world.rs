use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::collections::{HashMap, BTreeMap};
use std::any::Any;

use crate::ecs::entity::*;
use crate::ecs::component::*;
use crate::ecs::system::*;
use crate::ecs::components::*;

pub type EntityComponentCollection = HashMap<u32, Box<dyn Any>>;

pub struct WorldState {
    pub entity_store: RefCell<HashMap<u32, Entity>>,
    pub entity_manager: RefCell<EntityManager>,
    pub component_store: ComponentStore,
    pub component_manager: RefCell<ComponentManager>,
    pub system_store: RefCell<BTreeMap<String, Box<dyn System>>>,
    pub active_camera: Cell<u32>,
}

impl WorldState {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            entity_store: RefCell::new(HashMap::new()),
            entity_manager: RefCell::new(EntityManager::new()),
            component_store: ComponentStoreProto::new(),
            component_manager: RefCell::new(ComponentManager::new()),
            system_store: RefCell::new(BTreeMap::new()),
            active_camera: Cell::new(0),
        })
    }

    pub fn register_component<C: 'static + Component>(&self) -> u32 {
        let mut manager = self.component_manager.borrow_mut();
        let mut store = self.component_store.borrow_mut();
        store.register::<C>();
        manager.register_component::<C>()
    }

    pub fn register_system<S: 'static + System>(&self, name: &str, system: S) {
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

    pub fn bind_component<C: 'static + Component>(&self, entity_id: u32, component: C) -> bool {
        let comp_id = self.register_component::<C>();
        let mut entity_store = self.entity_store.borrow_mut();
        let component_store = self.component_store.borrow();
        if let Some(entity) = entity_store.get_mut(&entity_id) {
            entity.components &= comp_id;
            let mut store = component_store.get_mut::<C>();
            store.insert(entity_id, component);
            return true;
        }
        false
    }

    #[inline]
    pub fn get_component_id<C: 'static + Component>(&self) -> Option<u32> {
        match self.component_manager.borrow().get_code::<C>() {
            Some(id) => Some(id.clone()),
            None => None
        }
    }

    pub fn switch_camera(&self, camera: u32) {
        self.active_camera.set(camera);
    }

    /* FIXME Is it proper to fetch shared projection?
    pub fn get_active_camera(&self) ->  {
        let id = self.active_camera.get();
        let camera = self.component_store.borrow().get::<CameraComponent>().get(id).unwrap().
    }
    */
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

    pub fn attach_default_camera(&self) {
        let entity = self.state.create_entity();
        let transform = TransformComponent::default();
        let camera = CameraComponent::default();
        self.state.bind_component(entity, camera);
        self.state.bind_component(entity, transform);
        self.state.switch_camera(entity);
    }
}

