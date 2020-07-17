use std::cell::{Cell, RefCell };
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
    pub entity_store: RefCell<EntityStore>,
    pub component_store: ComponentStore,
    component_manager: RefCell<ComponentManager>,
    system_store: SystemStore,
    stage_store: StageStore,
    current_stage: RefCell<(String, Box<dyn Stage>)>,
    renderer_store: RefCell<BTreeMap<String, Box<dyn System>>>,
    active_camera: Cell<u32>,
    shape_store: RefCell<Vec<Shape>>,
    stage_event_callbacks: RefCell<BTreeMap<String, Box<dyn Fn(&str, &str)>>>,
}

impl WorldState {
    pub fn new(cap: usize) -> Rc<Self> {
        Rc::new(Self {
            entity_store: RefCell::new(EntityStore::new(cap)),
            component_store: ComponentStoreProto::new(),
            component_manager: RefCell::new(ComponentManager::new()),
            system_store: SystemStoreProto::new(),
            stage_store: StageStoreProto::new(),
            current_stage: RefCell::new((String::from("default"), Box::new(DefaultStage::new()))),
            renderer_store: RefCell::new(BTreeMap::new()),
            active_camera: Cell::new(0),
            shape_store: RefCell::new(Vec::new()),
            stage_event_callbacks: RefCell::new(BTreeMap::new()),
        })
    }

    pub fn current_stage(&self) -> String {
        self.current_stage.borrow().0.clone()
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
        self.stage_event_callback(old_stage.0.clone(), name.to_string());
    }

    pub fn register_stage_event_callback<CB>(&self, name: String, callback: CB) where CB: 'static + Fn(&str, &str) {
        let mut cbs = self.stage_event_callbacks.borrow_mut();
        cbs.insert(name, Box::new(callback));
    }

    pub fn unregister_stage_event_callback(&self, name: &str) {
        self.stage_event_callbacks.borrow_mut().remove(name);
    }

    fn stage_event_callback(&self, old: String, new: String) {
        for callback in self.stage_event_callbacks.borrow().values() {
            callback(&old, &new);
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

        let old_stage = if let Some(stage) = store.unregister(name) {
            let old_stage = self.current_stage.replace((name.to_string(), stage.into_inner()));
            store.save_stage(&old_stage.0, old_stage.1);
            old_stage.0.clone()
        } else { String::from("None") };

        // Enter the new stage
        {
            let mut current_stage = self.current_stage.borrow_mut();
            current_stage.1.as_mut().on_enter();
        }
        self.stage_event_callback(old_stage, name.to_string());
    }

    pub fn register_component<C: 'static + Component>(&self, cap: usize) {
        let mut manager = self.component_manager.borrow_mut();
        let mut store = self.component_store.borrow_mut();
        let id = manager.register_component::<C>();
        store.register::<C>(cap, id);
    }

    pub fn register_renderer<S: 'static + System>(&self, name: &str, system: S) {
        let mut store = self.renderer_store.borrow_mut();
        if !store.contains_key(name) {
            store.insert(name.to_string(), Box::new(system));
        }
    }

    pub fn unregister_renderer(&self, name: &str) {
        let mut store = self.renderer_store.borrow_mut();
        store.remove(name);
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
        let event = self.current_stage.borrow_mut().1.as_mut().tick();
        self.system_store.borrow().tick();
        match event {
            StageEvent::Null => {},
            StageEvent::EnterStage { name } => {
                self.enter_stage(&name);
            }
        };
    }

    pub fn render_tick(&self) {
        for system in self.renderer_store.borrow_mut().values_mut() {
            system.tick();
        }
    }

    pub fn remove_entity(&self, entity_id: u32) {
        let mut entity_store = self.entity_store.borrow_mut();
        /*
        let state = self.component_manager.borrow();
        let store = self.component_store.borrow_mut();
        let entity = entity_store.get_mut(entity_id).unwrap();
        let mut components = entity.components;
        while components > 0 {
            let id = 1u128 << components.trailing_zeros();
            components &= !id;
            let (c, dense) = state.get_component(id);
            if dense {
                store.get_dense_mut().remove(entity);
            } else {
                store.get_mut().remove(entity);
            }
        }*/
        entity_store.remove(entity_id);
    }

    #[inline]
    pub fn create_entity(&self) -> u32 {
        self.entity_store.borrow_mut().create_entity()
    }

    // Clear entities and components
    pub fn clear(&self) {
        self.entity_store.borrow_mut().reset();
        self.component_manager.borrow_mut().reset();
        self.component_store.borrow_mut().reset();

        self.create_global_entity();
    }

    #[inline]
    pub fn current_entities(&self) -> usize {
        self.entity_store.borrow().count()
    }

    fn create_global_entity(&self) -> u32 {
        self.entity_store.borrow_mut().create_first_entity()
    }

    pub fn remove_component<C: 'static + Component>(&self, entity_id: u32) {
        let mut entity_store = self.entity_store.borrow_mut();
        let component_store = self.component_store.borrow();
        if let Some(entity) = entity_store.get_mut(entity_id) {
            if C::dense() {
                component_store.get_dense_mut::<C>().remove(entity);
            } else {
                component_store.get_mut::<C>().remove(entity);
            }
        }
    }
   
    pub fn bind_component<C: 'static + Component>(&self, entity_id: u32, component: C) {
        let mut entity_store = self.entity_store.borrow_mut();
        let component_store = self.component_store.borrow();
        if let Some(entity) = entity_store.get_mut(entity_id) {
            if C::dense() {
                component_store.get_dense_mut::<C>().insert(entity, component);
            } else {
                component_store.get_mut::<C>().insert(entity, component);
            }
        }
    }

    #[inline]
    pub fn get_component_id<C: 'static + Component>(&self) -> Option<u128> {
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
    pub fn new(cap: usize) -> Self {
        let state = WorldState::new(cap);
        state.register_component::<MeshComponent>(10);
        state.register_component::<TransformComponent>(10);
        state.register_component::<CameraComponent>(10);
        state.register_component::<ShapeComponent>(10);
        state.register_component::<WidgetComponent>(10);
        state.create_global_entity();

        Self {
            state,
        }
    }

    pub fn new_basic(cap: usize) -> Self {
        let state = WorldState::new(cap);
        state.create_global_entity();
        Self { state }
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

