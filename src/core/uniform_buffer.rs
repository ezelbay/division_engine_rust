use std::ffi::{c_ulong, c_void};

use super::{
    ffi::{
        context::DivisionContext,
        uniform_buffer::{
            division_engine_uniform_buffer_alloc,
            division_engine_uniform_buffer_borrow_data_pointer,
            division_engine_uniform_buffer_free,
            division_engine_uniform_buffer_return_data_pointer,
            DivisionUniformBufferDescriptor,
        },
    },
    Context, DivisionId, context::Error,
};

pub struct UniformBufferData<'a, T> {
    pub data: &'a mut T,

    ctx: *mut DivisionContext,
    id: DivisionId,
}

impl Context {
    pub fn create_uniform_buffer(
        &mut self,
        size_bytes: usize,
    ) -> Result<DivisionId, Error> {
        let mut buffer_id = 0;
        unsafe {
            let desc = DivisionUniformBufferDescriptor {
                data_bytes: size_bytes as c_ulong,
            };
            if !division_engine_uniform_buffer_alloc(
                &mut *self,
                desc,
                &mut buffer_id,
            ) {
                return Err(Error::Core(
                    "Failed to create an uniform buffer".to_string(),
                ));
            }
        }

        return Ok(buffer_id);
    }

    pub fn create_uniform_buffer_with_size_of<T>(&mut self) -> Result<DivisionId, Error> {
        self.create_uniform_buffer(std::mem::size_of::<T>())
    }

    pub fn uniform_buffer_data<T>(
        &mut self,
        uniform_buffer_id: DivisionId,
    ) -> UniformBufferData<T> {
        unsafe {
            let ptr = division_engine_uniform_buffer_borrow_data_pointer(
                &mut *self,
                uniform_buffer_id,
            );
            let ptr = ptr as *mut T;

            UniformBufferData {
                data: ptr.as_mut().unwrap(),
                ctx: &mut *self,
                id: uniform_buffer_id,
            }
        }
    }

    pub fn delete_uniform_buffer(&mut self, uniform_buffer_id: DivisionId) {
        unsafe {
            division_engine_uniform_buffer_free(&mut *self, uniform_buffer_id);
        }
    }
}

impl<'a, T> Drop for UniformBufferData<'a, T> {
    fn drop(&mut self) {
        unsafe {
            division_engine_uniform_buffer_return_data_pointer(
                self.ctx,
                self.id,
                self.data as *mut T as *mut c_void,
            )
        }
    }
}
