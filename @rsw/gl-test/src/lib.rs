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
    
        let gl = self.render.get_gl().unwrap();
        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    
        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let vert_array = js_sys::Float32Array::view(&VERTS);
    
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
    
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        //Mesh test stuff
        self.mesh.add_vertex(vector!(-0.7, -0.7, 0.0));
        self.mesh.add_vertex(vector!(0.7, -0.7, 0.0));
        self.mesh.add_vertex(vector!(0.0, 0.0, 0.0));
        console_log!("{:?}", self.mesh);

        Ok(())
    }

    pub fn update(&mut self, dt: f32) {
        self.rot += dt / 5.0;
    }

    pub fn render(&self) {
        let gl = self.render.get_gl().unwrap();
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.program.as_ref().unwrap().set_uniform1f("rotation", self.rot);
    
        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (VERTS.len() / 3) as i32,
        );
    }

    pub fn exit(&self) {
        
    }
}
