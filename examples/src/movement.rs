use std::rc::Rc;
use dragon::ecs::*;
use dragon::core::*;
use wand::input::Input;
use std::collections::HashSet;

pub struct MovementSystem {
    state: Rc<WorldState>,
    input: Input,
}

impl MovementSystem {
    pub fn new(state: Rc<WorldState>, input: Input) -> Self {
        Self {
            state,
            input,
        }
    }
}

impl System for MovementSystem {
    fn tick(&mut self) {
        let transform_component_id = self.state.get_component_id::<TransformComponent>().unwrap();
        let mesh_component_id = self.state.get_component_id::<MeshComponent>().unwrap();

        // Debugging with random movement
        {
            let mut c_store = self.state.component_store.borrow_mut();
            let meshes: Vec<u32> = c_store.get(&mesh_component_id).unwrap().keys().cloned().collect();
            let transforms = c_store.get_mut(&transform_component_id).unwrap();
            for transform in transforms.iter_mut().filter(|(entity, _)| {
                meshes.contains(entity)
            }).map(|(_entity, trans)| trans.downcast_mut::<TransformComponent>().unwrap()) {
                transform.append_rotation(
                    Vector3::y_axis(),
                    self.input.borrow_mut().axis("ArrowLeft", "ArrowRight") * 0.1
                );
                transform.append_rotation(
                    Vector3::x_axis(),
                    self.input.borrow_mut().axis("ArrowUp", "ArrowDown") * 0.1
                );
                transform.prepend_translation(
                    Vector3::new(0., 0., self.input.borrow_mut().axis("Z", "X") * 0.1)
                );
            }
        }

    }
}

