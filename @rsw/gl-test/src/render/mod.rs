use web_sys::WebGlRenderingContext;
use wasm_bindgen::prelude::*;
use crate::shader::Shader;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log { ($($t:tt)*) => (log(&format!("[render] {}", &format_args!($($t)*)).to_string())) }

pub trait Renderer {
    fn create_shader(&self, vertex: &str, fragment: &str) -> Result<Shader, ()>;
    fn get_gl(&self) -> Option<&WebGlRenderingContext>;
}

pub struct GlRenderer {
    gl: WebGlRenderingContext,
}

impl Renderer for GlRenderer {
    fn create_shader(&self, vertex: &str, fragment: &str) -> Result<Shader, ()> {
        console_log!("Creating shader...");
        let program = Shader::new(&self.gl, vertex, fragment).unwrap();
        self.gl.use_program(Some(&program.program));
        Ok(program)
    }

    fn get_gl(&self) -> Option<&WebGlRenderingContext> {
        Some(&self.gl)
    }
}

impl GlRenderer {
    pub fn new(gl: WebGlRenderingContext) -> GlRenderer {
        GlRenderer { gl }
    }

    pub fn create(canvas: web_sys::HtmlCanvasElement) -> Result<GlRenderer, JsValue> {
        console_log!("Creating GlRenderer for canvas.");
        let gl = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;
        Ok(GlRenderer{gl})
    }
}
