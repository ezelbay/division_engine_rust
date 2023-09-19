use std::{env, fs};

use crate::core::{Core, DivisionError, DivisionId, ShaderSourceDescriptor, ShaderType};

impl Core {
    /// Creates program with vertex and fragment bundled shaders with same names.
    /// [`shader_path`] should be relative to the executable folder. 
    /// Not suitable for complex cases
    pub fn create_builtin_bundled_shader_program(
        &mut self,
        shader_path: &str,
    ) -> Result<DivisionId, DivisionError> {
        let (vert_entry, vert_path, frag_entry, frag_path) = if cfg!(target_os = "macos") {
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
                &fs::read_to_string(bin_root_path.join(vert_path)).unwrap(),
            ),
            ShaderSourceDescriptor::new(
                ShaderType::Fragment,
                frag_entry,
                &fs::read_to_string(bin_root_path.join(frag_path)).unwrap(),
            ),
        ])
    }
}