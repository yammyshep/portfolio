use web_sys::{WebGlRenderingContext, HtmlCanvasElement};
use nalgebra::vector;
use nalgebra::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;

use crate::app::Application;
use crate::render::{Renderer, GlRenderer, mesh::Mesh, light::AmbientLight, light::DirectionalLight};
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
    program: Option<Shader>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
    ambient_light: AmbientLight,
    dir_light: DirectionalLight,
}

impl Application for TestApplication {
    fn start(&mut self) -> Result<(), JsValue> {
        self.program = self.render.create_shader(SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG).ok();

        if self.program.is_none() {
            console_log!("Failed to compile shaders!");
            panic!();
        }

        console_log!("Shader compiled!");

        self.view = Matrix4::new_translation(&vector!(0.0, 0.0, -1.0));
        self.projection = Matrix4::new_perspective(self.render.aspect(), 70.0, 0.1, 100.0);

        self.mesh.add_verticies(
            recursive_subdivide(
                vector!(0.0, 0.5, 0.0),
                (Matrix2::new(0.0, -1.0, 1.0, 0.0) * (vector!(0.0, 0.5, 0.0) * 3.0.sqrt()).xy()).push(0.0),
                vector!(0.0, -0.5, 0.0),
                2
            )
        );

        self.mesh.add_verticies(
            recursive_subdivide(
                vector!(0.0, -0.5, 0.0),
                (Matrix2::new(0.0, -1.0, 1.0, 0.0) * (vector!(0.0, -0.5, 0.0) * 3.0.sqrt()).xy()).push(0.0),
                vector!(0.0, 0.5, 0.0),
                2
            )
        );

        for i in 0..self.mesh.len() {
            self.mesh.add_normal(vector!(0.0, 0.0, -1.0));
        }

        self.mesh.use_normals = true;

        self.mesh.update_buffers(&self.render);
        console_log!("Application started.");
        Ok(())
    }

    fn update(&mut self, dt: f32) {
        self.time += dt;
        self.projection = Matrix4::new_perspective(self.render.aspect(), 70.0, 0.1, 100.0);
    }

    fn render(&self) {
        self.render.clear(vector!(0.1, 0.1, 0.1, 1.0));

        // Find the upper right corner of screen
        let vp = self.projection * self.view;
        let vp_inv = vp.try_inverse().unwrap_or(vp.pseudo_inverse(0.000001).unwrap());
        let upper_right = vp_inv * vector!(1.0, 1.0, 0.0, 0.0);

        // Rotate and scale model to fill screen
        let angle: f32 = upper_right.xyz().angle(&vector!(0.0,1.0,0.0));
        let mut model = Matrix4::from_axis_angle(&Unit::new_normalize(vector!(0.0,0.0,-1.0)), angle);
        model = model * Matrix4::new_scaling(upper_right.magnitude() * 2.0);

        // This is probably over-correcting but I do not care :)
        let over_angle = angle - (60.0 * (std::f32::consts::PI / 180.0));
        if (over_angle >= 0.0) {
            model = model * Matrix4::new_scaling(1.0 + over_angle);
        }
        
        let mvp = self.projection * self.view * model;
        
        self.program.as_ref().unwrap().set_uniform_matrix4f("mvp", mvp);
        self.program.as_ref().unwrap().set_uniform_matrix4f("normalMatrix", model.transpose().try_inverse().unwrap_or(Matrix4::identity()));
        //self.program.as_ref().unwrap().set_uniform1f("time", self.time);
        self.program.as_ref().unwrap().set_uniform4f("ambientLightColor", self.ambient_light.color.push(self.ambient_light.intensity));
        self.program.as_ref().unwrap().set_uniform4f("directionalLightColor", self.dir_light.color.push(self.dir_light.intensity));
        self.program.as_ref().unwrap().set_uniform3f("directionalLightDir", self.dir_light.direction);
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
        Ok(TestApplication {
            render,
            mesh: Mesh::new(),
            program: None,
            time: 0.0,
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            ambient_light: AmbientLight{
                color: vector!(1.0, 1.0, 1.0),
                intensity: 0.25,
            },
            dir_light: DirectionalLight{
                color: vector!(1.0, 1.0, 1.0),
                intensity: 1.0,
                direction: vector!(1.0, 0.0, 1.0).normalize(),
            },
        })
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
