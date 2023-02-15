use crate::division_engine::bridge::shader::*;
pub use crate::division_engine::bridge::shader::ShaderType;

pub struct ShaderProgram {
    _id: i32
}

impl ShaderProgram {
    pub fn new(id: i32) -> Self { ShaderProgram { _id: id } }
    pub fn id(&self) -> i32 { self._id }
    pub fn set_current(&self) {
        unsafe {
            division_engine_shader_use_program(self._id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            division_engine_shader_destroy_program(self._id);
        }
    }
}