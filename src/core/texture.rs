use std::ffi::c_void;

use super::{
    c_interface::texture::{
        division_engine_texture_alloc, division_engine_texture_free,
        division_engine_texture_set_data, DivisionTextureDescriptor,
    },
    Core, DivisionId, Error, Image,
};

pub use super::c_interface::texture::DivisionTextureFormat as TextureFormat;

impl Core {
    pub fn create_texture_buffer(
        &mut self,
        width: u32,
        height: u32,
        texture_format: TextureFormat,
    ) -> Result<DivisionId, Error> {
        let texture_desc = DivisionTextureDescriptor {
            width,
            height,
            texture_format,
        };
        let mut texture_id = 0;
        unsafe {
            if !division_engine_texture_alloc(self.ctx, &texture_desc, &mut texture_id) {
                return Err(Error::Core("Failed to create texture".to_string()));
            }
        }

        Ok(texture_id)
    }

    pub fn create_texture_buffer_from_data(
        &mut self,
        width: u32,
        height: u32,
        texture_format: TextureFormat,
        data: &[u8],
    ) -> Result<DivisionId, Error> {
        let id = self.create_texture_buffer(width, height, texture_format)?;
        self.set_texture_buffer_data(id, data);
        Ok(id)
    }

    pub fn set_texture_buffer_data(&mut self, texture_buffer_id: DivisionId, data: &[u8]) {
        unsafe {
            division_engine_texture_set_data(
                self.ctx,
                texture_buffer_id,
                data.as_ptr() as *const c_void,
            )
        }
    }

    pub fn create_texture_buffer_from_image(&mut self, image: &Image) -> Result<DivisionId, Error> {
        self.create_texture_buffer_from_data(
            image.width(),
            image.height(),
            channels_to_texture_format(image.channels())?,            
            image.data(),
        )
    }

    pub fn delete_texture_buffer(&mut self, texture_buffer_id: DivisionId) {
        unsafe {
            division_engine_texture_free(self.ctx, texture_buffer_id);
        }
    }
}

#[inline]
fn channels_to_texture_format(channels: u32) -> Result<TextureFormat, Error> {
    Ok(match channels {
        1 => TextureFormat::R8Uint,
        3 => TextureFormat::RGB24Uint,
        4 => TextureFormat::RGBA32Uint,
        c => {
            return Err(Error::Core(format!(
                "Unknown texture format with color channels count: {c}"
            )))
        }
    })
}