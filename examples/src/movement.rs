use std::rc::Rc;
use dragon::ecs::*;
use dragon::core::*;
use wand::input::Input;

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
        let c_store = self.state.component_store.borrow();
        let meshes = c_store.get::<MeshComponent>();
        let mut transforms = c_store.get_mut::<TransformComponent>();
        for (_, transform) in transforms.iter_mut().filter(|(entity, _)| {
            meshes.contains_key(entity)
        }) {
            transform.append_rotation(
                Vector3::y_axis(),
                self.input.borrow_mut().axis("ArrowLeft", "ArrowRight") * 0.1
            );
            transform.append_rotation(
                Vector3::x_axis(),
                self.input.borrow_mut().axis("ArrowUp", "ArrowDown") * 0.1
            );
            transform.prepend_translation(
                Vector3::new(0., 0., self.input.borrow_mut().axis("z", "x") * 0.6)
            );
        }

    }
}

