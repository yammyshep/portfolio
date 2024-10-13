use web_sys::*;
use std::cell::RefCell;
use std::collections::HashMap;
use nalgebra::{ Vector1, Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4 };

use crate::shader::ShaderErr;
use crate::shader::shader::Shader;

pub struct Program {
    pub wgl_program: WebGlProgram,
    gl: WebGlRenderingContext,
    uniforms: RefCell<HashMap<String, WebGlUniformLocation>>,
}

impl Program {
    pub fn new(gl: &WebGlRenderingContext) -> Result<Self, ShaderErr> {
        Ok(Self { 
            gl: gl.clone(),
            wgl_program: gl.create_program().ok_or(ShaderErr::ProgramCreateErr)?,
            uniforms: RefCell::new(HashMap::new())
        })
    }

    pub fn link (
        &mut self,
        vert_shader: &Shader,
        frag_shader: &Shader,
    ) -> Result<(), ShaderErr> {
        vert_shader.attach(&self.wgl_program)?;
        frag_shader.attach(&self.wgl_program)?;
        self.gl.link_program(&self.wgl_program);

        let linked = self.gl.get_program_parameter(&self.wgl_program, WebGlRenderingContext::LINK_STATUS)
            .as_bool().unwrap_or(false);

        if linked { Ok(()) } else { Err(ShaderErr::LinkErr) }
    }

    /// Get the location of a uniform.
    /// If this is our first time retrieving it we will cache it so that for future retrievals
    /// we won't need to query the shader program.
    pub fn get_uniform_location(&self, uniform_name: &str) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();

        if uniforms.get(uniform_name).is_none() {
            match self.gl.get_uniform_location(&self.wgl_program, uniform_name) {
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