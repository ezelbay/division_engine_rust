mod image;
mod shader;

use std::{env, path::PathBuf};

pub use image::*;
pub use shader::*;

pub fn make_exe_dir_path() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
}
