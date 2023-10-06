use std::ffi::c_void;

use super::{
    c_interface::texture::{
        division_engine_texture_alloc, division_engine_texture_free,
        division_engine_texture_set_data, DivisionTextureDescriptor,
    },
    Context, DivisionId, Error, Image,
};

pub use super::c_interface::texture::{
    DivisionTextureChannelSwizzleVariant as TextureChannelSwizzleVariant,
    DivisionTextureChannelsSwizzle as TextureChannelsSwizzle,
    DivisionTextureFormat as TextureFormat,
};

impl Context {
    pub fn create_texture_buffer(
        &mut self,
        width: u32,
        height: u32,
        texture_format: TextureFormat,
    ) -> Result<DivisionId, Error> {
        self.create_texture_buffer_with_channels_swizzle(
            width,
            height,
            texture_format,
            None,
        )
    }

    pub fn create_texture_buffer_with_channels_swizzle(
        &mut self,
        width: u32,
        height: u32,
        texture_format: TextureFormat,
        channels_swizzle: Option<TextureChannelsSwizzle>,
    ) -> Result<DivisionId, Error> {
        let texture_desc = DivisionTextureDescriptor {
            width,
            height,
            texture_format,
            has_channels_swizzle: channels_swizzle.is_some(),
            channels_swizzle: match channels_swizzle {
                Some(v) => v,
                None => TextureChannelsSwizzle::default(),
            },
        };
        let mut texture_id = 0;
        unsafe {
            if !division_engine_texture_alloc(&mut *self, &texture_desc, &mut texture_id)
            {
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
        self.create_texture_buffer_from_data_with_channels_swizzle(
            width,
            height,
            texture_format,
            None,
            data,
        )
    }

    pub fn create_texture_buffer_from_data_with_channels_swizzle(
        &mut self,
        width: u32,
        height: u32,
        texture_format: TextureFormat,
        channels_swizzle: Option<TextureChannelsSwizzle>,
        data: &[u8],
    ) -> Result<DivisionId, Error> {
        let id = self.create_texture_buffer_with_channels_swizzle(
            width,
            height,
            texture_format,
            channels_swizzle,
        )?;
        self.set_texture_buffer_data(id, data);
        Ok(id)
    }

    pub fn set_texture_buffer_data(
        &mut self,
        texture_buffer_id: DivisionId,
        data: &[u8],
    ) {
        unsafe {
            division_engine_texture_set_data(
                &mut *self,
                texture_buffer_id,
                data.as_ptr() as *const c_void,
            )
        }
    }

    pub fn create_texture_buffer_from_image(
        &mut self,
        image: &Image,
    ) -> Result<DivisionId, Error> {
        self.create_texture_buffer_from_image_with_channels_swizzle(image, None)
    }

    pub fn create_texture_buffer_from_image_with_channels_swizzle(
        &mut self,
        image: &Image,
        channels_swizzle: Option<TextureChannelsSwizzle>,
    ) -> Result<DivisionId, Error> {
        self.create_texture_buffer_from_data_with_channels_swizzle(
            image.width(),
            image.height(),
            channels_to_texture_format(image.channels())?,
            channels_swizzle,
            image.data(),
        )
    }

    pub fn delete_texture_buffer(&mut self, texture_buffer_id: DivisionId) {
        unsafe {
            division_engine_texture_free(&mut *self, texture_buffer_id);
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

impl TextureChannelsSwizzle {
    pub fn all(value: TextureChannelSwizzleVariant) -> Self {
        Self {
            red: value,
            green: value,
            blue: value,
            alpha: value,
        }
    }
}

impl Default for TextureChannelsSwizzle {
    fn default() -> Self {
        Self {
            red: TextureChannelSwizzleVariant::Red,
            green: TextureChannelSwizzleVariant::Green,
            blue: TextureChannelSwizzleVariant::Blue,
            alpha: TextureChannelSwizzleVariant::Alpha,
        }
    }
}
