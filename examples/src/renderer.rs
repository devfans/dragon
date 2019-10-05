use std::rc::Rc;
use dragon::ecs::*;
use std::collections::HashSet;

pub struct RenderingSystem {
    state: Rc<WorldState>,
    ctx: web_sys::CanvasRenderingContext2d,
}

impl RenderingSystem {
    pub fn new(state: Rc<WorldState>, ctx: web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            state,
            ctx,
        }
    }
}

impl System for RenderingSystem {
    fn tick(&mut self) {
        log!("renderer ticking");
        let c_store = self.state.component_store.borrow();
        let active_camera = self.state.active_camera.get();
        let camera_component_id = self.state.get_component_id::<CameraComponent>().unwrap();
        let mesh_component_id = self.state.get_component_id::<MeshComponent>().unwrap();
        let transform_component_id = self.state.get_component_id::<TransformComponent>().unwrap();
        let camera = c_store.get(&camera_component_id).unwrap()
            .get(&active_camera).unwrap()
            .downcast_ref::<CameraComponent>().unwrap();
        let meshes = c_store.get(&mesh_component_id).unwrap();
        let transforms = c_store.get(&transform_component_id).unwrap();


        for (_entity, mesh) in meshes.iter().filter(|entity| transforms.contains_key(entity.0)) {
            let mesh = mesh.downcast_ref::<MeshComponent>().unwrap();
            let mut cutter = mesh.breaks.iter();
            let count = mesh.vertices.len() - 1;
            let mut cut_at = cutter.next().unwrap_or(&count);
            let mut lines = Vec::new();
            let mut line = Vec::new();
            for (index, vertex) in mesh.vertices.iter().enumerate() {
                line.push(camera.project_point(vertex));
                if index == *cut_at {
                    lines.push(line.clone());
                    line.clear();
                    cut_at = cutter.next().unwrap_or(&count);
                }
            }
            for line in lines {
                log!("{:?}", line);
            }
        }

    }
}

