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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log { ($($t:tt)*) => (log(&format_args!($($t)*).to_string())) }

const VERTS: [f32; 18] = [
    -0.7, -0.7, 0.0,
    0.7, -0.7, 0.0,
    0.0, 0.0, 0.0,
    -0.7, 0.7, 0.0,
    0.0, 0.0, 0.0,
    0.7, 0.7, 0.0,
];

#[wasm_bindgen]
pub struct WebClient {
    render: GlRenderer,
    mesh: Mesh,
    rot: f32,
    program: Option<Shader>,
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebClient, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    
        let render = GlRenderer::create(canvas)?;
    
        let program = render.create_shader(SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG).unwrap();

        Ok(WebClient { render, rot: 0.0, program: Some(program), mesh: Mesh::new() })
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        console_log!("Starting!");

        self.mesh.add_vertex(vector!(-0.7, -0.7, 0.0));
        self.mesh.add_vertex(vector!(0.7, -0.7, 0.0));
        self.mesh.add_vertex(vector!(0.0, 0.0, 0.0));
        self.mesh.add_vertex(vector!(-0.7, 0.7, 0.0));
        self.mesh.add_vertex(vector!(0.0, 0.0, 0.0));
        self.mesh.add_vertex(vector!(0.7, 0.7, 0.0));

        self.mesh.update_buffers(&self.render);

        Ok(())
    }

    pub fn update(&mut self, dt: f32) {
        self.rot += dt / 5.0;
    }

    pub fn render(&self) {
        let gl = self.render.get_gl().unwrap();
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.render.draw_mesh(&self.mesh);

        self.program.as_ref().unwrap().set_uniform1f("rotation", self.rot);
    }

    pub fn exit(&self) {
        
    }
}
