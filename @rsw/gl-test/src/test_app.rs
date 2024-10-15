use web_sys::{WebGlRenderingContext, HtmlCanvasElement};
use nalgebra::vector;
use nalgebra::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

use crate::app::Application;
use crate::render::{Renderer, GlRenderer, mesh::Mesh, light::AmbientLight, light::DirectionalLight};
use crate::render::shader::Shader;
use crate::render::program::Program;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log { ($($t:tt)*) => (log(&format!("[test_app] {}", &format_args!($($t)*)).to_string())) }

pub struct TestApplication {
    render: GlRenderer,
    mesh: Mesh,
    computed_mesh: Mesh,
    outline_mesh: Mesh,
    time: f32,
    program: Option<Program>,
    outline_program: Option<Program>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
    ambient_light: AmbientLight,
    dir_light: DirectionalLight,
    perlin: Perlin,
}

impl Application for TestApplication {
    fn start(&mut self) -> Result<(), JsValue> {
        self.program = self.render.create_program("simple.vert", "simple.frag").ok();
        self.outline_program = self.render.create_program("simple.vert", "flatcolor.frag").ok();

        if self.program.is_none() || self.outline_program.is_none() {
            console_log!("Failed to compile shaders!");
            panic!();
        }

        console_log!("Shader compiled!");

        self.render.enable_depth_test();

        self.view = Matrix4::new_translation(&vector!(0.0, 0.0, -1.0));
        self.projection = Matrix4::new_perspective(self.render.aspect(), 70.0, 0.1, 100.0);

        self.mesh.add_verticies(
            recursive_subdivide(
                vector!(0.0, 0.5, 0.0),
                (Matrix2::new(0.0, -1.0, 1.0, 0.0) * (vector!(0.0, 0.5, 0.0) * 3.0.sqrt()).xy()).push(0.0),
                vector!(0.0, -0.5, 0.0),
                5
            )
        );

        self.mesh.add_verticies(
            recursive_subdivide(
                vector!(0.0, -0.5, 0.0),
                (Matrix2::new(0.0, -1.0, 1.0, 0.0) * (vector!(0.0, -0.5, 0.0) * 3.0.sqrt()).xy()).push(0.0),
                vector!(0.0, 0.5, 0.0),
                5
            )
        );

        //self.mesh.update_buffers(&self.render);
        console_log!("Application started.");
        Ok(())
    }

    fn update(&mut self, dt: f32) {
        self.time += dt;
        self.projection = Matrix4::new_perspective(self.render.aspect(), 70.0, 0.01, 100.0);

        
        self.computed_mesh = Mesh::new();
        for vert in self.mesh.get_verticies() {
            let noise_in = (vert * 7.5) + vector!(128.0, 128.0, 0.0);

            let noise_out: Vector3<f32> = Vector3::new(0.0, 0.0,
                0.1 * self.perlin.get([noise_in.x as f64, noise_in.y as f64, (self.time * 0.5) as f64]) as f32,
            );

            self.computed_mesh.add_vertex(vert + noise_out);
            self.computed_mesh.add_color(vector!(0.114, 0.137, 0.165, 1.0));
        }

        self.computed_mesh.use_colors = true;

        self.computed_mesh.generate_normals();
        self.computed_mesh.update_buffers(&self.render);

        // Create wireframe mesh of lines
        self.outline_mesh = Mesh::new();
        let v = self.computed_mesh.get_verticies();
        for i in (0..self.computed_mesh.len()).step_by(3) {
            let o = vector!(0.0, 0.0, 0.00005);
            self.outline_mesh.add_verticies(vec!(v[i]+o, v[i + 1]+o, v[i + 1]+o, v[i + 2]+o));
        }
        self.outline_mesh.draw_mode = WebGlRenderingContext::LINES;
        self.outline_mesh.update_buffers(&self.render);
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
        model = model * Matrix4::new_scaling(upper_right.magnitude() * 2.5); // extra 0.5

        // This is probably over-correcting but I do not care :)
        let over_angle = angle - (60.0 * (std::f32::consts::PI / 180.0));
        if (over_angle >= 0.0) {
            model = model * Matrix4::new_scaling(1.0 + over_angle);
        }
        
        let mvp = self.projection * self.view * model;

        // Render mesh
        self.render.set_program(self.program.as_ref());
        self.program.as_ref().unwrap().set_uniform_matrix4f("mvp", mvp);
        self.program.as_ref().unwrap().set_uniform_matrix4f("normalMatrix", model.transpose().try_inverse().unwrap_or(Matrix4::identity()));
        self.program.as_ref().unwrap().set_uniform1f("time", self.time);
        self.program.as_ref().unwrap().set_uniform4f("ambientLightColor", self.ambient_light.color.push(self.ambient_light.intensity));
        self.program.as_ref().unwrap().set_uniform4f("directionalLightColor", self.dir_light.color.push(self.dir_light.intensity));
        self.program.as_ref().unwrap().set_uniform3f("directionalLightDir", self.dir_light.direction);
        self.render.draw_mesh(&self.computed_mesh);

        // Render the outline
        self.render.set_program(self.outline_program.as_ref());
        self.outline_program.as_ref().unwrap().set_uniform_matrix4f("mvp", mvp);
        self.outline_program.as_ref().unwrap().set_uniform4f("flatColor", vector!(0.851, 0.149, 0.663, 1.0));
        self.render.draw_mesh(&self.outline_mesh);
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
            computed_mesh: Mesh::new(),
            outline_mesh: Mesh::new(),
            program: None,
            outline_program: None,
            time: 0.0,
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            ambient_light: AmbientLight{
                color: vector!(1.0, 1.0, 1.0),
                intensity: 0.1,
            },
            dir_light: DirectionalLight{
                color: vector!(0.545, 0.329, 0.929),
                intensity: 0.66,
                direction: vector!(1.0, 1.0, 1.0).normalize(),
            },
            perlin: Perlin::new(3),
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
