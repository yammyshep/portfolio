use web_sys::{WebGlRenderingContext, WebGlBuffer};
use wasm_bindgen::prelude::*;
use nalgebra::{ Vector1, Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4 };
use crate::shader::Shader;
use crate::Mesh;

pub mod mesh;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log { ($($t:tt)*) => (log(&format!("[render] {}", &format_args!($($t)*)).to_string())) }

const POSITION_ATTRIBUTE: u32 = 0;
const NORMAL_ATTRIBUTE: u32 = 1;
const COLOR_ATTRIBUTE: u32 = 2;
const TEXCOORD_ATTRIBUTE: u32 = 3;

pub trait Renderer {
    fn create_shader(&self, vertex: &str, fragment: &str) -> Result<Shader, ()>;
    fn draw_mesh(&self, mesh: &Mesh);
    fn create_buffer(&self) -> Result<WebGlBuffer, ()>;
    fn clear(&self, color: Vector4<f32>);
    fn begin_render(&self);
    fn end_render(&self);
    fn aspect(&self) -> f32;

    #[deprecated]
    fn get_gl(&self) -> Option<&WebGlRenderingContext>;
}

pub struct GlRenderer {
    gl: WebGlRenderingContext,
    canvas: Option<web_sys::HtmlCanvasElement>,
}

impl Renderer for GlRenderer {
    fn create_shader(&self, vertex: &str, fragment: &str) -> Result<Shader, ()> {
        console_log!("Creating shader...");
        let program = Shader::new(&self.gl, vertex, fragment).unwrap();
        self.gl.use_program(Some(&program.program));
        Ok(program)
    }

    fn draw_mesh(&self, mesh: &Mesh) {
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, mesh.get_vertex_buffer());
        self.gl.enable_vertex_attrib_array(POSITION_ATTRIBUTE);
        self.gl.vertex_attrib_pointer_with_i32(POSITION_ATTRIBUTE, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        if (mesh.using_normals()) {
            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, mesh.get_normal_buffer());
            self.gl.enable_vertex_attrib_array(NORMAL_ATTRIBUTE);
            self.gl.vertex_attrib_pointer_with_i32(NORMAL_ATTRIBUTE, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        } else {
            self.gl.disable_vertex_attrib_array(NORMAL_ATTRIBUTE);
        }

        if (mesh.using_colors()) {
            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, mesh.get_colors_buffer());
            self.gl.enable_vertex_attrib_array(COLOR_ATTRIBUTE);
            self.gl.vertex_attrib_pointer_with_i32(COLOR_ATTRIBUTE, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
        } else {
            self.gl.disable_vertex_attrib_array(COLOR_ATTRIBUTE);
        }

        if (mesh.using_texcoords()) {
            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, mesh.get_texcoord_buffer());
            self.gl.enable_vertex_attrib_array(TEXCOORD_ATTRIBUTE);
            self.gl.vertex_attrib_pointer_with_i32(TEXCOORD_ATTRIBUTE, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        } else {
            self.gl.disable_vertex_attrib_array(TEXCOORD_ATTRIBUTE);
        }

        self.gl.draw_arrays(
            mesh.draw_mode,
            0,
            mesh.num_verticies(),
        );
    }

    fn create_buffer(&self) -> Result<WebGlBuffer, ()> {
        self.gl.create_buffer().ok_or(())
    }

    fn clear(&self, color: Vector4<f32>) {
        self.gl.clear_color(color[0], color[1], color[2], color[3]);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    fn begin_render(&self) {
        if self.canvas.is_some() {
            let width: i32 = self.canvas.as_ref().unwrap().width().try_into().unwrap();
            let height: i32 = self.canvas.as_ref().unwrap().height().try_into().unwrap();
            self.gl.viewport(0, 0, width, height);
        }
    }

    fn end_render(&self) {

    }

    fn aspect(&self) -> f32 {
        let width: i32 = self.canvas.as_ref().unwrap().width().try_into().unwrap();
        let height: i32 = self.canvas.as_ref().unwrap().height().try_into().unwrap();
        (width as f32) / (height as f32)
    }

    fn get_gl(&self) -> Option<&WebGlRenderingContext> {
        Some(&self.gl)
    }
}

impl GlRenderer {
    pub fn new(gl: WebGlRenderingContext) -> GlRenderer {
        GlRenderer { gl, canvas: None }
    }

    pub fn create(canvas: web_sys::HtmlCanvasElement) -> Result<GlRenderer, JsValue> {
        console_log!("Creating GlRenderer for canvas.");
        let gl = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;
        Ok(GlRenderer{gl, canvas: Some(canvas)})
    }
}
