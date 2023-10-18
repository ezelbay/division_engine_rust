use std::path::Path;

use crate::core::{Image, TextureFormat, ImageSettings};

use super::make_exe_dir_path;

impl Image {
    pub fn create_bundled_image(
        path: &Path,
        image_settings: ImageSettings,
    ) -> Option<Image> {
        let exe_dir = make_exe_dir_path();
        let path = exe_dir.join(path);

        Image::create_from_path(&path, image_settings)
    }
}
