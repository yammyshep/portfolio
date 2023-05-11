#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod util;
mod shader;

use shader::Shader;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct WebClient {}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
       WebClient {}
    }

    pub fn start() -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    
        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;
    
        let vert_str = include_str!("./shader/simple.vert");
        let frag_str = include_str!("./shader/simple.frag");
    
        let program = Shader::new(&context, vert_str, frag_str).expect("Failed to compile shader program");
        context.use_program(Some(&program.program));
    
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    
        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    
        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
    
            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
    
        context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);
    
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
        context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 3) as i32,
        );
        Ok(())
    }

    pub fn update(dt: f32) {

    }

    pub fn render() {

    }

    pub fn exit() {
        
    }
}
