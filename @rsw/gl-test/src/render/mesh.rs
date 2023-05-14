use nalgebra::{Vector2, Vector3, Vector4};
use crate::render::Renderer;

pub type Mesh = MeshGen<Vector3<f32>, Vector3<f32>, Vector4<f32>, Vector2<f32>>;

#[derive(Debug)]
pub struct MeshGen<V, N, C, T> {
    verticies: Vec<V>,
    normals: Vec<N>,
    colors: Vec<C>,
    texcoords: Vec<T>,
    use_normals: bool,
    use_colors: bool,
    use_texcoords: bool,
}

impl<V, N, C, T> MeshGen<V, N, C, T> {
    pub fn new() -> Self {
        MeshGen {
            verticies: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            texcoords: Vec::new(),
            use_normals: false,
            use_colors: false,
            use_texcoords: false,
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

    pub fn using_normals(&self) -> bool { self.use_normals }
    pub fn using_colors(&self) -> bool { self.use_colors }
    pub fn using_texcoords(&self) -> bool { self.use_texcoords }

    pub fn draw(&self) {
        
    }
}
