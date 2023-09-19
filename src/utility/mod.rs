mod image;
mod shader;

use std::{path::PathBuf, env};

pub use image::*;
pub use shader::*;

pub fn make_exe_dir_path() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
}