use alloc::ffi::CString;
use crate::division_engine::bridge::shader::*;
use crate::division_engine::rendering::*;

pub struct ShaderProgramBuilder {
    _id: i32
}

impl ShaderProgramBuilder {
    pub fn new() -> ShaderProgramBuilder {
        return ShaderProgramBuilder {
            _id : unsafe {
                division_engine_shader_create_program()
            }
        };
    }

    pub fn add_shader_source(&self, path: &str, shader_type: ShaderType) -> &Self {
        let c_path = CString::new(path).unwrap();
        unsafe {
            division_engine_shader_attach_to_program(
                c_path.as_ptr(), shader_type, self._id);
        }

        return self;
    }

    pub fn compile(&self) -> ShaderProgram {
        unsafe {
            assert!(division_engine_shader_link_program(self._id));
        }
        return ShaderProgram::new(self._id);
    }
}