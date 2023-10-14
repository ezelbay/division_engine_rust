use std::{
    ffi::{c_int, CString},
    mem::MaybeUninit,
    path::Path,
    slice,
};

use super::{
    ffi::stb::{
        stbi_image_free, stbi_load, stbi_load_from_memory,
        stbi_set_flip_vertically_on_load, stbi_write_jpg,
    },
    context::Error,
};

enum ImageImpl {
    Stb(*mut u8),
    Raw(Vec<u8>),
}

pub struct Image {
    imp: ImageImpl,
    width: usize,
    height: usize,
    channels: usize,
}

impl Image {
    pub unsafe fn create_from_raw_in_memory(
        buffer: Vec<u8>,
        width: usize,
        height: usize,
        channels: usize,
    ) -> Image {
        Image {
            imp: ImageImpl::Raw(buffer),
            width,
            height,
            channels,
        }
    }

    pub fn create_from_compressed_in_memory(buffer: Vec<u8>) -> Option<Image> {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            let mut channels = MaybeUninit::uninit();

            stbi_set_flip_vertically_on_load(1);
            let ptr = stbi_load_from_memory(
                buffer.as_ptr(),
                buffer.len() as c_int,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                channels.as_mut_ptr(),
                0,
            );

            if ptr.is_null() {
                return None;
            }

            return Some(Image {
                imp: ImageImpl::Stb(ptr),
                width: width.assume_init() as usize,
                height: height.assume_init() as usize,
                channels: channels.assume_init() as usize,
            });
        }
    }

    pub fn create_from_path(path: &Path) -> Option<Image> {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            let mut channels = MaybeUninit::uninit();

            let path = path.to_str()?;
            let path = CString::new(path);
            let path = path.ok()?;

            let ptr = stbi_load(
                path.as_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                channels.as_mut_ptr(),
                0,
            );

            if ptr.is_null() {
                return None;
            }

            return Some(Image {
                imp: ImageImpl::Stb(ptr),
                width: width.assume_init() as usize,
                height: height.assume_init() as usize,
                channels: channels.assume_init() as usize,
            });
        }
    }

    pub fn write_to_file_jpg(&self, path: &Path) -> Result<(), Error> {
        self.write_to_file_jpg_with_quality(path, 80)
    }

    pub fn write_to_file_jpg_with_quality(
        &self,
        path: &Path,
        quality: u32,
    ) -> Result<(), Error> {
        let c_str = CString::new(path.to_str().unwrap()).unwrap();
        let result = unsafe {
            stbi_write_jpg(
                c_str.as_ptr(),
                self.width as c_int,
                self.height as c_int,
                self.channels as c_int,
                self.data().as_ptr(),
                quality as c_int,
            )
        };
        
        if result {
            Ok(())
        } else {
            Err(Error::Core("Failed to write an image to file".to_string()))
        }
    }

    pub fn data(&self) -> &[u8] {
        match self.imp {
            ImageImpl::Raw(ref buf) => buf,
            ImageImpl::Stb(ptr) => unsafe { slice::from_raw_parts(ptr, self.len()) },
        }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        match self.imp {
            ImageImpl::Raw(ref mut buf) => buf,
            ImageImpl::Stb(ptr) => unsafe { slice::from_raw_parts_mut(ptr, self.len()) },
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.width as usize * self.height as usize * self.channels as usize
    }

    #[inline]
    pub fn channels(&self) -> usize {
        self.channels
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if let ImageImpl::Stb(ptr) = self.imp {
            unsafe { stbi_image_free(ptr) }
        }
    }
}
