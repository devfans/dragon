use crate::core::transform::{ Transform2, Transform3 };
use crate::ecs::component::Component;

pub type TransformComponent = Transform3;
pub type Transform2Component = Transform2;

impl Component for TransformComponent {}
impl Component for Transform2Component {}



