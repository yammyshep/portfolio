use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::*;
use nalgebra::{ Vector1, Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4 };

pub const SHADER_SIMPLE_VERT: &str = include_str!("./simple.v.glsl");
pub const SHADER_SIMPLE_FRAG: &str = include_str!("./simple.f.glsl");

pub struct Shader {
    pub program: WebGlProgram,
    gl: WebGlRenderingContext,
    uniforms: RefCell<HashMap<String, WebGlUniformLocation>>,
}

impl Shader {
    /// Create a new Shader program from a vertex and fragment shader
    pub fn new(
        gl: &WebGlRenderingContext,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<Shader, JsValue> {
        let vert_shader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vert_shader)?;
        let frag_shader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;

        let uniforms = RefCell::new(HashMap::new());

        Ok(Shader { program, gl: gl.clone(), uniforms })
    }

    /// Get the location of a uniform.
    /// If this is our first time retrieving it we will cache it so that for future retrievals
    /// we won't need to query the shader program.
    pub fn get_uniform_location(&self, uniform_name: &str) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();

        if uniforms.get(uniform_name).is_none() {
            match self.gl.get_uniform_location(&self.program, uniform_name) {
                Some(loc) => uniforms.insert(uniform_name.to_string(), loc),
                None => None
            };
        }

        uniforms.get(uniform_name).map(|l| l.clone())
    }

    pub fn set_uniform1f(&self, name: &str, x: f32) -> Result<(), ()> {
        let location = self.get_uniform_location(name).ok_or(())?;
        self.gl.uniform1f(Some(&location), x);
        Ok(())
    }

    pub fn set_uniform1f_vec(&self, name: &str, val: Vector1<f32>) -> Result<(), ()> {
        self.set_uniform1f(name, val[0])
    }

    pub fn set_uniform2f(&self, name: &str, val: Vector2<f32>) -> Result<(), ()> {
        let location = self.get_uniform_location(name).ok_or(())?;
        self.gl.uniform2fv_with_f32_array(Some(&location), val.as_slice());
        Ok(())
    }

    pub fn set_uniform3f(&self, name: &str, val: Vector3<f32>) -> Result<(), ()> {
        let location = self.get_uniform_location(name).ok_or(())?;
        self.gl.uniform3fv_with_f32_array(Some(&location), val.as_slice());
        Ok(())
    }

    pub fn set_uniform4f(&self, name: &str, val: Vector4<f32>) -> Result<(), ()> {
        let location = self.get_uniform_location(name).ok_or(())?;
        self.gl.uniform4fv_with_f32_array(Some(&location), val.as_slice());
        Ok(())
    }

    pub fn set_uniform_matrix2f(&self, name: &str, val: Matrix2<f32>) -> Result<(), ()> {
        let location = self.get_uniform_location(name).ok_or(())?;
        self.gl.uniform_matrix2fv_with_f32_array(Some(&location), false, val.as_slice());
        Ok(())
    }
    
    pub fn set_uniform_matrix3f(&self, name: &str, val: Matrix3<f32>) -> Result<(), ()> {
        let location = self.get_uniform_location(name).ok_or(())?;
        self.gl.uniform_matrix3fv_with_f32_array(Some(&location), false, val.as_slice());
        Ok(())
    }
    
    pub fn set_uniform_matrix4f(&self, name: &str, val: Matrix4<f32>) -> Result<(), ()> {
        let location = self.get_uniform_location(name).ok_or(())?;
        self.gl.uniform_matrix4fv_with_f32_array(Some(&location), false, val.as_slice());
        Ok(())
    }
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
