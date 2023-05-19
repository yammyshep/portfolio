use nalgebra::{Vector2, Vector3, Vector4};
use crate::render::Renderer;
use web_sys::{WebGlBuffer, WebGlRenderingContext};
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log { ($($t:tt)*) => (log(&format_args!($($t)*).to_string())) }

pub type Mesh = MeshGen<Vector3<f32>, Vector3<f32>, Vector4<f32>, Vector2<f32>>;

#[derive(Debug)]
pub struct MeshGen<V, N, C, T> {
    verticies: Vec<V>,
    normals: Vec<N>,
    colors: Vec<C>,
    texcoords: Vec<T>,
    vertex_buffer: Option<WebGlBuffer>,
    normal_buffer: Option<WebGlBuffer>,
    colors_buffer: Option<WebGlBuffer>,
    texcoord_buffer: Option<WebGlBuffer>,
    use_normals: bool,
    use_colors: bool,
    use_texcoords: bool,
    pub draw_mode: u32,
}

impl<V, N, C, T> MeshGen<V, N, C, T> {
    pub fn new() -> Self {
        MeshGen {
            verticies: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            texcoords: Vec::new(),
            vertex_buffer: None,
            normal_buffer: None,
            colors_buffer: None,
            texcoord_buffer: None,
            use_normals: false,
            use_colors: false,
            use_texcoords: false,
            draw_mode: WebGlRenderingContext::TRIANGLES,
        }
    }

    pub fn add_vertex(&mut self, vert: V) {
        self.verticies.push(vert);
    }

    pub fn add_normal(&mut self, norm: N) {
        self.normals.push(norm);
    }

    pub fn add_color(&mut self, color: C) {
        self.colors.push(color);
    }

    pub fn add_texcoord(&mut self, coord: T) {
        self.texcoords.push(coord);
    }

    pub fn get_verticies(&self) -> &Vec<V> { &self.verticies }
    pub fn get_normals(&self) -> &Vec<N> { &self.normals }
    pub fn get_colors(&self) -> &Vec<C> { &self.colors }
    pub fn get_texcoords(&self) -> &Vec<T> { &self.texcoords }

    pub fn num_verticies(&self) -> i32 {
        self.verticies.len() as i32
    }

    pub fn using_normals(&self) -> bool { self.use_normals }
    pub fn using_colors(&self) -> bool { self.use_colors }
    pub fn using_texcoords(&self) -> bool { self.use_texcoords }

    pub fn get_vertex_buffer(&self) -> Option<&WebGlBuffer> { self.vertex_buffer.as_ref() }
    pub fn get_normal_buffer(&self) -> Option<&WebGlBuffer> { self.normal_buffer.as_ref() }
    pub fn get_colors_buffer(&self) -> Option<&WebGlBuffer> { self.colors_buffer.as_ref() }
    pub fn get_texcoord_buffer(&self) -> Option<&WebGlBuffer> { self.texcoord_buffer.as_ref() }

    pub fn draw(&self) {
        
    }
}

impl Mesh {
    pub fn update_buffers(&mut self, render: &dyn Renderer) {
        if self.vertex_buffer == None {
            self.vertex_buffer = render.create_buffer().ok();
        }

        //TODO: Fix this fucking awful algorithm
        let indicies: usize = (self.num_verticies() * 3).try_into().unwrap();
        let vert_array = Float32Array::new_with_length(indicies as u32);
        let vertslice = self.verticies.as_slice();
        let verts = self.num_verticies();
        for i in 0..verts {
            for j in 0..3 {
                let index: usize = i.try_into().unwrap();
                let pos: i32 = (i*3) + (j as i32);
                vert_array.set_index(pos as u32, vertslice[index][j]);
            }
        }

        let gl = render.get_gl().unwrap();
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, self.vertex_buffer.as_ref());
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
}