use web_sys::WebGlRenderingContext;
use crate::render::Renderer;
use wasm_bindgen::JsValue;

pub trait Application {
    fn start(&mut self) -> Result<(), JsValue>;
    fn update(&mut self, dt: f32);
    fn render(&self);
    fn exit(&self);
    fn get_renderer(&self) -> &dyn Renderer;
}
