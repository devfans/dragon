#[macro_use] mod utils;
mod movement;

use wasm_bindgen::prelude::*;
extern crate wand;
use dragon::*;

use web_sys;

use std::cell::RefCell;
use std::any::Any;
use wasm_bindgen::prelude::*;

pub struct CursorSpan {
    pub name: String,
    text: String,

    x: f64,
    y: f64,
    w: f64,
    h: f64,

    pub width: f32,
    pub height: f32,

    state: wand::core::State,
    fps: wand::FpsCounter, 
    font_cache: RefCell<Option<String>>, // Caching proper font for the string
}

impl CursorSpan {
    pub fn new(
        state: wand::core::State,
        fps: wand::FpsCounter, 
        name: &str,
        text: &str,
        width: f32, height: f32) -> Self {
        Self {
            name: name.to_string(),
            text: text.to_string(),
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,

            width,
            height,
            state,
            fps,
            font_cache: RefCell::new(None),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("white"));
        ctx.stroke_rect(self.x, self.y, self.w, self.h);
    }

}

impl wand::SpanTrait for CursorSpan {

    fn get_name(&self) -> &str {
        &self.name
    }

    fn dispatch_event(&mut self, ev: &mut wand::component::Event) {
    }

    fn dispath(&mut self, data: Box<dyn Any>) {
        if let Ok(text) = data.downcast::<String>() {
            self.text = text.to_string();
            // Clear font cache
            let mut font = self.font_cache.borrow_mut();
            *font = None;
        }
    }

    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        self.draw_outline(ctx);
        let mut font = self.font_cache.borrow_mut();
        if font.is_none() {
            let size = wand::utils::get_font_with_limit(ctx, &self.text, self.w * 0.8, "Arial").min(20).max(10);
            *font = Some(format!("{}px {}", size, "Arail"));
        }
        if !font.is_none() {
            ctx.set_font(font.as_ref().unwrap());
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_fill_style(&JsValue::from_str("green"));
            let _ = ctx.fill_text(&format!("FPS: {}/s", self.fps.borrow().get()), self.x + self.w/2., self.y + self.h/2. - 10.);
            let _ = ctx.fill_text(&self.text, self.x + self.w/2., self.y + self.h/2. + 10.);
        }

    }

    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool) {
        self.x = left;
        self.y = top;
        self.w = self.width as f64 * (right - left);
        self.h = self.height as f64 * (bottom - top);
        // Clear font cache
        let mut font = self.font_cache.borrow_mut();
        *font = None;
        (0., 0., true)
    }

}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("test");
}

pub fn start() {
    let app = wand::core::Application::new_with_canvas_id("canvas");
    app.draw();
}

#[wasm_bindgen]
pub struct Application {
    app: wand::core::Application,
}



#[wasm_bindgen]
impl Application {
    pub fn new() -> Self {
        let mut app = wand::Application::new_with_canvas_id("canvas");
        let state = app.get_state();

        let mut scene = wand::Scene::default(state.clone());
        let section1 = app.new_section("section1", 1., 1., 0.);
        let cursor_span = CursorSpan::new(state.clone(), app.counter.clone(), "cursor", "Cursor:(N/A)", 0.2, 0.2);
        let world_span = wand::WorldSpan::new(state.clone(), app.context.clone(), "world", "World", 1., 1.);
        let w = &world_span.world.state;
        {
            // Attach cube entity
            let entity = world_span.world.state.create_entity();
            let vertices = [
                (-2., -2., 4.,),
                (2., -2., 4.),
                (2., 2., 4.),
                (-2., 2., 4.),
                (-2., -2., 4.,),
                (-2., -2., -4.,),
                (2., -2., -4.),
                (2., 2., -4.),
                (-2., 2., -4.),
                (-2., -2., -4.,),
                // other lines
                (2., -2., 4.),
                (2., -2., -4.),
                (2., 2., 4.),
                (2., 2., -4.),
                (-2., 2., 4.),
                (-2., 2., -4.)
            ]
                .into_iter()
                .map(|v| core::Point3::new(v.0, v.1, v.2)).collect();
            let mesh = core::BasicMesh::new(vertices, vec!(9, 11, 13));
            let mut transform = ecs::TransformComponent::default();
            transform.set_translation_xyz(-5., 0., -16.);
            w.bind_component(entity, mesh);
            w.bind_component(entity, transform);
            let movement_system = movement::MovementSystem::new(w.clone(), app.input.clone());
            w.register_system("movement", movement_system);
        }

        // Add a simple mesh entity
        {
            // Attach cube entity
            let entity = world_span.world.state.create_entity();
            let vertices = [
                (-2., -2., 4.,),  // 0
                (2., -2., 4.),    // 1
                (2., 2., 4.),     // 2
                (-2., 2., 4.),    // 3
                (-2., -2., -4.,), // 0' 4
                (2., -2., -4.),   // 1' 5
                (2., 2., -4.),    // 2' 6
                (-2., 2., -4.),   // 3' 7
            ]
                .into_iter()
                .map(|v| core::Point3::new(v.0, v.1, v.2)).collect();
            let polygons = vec!(
                (0, 2, 1, "silver"),
                (0, 2, 3, "silver"),
                (4, 5, 6, "grey"),
                (4, 7, 6, "grey"),
                (0, 7, 3, "white"),
                (0, 7, 4, "white"),
                (1, 6, 2, "blue"),
                (1, 6, 5, "blue"),
                (2, 6, 7, "red"),
                (2, 3, 7, "red"),
                (0, 5, 1, "orange"),
                (0, 5, 4, "orange"),
            ).into_iter().map(|(a, b, c, d)| (a, b, c, d.to_string())).collect();
            let mesh = core::SimpleMesh::new(vertices, polygons);
            let mut transform = ecs::TransformComponent::default();
            transform.set_translation_xyz(5., 0., -16.);
            w.bind_component(entity, mesh);
            w.bind_component(entity, transform);
            let movement_system = movement::MovementSystem::new(w.clone(), app.input.clone());
            w.register_system("movement", movement_system);
        }

        {
            let mut section = section1.borrow_mut();
            section.register_span(cursor_span);
            section.register_span(world_span);
        }

        scene.add_section(&section1);
        app.register(scene);

        Self {
            app,
        }
    }

    pub fn draw(&self) {
        self.app.draw();
    }

    pub fn tick(&mut self) {
        self.app.tick();
    }


    pub fn on_size_change(&mut self) {
        self.app.on_resize();
    }

    pub fn on_keyup(&mut self, key: &str) {
        self.app.on_keyup(key);
    }
 
    pub fn on_keydown(&mut self, key: &str) {
        self.app.on_keydown(key);
    }
    
    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        self.app.on_mouse_move(x, y);
        {
            let state = self.app.get_state();
            let state = state.borrow_mut();
            let cursor = state.fetch_span("cursor").unwrap();
            let mut cursor = cursor.borrow_mut();
            // log!("Call {}", cursor.get_name());
            cursor.as_mut().dispath(
                Box::new(format!("Cursor: x: {}, y: {}", x, y))
            );
        }
        // self.app.draw();
    }
}
