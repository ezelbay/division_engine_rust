use std::{ffi::CString};

use super::{
    interface::{
        shader::{
            division_engine_shader_program_alloc, division_engine_shader_program_free,
            DivisionShaderSourceDescriptor,
        },
    },
    DivisionCore, DivisionError,
};

pub use super::interface::shader::ShaderType;
pub use super::interface::shader::ShaderVariableType;

pub struct ShaderProgram {
    id: u32,
}

pub struct ShaderSourceDescriptor {
    shader_type: ShaderType,
    entry_point: CString,
    source: CString,
}

impl DivisionCore {
    pub fn create_shader_program(&self, descriptors: &[ShaderSourceDescriptor]) -> Result<ShaderProgram, DivisionError> {
        let c_desc: Vec<DivisionShaderSourceDescriptor> = descriptors
            .into_iter()
            .map(|d| DivisionShaderSourceDescriptor {
                entry_point_name: d.entry_point.as_ptr(),
                shader_type: d.shader_type,
                source: d.source.as_ptr(),
                source_size: d.source.as_bytes().len() as i32,
            })
            .collect();

        let mut shader_program = ShaderProgram { id: 0 };
        unsafe {
            if !division_engine_shader_program_alloc(
                self.ctx,
                c_desc.as_ptr(),
                c_desc.len() as i32,
                &mut shader_program.id,
            ) {
                return Err(DivisionError::Core(String::from("Failed to create a shader")));
            }
        }

        Ok(shader_program)
    }

    pub fn drop_shader_program(&self, shader_program: ShaderProgram) {
        unsafe {
            division_engine_shader_program_free(self.ctx, shader_program.id);
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
