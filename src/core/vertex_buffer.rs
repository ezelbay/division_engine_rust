use std::mem::MaybeUninit;

use super::{
    context::Error,
    ffi::{
        context::DivisionContext,
        vertex_buffer::{
            division_engine_vertex_buffer_alloc,
            division_engine_vertex_buffer_borrow_data,
            division_engine_vertex_buffer_free, division_engine_vertex_buffer_resize,
            division_engine_vertex_buffer_return_data, DivisionVertexBufferBorrowedData,
            DivisionVertexBufferDescriptor,
        },
    },
    Context, DivisionId,
};

pub use super::ffi::{
    shader::DivisionShaderVariableType as ShaderVariableType,
    vertex_buffer::{
        DivisionRenderTopology as RenderTopology,
        DivisionVertexAttributeDescriptor as VertexAttributeDescriptor,
        DivisionVertexBufferSize as VertexBufferSize,
    },
};

pub use division_engine_rust_macro::*;

pub trait VertexData {
    fn vertex_attributes() -> Vec<VertexAttributeDescriptor>;
}

pub struct VertexBufferData<'a, TVertexData, TInstanceData> {
    pub per_vertex_data: &'a mut [TVertexData],
    pub per_instance_data: &'a mut [TInstanceData],
    pub vertex_indices: &'a mut [u32],

    ctx: *mut DivisionContext,
    borrowed: DivisionVertexBufferBorrowedData,
    vertex_buffer_id: u32,
}

impl Context {
    pub fn create_vertex_buffer<TVertexData: VertexData, TInstanceData: VertexData>(
        &mut self,
        size: VertexBufferSize,
        topology: RenderTopology,
    ) -> Result<DivisionId, Error> {
        self.create_vertex_buffer_with_attributes(
            &TVertexData::vertex_attributes(),
            &TInstanceData::vertex_attributes(),
            size,
            topology,
        )
    }

    pub fn create_vertex_buffer_with_attributes(
        &mut self,
        per_vertex_attributes: &[VertexAttributeDescriptor],
        per_instance_attributes: &[VertexAttributeDescriptor],
        size: VertexBufferSize,
        topology: RenderTopology,
    ) -> Result<DivisionId, Error> {
        let mut id = 0;

        unsafe {
            if !division_engine_vertex_buffer_alloc(
                self,
                &DivisionVertexBufferDescriptor {
                    size,
                    per_vertex_attributes: per_vertex_attributes.as_ptr(),
                    per_vertex_attribute_count: per_vertex_attributes.len() as i32,
                    per_instance_attributes: per_instance_attributes.as_ptr(),
                    per_instance_attribute_count: per_instance_attributes.len() as i32,
                    topology,
                },
                &mut id,
            ) {
                return Err(Error::Core(String::from(
                    "Failed to create a vertex buffer",
                )));
            }
        }

        Ok(id)
    }

    pub fn vertex_buffer_resize(
        &mut self,
        vertex_buffer_id: DivisionId,
        new_size: VertexBufferSize,
    ) {
        unsafe {
            division_engine_vertex_buffer_resize(self, vertex_buffer_id, new_size);
        }
    }

    pub fn vertex_buffer_data<'a, TVertex, TInstance>(
        &'a mut self,
        vertex_buffer_id: DivisionId,
    ) -> VertexBufferData<TVertex, TInstance> {
        unsafe {
            let mut borrowed = MaybeUninit::uninit();
            division_engine_vertex_buffer_borrow_data(
                self,
                vertex_buffer_id,
                borrowed.as_mut_ptr(),
            );

            let borrowed = borrowed.assume_init();
            let per_vert_ptr = borrowed.vertex_data_ptr as *mut TVertex;
            let per_inst_ptr = borrowed.instance_data_ptr as *mut TInstance;
            let index_ptr = borrowed.index_data_ptr as *mut u32;
            let buff_size = &borrowed.size;

            return VertexBufferData {
                ctx: &mut *self,
                vertex_buffer_id,
                per_vertex_data: std::slice::from_raw_parts_mut(
                    per_vert_ptr,
                    buff_size.vertex_count as usize,
                ),
                per_instance_data: std::slice::from_raw_parts_mut(
                    per_inst_ptr,
                    buff_size.instance_count as usize,
                ),
                vertex_indices: std::slice::from_raw_parts_mut(
                    index_ptr,
                    buff_size.index_count as usize,
                ),
                borrowed,
            };
        }
    }

    pub fn delete_vertex_buffer(&mut self, vertex_buffer_id: DivisionId) {
        unsafe {
            division_engine_vertex_buffer_free(&mut *self, vertex_buffer_id);
        }
    }
}

impl<'a, TX, TY> Drop for VertexBufferData<'a, TX, TY> {
    fn drop(&mut self) {
        unsafe {
            division_engine_vertex_buffer_return_data(
                self.ctx,
                self.vertex_buffer_id,
                &mut self.borrowed,
            )
        }
    }
}
