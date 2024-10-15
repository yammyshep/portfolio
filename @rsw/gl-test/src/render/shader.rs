use web_sys::*;
use include_dir::{include_dir, Dir};
use std::collections::HashMap;

use crate::render::ShaderErr;

static SHADERS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/shaders");

pub struct Shader {
    name: String,
    shader_type: u32,
    wgl_shader: Option<WebGlShader>,
    gl: WebGlRenderingContext,
}

impl Shader {
    pub fn new(
        gl: &WebGlRenderingContext,
        name: String,
        shader_type: u32
    ) -> Self {
        Self {
            name,
            shader_type,
            wgl_shader: None,
            gl: gl.clone(),
        }
    }

    pub fn compile(&mut self) -> Result<(), ShaderErr> {
        self.wgl_shader = Some(self.gl.create_shader(self.shader_type).ok_or(ShaderErr::ShaderCreateErr)?);
        let shader = self.wgl_shader.as_ref().unwrap();

        let source = load_shader(&self.name, &HashMap::new())?;

        self.gl.shader_source(shader, &source);
        self.gl.compile_shader(shader);

        let compiled = self.gl.get_shader_parameter(shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false);

        if compiled { Ok(()) } else { Err(ShaderErr::UnknownError) }
    }

    pub fn attach(&self, prog: &WebGlProgram) -> Result<(), ShaderErr> {
        self.gl.attach_shader(prog, &self.wgl_shader.as_ref().ok_or(ShaderErr::LinkErr)?.clone());
        Ok(())
    }
}

fn load_shader(filename: &str, mut defines: &HashMap<String, String>) -> Result<String, ShaderErr> {
    let source = SHADERS_DIR.get_file(filename).ok_or(ShaderErr::FileNotFound)?.contents_utf8().ok_or(ShaderErr::UnknownError)?;
    preprocess(source, defines)
}

fn preprocess(source: &str, mut defines: &HashMap<String, String>) -> Result<String, ShaderErr> {
    let mut source_out = String::new();
    for line in source.lines() {
        let trim = line.trim();
        if trim.starts_with("#include") {
            let mut filename = trim.split_whitespace().skip(1).next()
                .ok_or(ShaderErr::UnknownError)?.to_string();
            filename.retain(|c| !r#""<>"#.contains(c));

            source_out.push_str(&preprocess(&filename, defines)?);
        } else {
            source_out.push_str(line);
        }
        source_out.push_str("\n");
    }
    Ok(source_out)
}
