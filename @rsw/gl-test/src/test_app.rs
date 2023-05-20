use web_sys::{WebGlRenderingContext, HtmlCanvasElement};
use nalgebra::vector;
use nalgebra::{Matrix4};
use wasm_bindgen::{JsCast, JsValue};

use crate::app::Application;
use crate::render::{Renderer, GlRenderer, mesh::Mesh};
use crate::shader::Shader;
use crate::shader::{SHADER_SIMPLE_FRAG, SHADER_SIMPLE_VERT};

pub struct TestApplication {
    render: GlRenderer,
    mesh: Mesh,
    rot: f32,
    program: Option<Shader>,
}

impl Application for TestApplication {
    fn start(&mut self) -> Result<(), JsValue> {
        self.program = self.render.create_shader(SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG).ok();

        self.mesh.add_vertex(vector!(0.0, 0.0, 0.0));
        self.mesh.add_vertex(vector!(-0.7, -0.7, 0.0));
        self.mesh.add_vertex(vector!(0.7, -0.7, 0.0));
        self.mesh.add_vertex(vector!(-0.7, 0.7, 0.0));
        self.mesh.add_vertex(vector!(0.0, 0.0, 0.0));
        self.mesh.add_vertex(vector!(0.7, 0.7, 0.0));
        self.mesh.add_vertex(vector!(-0.7, 0.7, 0.0));

        self.mesh.draw_mode = WebGlRenderingContext::LINE_STRIP;

        self.mesh.update_buffers(&self.render);
        Ok(())
    }

    fn update(&mut self, dt: f32) {
        self.rot += dt / 5.0;
    }

    fn render(&self) {
        self.render.clear(vector!(0.1, 0.1, 0.1, 1.0));
        let model = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, -3.0,
            0.0, 0.0, 0.0, 1.0
        );
        let model = model * Matrix4::from_euler_angles(0.0, self.rot, self.rot);
        let projection = Matrix4::new_perspective(self.render.aspect(), 70.0, 0.1, 100.0);
        let mvp = projection * model;
        self.program.as_ref().unwrap().set_uniform_matrix4f("mvp", mvp);
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
        Ok(TestApplication{ render, mesh: Mesh::new(), rot: 0.0, program: None })
    }
}