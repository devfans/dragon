use std::rc::Rc;
use crate::ecs::*;

pub struct RenderingSystem {
    state: Rc<WorldState>
}

impl RenderingSystem {
    pub fn new(state: Rc<WorldState>) -> Self {
        Self {
            state
        }
    }
}

impl System for RenderingSystem {
    fn tick(&mut self) {
    }
}

