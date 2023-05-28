use std::ffi::c_void;

use super::{
    interface::texture::{
        division_engine_texture_alloc, division_engine_texture_free,
        division_engine_texture_set_data, TextureDescriptor,
    },
    DivisionCore, DivisionError, DivisionId,
};

pub use super::interface::texture::TextureFormat;

impl DivisionCore {
    pub fn create_texture_buffer(
        &mut self,
        width: u32,
        height: u32,
        texture_format: TextureFormat,
    ) -> Result<DivisionId, DivisionError> {
        let texture_desc = TextureDescriptor {
            width,
            height,
            texture_format,
        };
        let mut texture_id = 0;
        unsafe {
            if !division_engine_texture_alloc(self.ctx, &texture_desc, &mut texture_id) {
                return Err(DivisionError::Core("Failed to create texture".to_string()));
            }
        }

        Ok(texture_id)
    }

    pub fn create_texture_buffer_with_data(
        &mut self,
        width: u32,
        height: u32,
        texture_format: TextureFormat,
        data: &[u8],
    ) -> Result<DivisionId, DivisionError> {
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

    pub fn delete_texture_buffer(&mut self, texture_buffer_id: DivisionId) {
        unsafe {
            division_engine_texture_free(self.ctx, texture_buffer_id);
        }
    }
}
