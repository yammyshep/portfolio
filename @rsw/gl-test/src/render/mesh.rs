use nalgebra::*;
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

    pub fn add_verticies(&mut self, verts: Vec<V>) {
        for vert in verts {
            self.add_vertex(vert);
        }
    }

    pub fn add_normals(&mut self, norms: Vec<N>) {
        for norm in norms {
            self.add_normal(norm);
        }
    }

    pub fn add_colors(&mut self, colors: Vec<C>) {
        for color in colors {
            self.add_color(color);
        }
    }

    pub fn add_texcoords(&mut self, coords: Vec<T>) {
        for coord in coords {
            self.add_texcoord(coord);
        }
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

        let vert_array = copy_to_array(&self.verticies);
        self.bind_array_to_buffer(&vert_array, &self.vertex_buffer, render);

        if self.use_normals {
            if self.normal_buffer.is_none() {
                self.normal_buffer = render.create_buffer().ok();
            }

            let norm_array = copy_to_array(&self.normals);
            self.bind_array_to_buffer(&norm_array, &self.normal_buffer, render);
        }

        if self.use_colors {
            if self.colors_buffer.is_none() {
                self.colors_buffer = render.create_buffer().ok()
            }

            let color_array = copy_to_array(&self.colors);
            self.bind_array_to_buffer(&color_array, &self.colors_buffer, render);
        }

        if self.use_texcoords {
            if self.texcoord_buffer.is_none() {
                self.texcoord_buffer = render.create_buffer().ok();
            }

            let texcoord_array = copy_to_array(&self.texcoords);
            self.bind_array_to_buffer(&texcoord_array, &self.texcoord_buffer, render);
        }
    }

    fn bind_array_to_buffer(&self, array: &Float32Array, buffer: &Option<WebGlBuffer>, render: &dyn Renderer) {
        let gl = render.get_gl().unwrap();
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, buffer.as_ref());
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            array,
            WebGlRenderingContext::STATIC_DRAW
        );
    }
}

fn copy_to_array<D, S>(input: &Vec<Vector<f32, D, S>>) -> Float32Array 
    where D: Dim, S: RawStorage<f32, D> {
    let indicies: u32 = input.len().try_into().unwrap();
    let stride: u32 = input[0].len().try_into().unwrap();
    let array = Float32Array::new_with_length(indicies * stride);

    let slice = input.as_slice();
    for i in 0..indicies {
        for j in 0..stride {
            array.set_index((i*3)+j, slice[i as usize][j as usize].try_into().unwrap());
        }
    }

    array
}
