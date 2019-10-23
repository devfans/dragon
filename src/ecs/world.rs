use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::collections::{HashMap, BTreeMap};
use std::any::Any;

use crate::ecs::entity::*;
use crate::ecs::component::*;
use crate::ecs::system::*;
use crate::ecs::components::*;
use crate::ecs::stage::*;
use crate::core::Shape;

pub type EntityComponentCollection = HashMap<u32, Box<dyn Any>>;

pub struct WorldState {
    pub entity_store: RefCell<HashMap<u32, Entity>>,
    pub entity_manager: RefCell<EntityManager>,
    pub component_store: ComponentStore,
    pub component_manager: RefCell<ComponentManager>,
    pub system_store: SystemStore,
    pub stage_store: StageStore,
    pub current_stage: RefCell<(String, Box<dyn Stage>)>,
    pub renderer_store: RefCell<BTreeMap<String, Box<dyn System>>>,
    pub active_camera: Cell<u32>,
    pub shape_store: RefCell<Vec<Shape>>,
}

impl WorldState {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            entity_store: RefCell::new(HashMap::new()),
            entity_manager: RefCell::new(EntityManager::new()),
            component_store: ComponentStoreProto::new(),
            component_manager: RefCell::new(ComponentManager::new()),
            system_store: SystemStoreProto::new(),
            stage_store: StageStoreProto::new(),
            current_stage: RefCell::new((String::from("default"), Box::new(DefaultStage::new()))),
            renderer_store: RefCell::new(BTreeMap::new()),
            active_camera: Cell::new(0),
            shape_store: RefCell::new(Vec::new()),
        })
    }

    pub fn enter<S: 'static + Stage>(&self, name: &str, stage: S) {
        // Exit the current stage
        {
            let mut current_stage = self.current_stage.borrow_mut();
            current_stage.1.as_mut().on_exit();
        }

        // Popout the stage and swap with the old one
        let mut store = self.stage_store.borrow_mut();
        let old_stage = self.current_stage.replace((name.to_string(), Box::new(stage)));
        store.save_stage(&old_stage.0, old_stage.1);
        // Enter the new stage
        {
            let mut current_stage = self.current_stage.borrow_mut();
            current_stage.1.as_mut().on_enter();
        }
    }

    pub fn enter_stage(&self, name: &str) {
        // Exit the current stage
        {
            let mut current_stage = self.current_stage.borrow_mut();
            current_stage.1.as_mut().on_exit();
        }

        // Popout the stage and swap with the old one
        let mut store = self.stage_store.borrow_mut();
        if let Some(stage) = store.unregister(name) {
            let old_stage = self.current_stage.replace((name.to_string(), stage.into_inner()));
            store.save_stage(&old_stage.0, old_stage.1);
        }

        // Enter the new stage
        {
            let mut current_stage = self.current_stage.borrow_mut();
            current_stage.1.as_mut().on_enter();
        }
    }

    pub fn register_component<C: 'static + Component>(&self) -> u32 {
        let mut manager = self.component_manager.borrow_mut();
        let mut store = self.component_store.borrow_mut();
        store.register::<C>();
        manager.register_component::<C>()
    }

    pub fn register_renderer<S: 'static + System>(&self, name: &str, system: S) {
        let mut store = self.renderer_store.borrow_mut();
        if !store.contains_key(name) {
            store.insert(name.to_string(), Box::new(system));
        }
    }

    pub fn register_system<S: 'static + System>(&self, name: &str, system: S) {
        let mut store = self.system_store.borrow_mut();
        store.register(name, system);
    }

    pub fn unregister_system(&self, name: &str) {
        let mut store = self.system_store.borrow_mut();
        store.unregister(name);
    }

    pub fn register_stage<S: 'static + Stage>(&self, name: &str, stage: S) {
        let mut store = self.stage_store.borrow_mut();
        store.register(name, stage);
    }

    pub fn unregister_stage(&self, name: &str) {
        let mut store = self.stage_store.borrow_mut();
        store.unregister(name);
    }


    pub fn tick(&self) {
        self.system_store.borrow().tick();
        self.current_stage.borrow_mut().1.as_mut().tick();
    }

    pub fn render_tick(&self) {
        for system in self.renderer_store.borrow_mut().values_mut() {
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
        state.register_component::<ShapeComponent>();
        state.register_component::<WidgetComponent>();

        Self {
            state,
        }
    }

    pub fn attach_default_camera(&self) {
        let entity = self.state.create_entity();
        let mut transform = TransformComponent::default();
        transform.set_translation_xyz(0., 0., 300.);

        let camera = CameraComponent::default();
        self.state.bind_component(entity, camera);
        self.state.bind_component(entity, transform);
        self.state.switch_camera(entity);
    }
}

