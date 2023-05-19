#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod app;
mod render;
mod shader;

use shader::Shader;
use shader::{SHADER_SIMPLE_FRAG, SHADER_SIMPLE_VERT};
use app::*;
use render::*;
use render::mesh::Mesh;
use nalgebra::vector;

mod test_app;
use test_app::TestApplication;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log { ($($t:tt)*) => (log(&format_args!($($t)*).to_string())) }

#[wasm_bindgen]
pub struct WebClient {
    app: TestApplication,
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebClient, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let app = TestApplication::new(canvas).unwrap();
        Ok(WebClient { app })
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        console_log!("Starting!");
        self.app.start()
    }

    pub fn update(&mut self, dt: f32) {
        self.app.update(dt);
    }

    pub fn render(&self) {
        let render = self.app.get_renderer();
        render.begin_render();

        self.app.render();

        render.end_render();
    }

    pub fn exit(&self) {
        self.app.exit();
    }
}
