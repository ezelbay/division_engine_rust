use std::ffi::CString;

use super::{
    c_interface::shader::{
        division_engine_shader_program_alloc, division_engine_shader_program_free,
        DivisionShaderSourceDescriptor,
    },
    Context, DivisionId, Error,
};

pub use super::c_interface::shader::DivisionShaderType as ShaderType;
pub use super::c_interface::shader::DivisionShaderVariableType as ShaderVariableType;

pub struct ShaderSourceDescriptor {
    shader_type: ShaderType,
    entry_point: CString,
    source: CString,
}

impl Context {
    pub fn create_shader_program(
        &mut self,
        descriptors: &[ShaderSourceDescriptor],
    ) -> Result<DivisionId, Error> {
        let c_desc: Vec<DivisionShaderSourceDescriptor> = descriptors
            .into_iter()
            .map(|d| DivisionShaderSourceDescriptor {
                entry_point_name: d.entry_point.as_ptr(),
                shader_type: d.shader_type,
                source: d.source.as_ptr(),
                source_size: d.source.as_bytes().len() as i32,
            })
            .collect();

        let mut shader_id = 0;
        unsafe {
            if !division_engine_shader_program_alloc(
                &mut self.c_context,
                c_desc.as_ptr(),
                c_desc.len() as i32,
                &mut shader_id,
            ) {
                return Err(Error::Core(String::from("Failed to create a shader")));
            }
        }

        Ok(shader_id)
    }

    pub fn delete_shader_program(&mut self, id: DivisionId) {
        unsafe {
            division_engine_shader_program_free(&mut self.c_context, id);
        }
    }
}

impl ShaderSourceDescriptor {
    pub fn new(shader_type: ShaderType, entry_point: &str, source: &str) -> Self {
        return ShaderSourceDescriptor {
            source: CString::new(source).unwrap(),
            entry_point: CString::new(entry_point).unwrap(),
            shader_type,
        };
    }
}
