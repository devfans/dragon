pub mod mesh;
pub mod camera;
pub mod transform;
pub mod sprite;

pub use camera::CameraComponent;
pub use mesh::MeshComponent;
pub use transform::{TransformComponent, Transform2Component};
pub use sprite::SpriteComponent;

use crate::ecs::Component;
use crate::core::Shape;
use crate::core::Widget;

pub type ShapeComponent = Shape;
impl Component for ShapeComponent {}

pub type WidgetComponent = Widget;
impl Component for WidgetComponent {}

