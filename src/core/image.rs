use std::{
    ffi::{c_int, CString},
    mem::MaybeUninit,
    path::Path,
    slice,
};

use super::c_interface::stb::{stbi_image_free, stbi_load, stbi_load_from_memory, stbi_set_flip_vertically_on_load};

pub struct Image {
    ptr: *mut u8,
    width: u32,
    height: u32,
    channels: u32,
}

impl Image {
    pub fn create_from_memory(buffer: Vec<u8>) -> Option<Image> {
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
                ptr,
                width: width.assume_init() as u32,
                height: height.assume_init() as u32,
                channels: channels.assume_init() as u32,
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

            stbi_set_flip_vertically_on_load(1);

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
                ptr,
                width: width.assume_init() as u32,
                height: height.assume_init() as u32,
                channels: channels.assume_init() as u32,
            });
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len()) }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len()) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.width as usize * self.height as usize * self.channels as usize
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.channels
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            stbi_image_free(self.ptr);
        }
    }
}
