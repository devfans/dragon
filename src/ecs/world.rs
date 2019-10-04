use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::any::Any;

use crate::ecs::entity::*;
use crate::ecs::component::*;
use crate::ecs::system::*;
use crate::ecs::components::camera::CameraComponent;

pub struct WorldState {
    entity_store: Rc<RefCell<HashMap<u32, Entity>>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    component_store: Rc<RefCell<HashMap<u32, HashMap<u32, Box<dyn Any>>>>>,
    component_manager: Rc<RefCell<ComponentManager>>,
    system_store: Rc<RefCell<Vec<Box<dyn System>>>>,
    active_camera: Rc<RefCell<(u32, u32)>>,
}

impl WorldState {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            entity_store: Rc::new(RefCell::new(HashMap::new())),
            entity_manager: Rc::new(RefCell::new(EntityManager::new())),
            component_store: Rc::new(RefCell::new(HashMap::new())),
            component_manager: Rc::new(RefCell::new(ComponentManager::new())),
            system_store: Rc::new(RefCell::new(Vec::new())),
            active_camera: Rc::new(RefCell::new((0,0))),
        })
    }
}

#[derive(Clone)]
pub struct World {
    pub state: Rc<WorldState>,
}

impl World {
    pub fn new() -> Self {
        Self {
            state: WorldState::new(),
        }
    }
}
