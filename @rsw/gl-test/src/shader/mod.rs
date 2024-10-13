pub mod shader;
pub mod program;

pub enum ShaderErr {
    FileNotFound,
    ShaderCreateErr,
    ProgramCreateErr,
    CompileErr,
    LinkErr,
    UnknownError,
}
