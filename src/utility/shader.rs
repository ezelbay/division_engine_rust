use std::{env, fs, io, path::Path};

use crate::core::{Context, DivisionId, context::Error, ShaderSourceDescriptor, ShaderType};

impl Context {
    /// Creates program with vertex and fragment bundled shaders with same names.
    /// [`shader_path`] should be relative to the executable folder.
    /// Not suitable for complex cases
    pub fn create_bundled_shader_program(
        &mut self,
        shader_path: &Path,
    ) -> Result<DivisionId, Error> {
        let shader_path = shader_path
            .to_str()
            .ok_or_else(|| Error::Core("The shader path is incorrect".to_string()))?;

        let (vert_entry, vert_path, frag_entry, frag_path) = if cfg!(target_os = "macos")
        {
            (
                "vert",
                format!("{shader_path}.vert.metal"),
                "frag",
                format!("{shader_path}.frag.metal"),
            )
        } else {
            (
                "main",
                format!("{shader_path}.vert"),
                "main",
                format!("{shader_path}.frag"),
            )
        };

        let bin_root_path = env::current_exe().unwrap();
        let bin_root_path = bin_root_path.parent().unwrap();

        self.create_shader_program(&[
            ShaderSourceDescriptor::new(
                ShaderType::Vertex,
                vert_entry,
                &fs::read_to_string(bin_root_path.join(vert_path))
                    .map_err(|e| new_shader_read_error("vertex", e))?,
            ),
            ShaderSourceDescriptor::new(
                ShaderType::Fragment,
                frag_entry,
                &fs::read_to_string(bin_root_path.join(frag_path))
                    .map_err(|e| new_shader_read_error("fragment", e))?,
            ),
        ])
    }
}

fn new_shader_read_error(shader_type: &str, base_error: io::Error) -> Error {
    let io_message = base_error.to_string();
    Error::Core(format!(
        "Failed to read {shader_type} shader. io::Error message: `{io_message}`"
    ))
}
