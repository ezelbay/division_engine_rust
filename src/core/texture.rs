use std::ffi::c_void;

use super::{
    ffi::texture::{
        division_engine_texture_alloc, division_engine_texture_free,
        division_engine_texture_set_data,
    },
    context::Error,
    Context, DivisionId, Image,
};

pub use super::ffi::texture::{
    DivisionTextureChannelSwizzleVariant as TextureChannelSwizzleVariant,
    DivisionTextureChannelsSwizzle as TextureChannelsSwizzle,
    DivisionTextureDescriptor as TextureDescriptor,
    DivisionTextureFormat as TextureFormat,
    DivisionTextureMinMagFilter as TextureMinMagFilter,
};

impl Context {
    pub fn create_texture_buffer(
        &mut self,
        texture_descriptor: &TextureDescriptor,
    ) -> Result<DivisionId, Error> {
        let mut texture_id = 0;
        unsafe {
            if !division_engine_texture_alloc(
                &mut *self,
                texture_descriptor,
                &mut texture_id,
            ) {
                return Err(Error::Core("Failed to create texture".to_string()));
            }
        }

        Ok(texture_id)
    }

    pub fn create_texture_buffer_from_data(
        &mut self,
        texture_descriptor: &TextureDescriptor,
        data: &[u8],
    ) -> Result<DivisionId, Error> {
        let id = self.create_texture_buffer(texture_descriptor)?;
        self.set_texture_buffer_data(id, data);
        Ok(id)
    }

    pub fn set_texture_buffer_data(&mut self, texture_id: DivisionId, data: &[u8]) {
        unsafe {
            division_engine_texture_set_data(
                &mut *self,
                texture_id,
                data.as_ptr() as *const c_void,
            )
        }
    }

    pub unsafe fn set_texture_buffer_data_ptr(
        &mut self,
        texture_id: DivisionId,
        data_ptr: *const u8,
    ) {
        unsafe {
            division_engine_texture_set_data(
                &mut *self,
                texture_id,
                data_ptr as *const c_void,
            )
        }
    }

    pub fn create_texture_buffer_from_image(
        &mut self,
        image: &Image,
    ) -> Result<DivisionId, Error> {
        self.create_texture_buffer_from_image_advanced(
            image,
            None,
            TextureMinMagFilter::Nearest,
            TextureMinMagFilter::Nearest,
        )
    }

    pub fn create_texture_buffer_from_image_advanced(
        &mut self,
        image: &Image,
        channels_swizzle: Option<TextureChannelsSwizzle>,
        min_filter: TextureMinMagFilter,
        mag_filter: TextureMinMagFilter,
    ) -> Result<DivisionId, Error> {
        let mut desc = TextureDescriptor::new(
            image.width(),
            image.height(),
            channels_to_texture_format(image.channels())?,
        )
        .with_min_mag_filter(min_filter, mag_filter);

        if let Some(s) = channels_swizzle {
            desc.channels_swizzle = s;
            desc.has_channels_swizzle = true;
        }

        self.create_texture_buffer_from_data(&desc, image.data())
    }

    pub fn delete_texture_buffer(&mut self, texture_buffer_id: DivisionId) {
        unsafe {
            division_engine_texture_free(&mut *self, texture_buffer_id);
        }
    }
}

#[inline]
fn channels_to_texture_format(channels: usize) -> Result<TextureFormat, Error> {
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

impl TextureDescriptor {
    pub fn new(width: usize, height: usize, texture_format: TextureFormat) -> Self {
        Self {
            width: width as u32,
            height: height as u32,
            texture_format,
            has_channels_swizzle: false,
            channels_swizzle: TextureChannelsSwizzle::default(),
            min_filter: TextureMinMagFilter::Nearest,
            mag_filter: TextureMinMagFilter::Nearest,
        }
    }

    pub fn with_channels_swizzle(
        mut self,
        channels_swizzle: TextureChannelsSwizzle,
    ) -> Self {
        self.has_channels_swizzle = true;
        self.channels_swizzle = channels_swizzle;

        self
    }

    pub fn with_min_mag_filter(
        mut self,
        min_filter: TextureMinMagFilter,
        mag_filter: TextureMinMagFilter,
    ) -> Self {
        self.min_filter = min_filter;
        self.mag_filter = mag_filter;

        self
    }
}
