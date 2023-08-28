use web_sys::{WebGlRenderingContext, HtmlCanvasElement};
use nalgebra::vector;
use nalgebra::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;

use crate::app::Application;
use crate::render::{Renderer, GlRenderer, mesh::Mesh};
use crate::shader::Shader;
use crate::shader::{SHADER_SIMPLE_FRAG, SHADER_SIMPLE_VERT};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log { ($($t:tt)*) => (log(&format!("[test_app] {}", &format_args!($($t)*)).to_string())) }

pub struct TestApplication {
    render: GlRenderer,
    mesh: Mesh,
    time: f32,
    rot: f32,
    program: Option<Shader>,
}

impl Application for TestApplication {
    fn start(&mut self) -> Result<(), JsValue> {
        self.program = self.render.create_shader(SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG).ok();

        if self.program.is_none() {
            console_log!("Failed to compile shaders!");
            panic!();
        }

        //self.mesh.add_vertex(vector!(0.0, 0.0, 0.0));
        //self.mesh.add_vertex(vector!(-0.7, -0.7, 0.0));
        //self.mesh.add_vertex(vector!(0.7, -0.7, 0.0));
        //self.mesh.add_vertex(vector!(-0.7, 0.7, 0.0));
        //self.mesh.add_vertex(vector!(0.0, 0.0, 0.0));
        //self.mesh.add_vertex(vector!(0.7, 0.7, 0.0));

        self.mesh.add_verticies(
            recursive_subdivide(
                vector!(0.0, 1.0, 0.0),
                vector!(-1.7, -1.7, 0.0),
                vector!(1.7, -1.7, 0.0), 2)
        );

        self.mesh.update_buffers(&self.render);
        Ok(())
    }

    fn update(&mut self, dt: f32) {
        self.time += dt;
        self.rot += dt / 5.0;
    }

    fn render(&self) {
        self.render.clear(vector!(0.1, 0.1, 0.1, 1.0));
        let model = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -5.0));
        let model = model * Matrix4::from_euler_angles(0.0, self.rot, self.rot);
        let projection = Matrix4::new_perspective(self.render.aspect(), 70.0, 0.1, 100.0);
        let mvp = projection * model;
        
        self.program.as_ref().unwrap().set_uniform_matrix4f("mvp", mvp);
        self.program.as_ref().unwrap().set_uniform1f("time", self.time);
        self.render.draw_mesh(&self.mesh);
    }

    fn exit(&self) {
        
    }

    fn get_renderer(&self) -> &dyn Renderer {
        &self.render
    }
}

impl TestApplication {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let render = GlRenderer::create(canvas)?;
        Ok(TestApplication{ render, mesh: Mesh::new(), rot: 0.0, program: None, time: 0.0 })
    }
}

fn recursive_subdivide(a: Vector3<f32>, b: Vector3<f32>, c: Vector3<f32>, iterations: i32) -> Vec<Vector3<f32>> {
    let mut divisions = subdivide_trig(a, b, c);

    if iterations > 2 {
        let mut divOut: Vec<Vector3<f32>> = Vec::new();
        for i in 0..4 {
            divOut.append(&mut recursive_subdivide(divisions[i*3], divisions[(i*3)+1], divisions[(i*3)+2], iterations - 1));
        }
        divisions = divOut;
    }

    divisions
}

fn subdivide_trig(a: Vector3<f32>, b: Vector3<f32>, c: Vector3<f32>) -> Vec<Vector3<f32>> {
    let ab = ((b - a) * 0.5) + a;
    let bc = ((c - b) * 0.5) + b;
    let ca = ((c - a) * 0.5) + a;
    vec!(a, ab, ca, b, bc, ab, c, ca, bc, ab, bc, ca)
}
