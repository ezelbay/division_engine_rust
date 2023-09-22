use std::mem::MaybeUninit;

use super::{
    c_interface::{
        context::DivisionContext,
        vertex_buffer::{
            division_engine_vertex_buffer_alloc,
            division_engine_vertex_buffer_borrow_data,
            division_engine_vertex_buffer_free,
            division_engine_vertex_buffer_return_data, DivisionVertexBufferBorrowedData,
            DivisionVertexBufferDescriptor,
        },
    },
    Context, DivisionId, Error,
};

pub use super::c_interface::{
    shader::DivisionShaderVariableType as ShaderVariableType,
    vertex_buffer::{
        DivisionRenderTopology as RenderTopology,
        DivisionVertexAttributeDescriptor as VertexAttributeDescriptor,
    },
};

pub struct VertexBufferData<'a, TVertexData, TInstanceData> {
    pub per_vertex_data: &'a mut [TVertexData],
    pub per_instance_data: &'a mut [TInstanceData],
    pub vertex_indices: &'a mut [u32],

    ctx: *mut DivisionContext,
    borrowed: DivisionVertexBufferBorrowedData,
    vertex_buffer_id: u32,
}

impl Context {
    pub fn create_vertex_buffer(
        &mut self,
        per_vertex_attributes: &[VertexAttributeDescriptor],
        per_instance_attributes: &[VertexAttributeDescriptor],
        vertex_count: usize,
        index_count: usize,
        instance_count: usize,
        topology: RenderTopology,
    ) -> Result<DivisionId, Error> {
        let mut id = 0;

        unsafe {
            if !division_engine_vertex_buffer_alloc(
                &mut *self,
                &DivisionVertexBufferDescriptor {
                    per_vertex_attributes: per_vertex_attributes.as_ptr(),
                    per_vertex_attribute_count: per_vertex_attributes.len() as i32,
                    per_instance_attributes: per_instance_attributes.as_ptr(),
                    per_instance_attribute_count: per_instance_attributes.len() as i32,
                    vertex_count: vertex_count as i32,
                    index_count: index_count as i32,
                    instance_count: instance_count as i32,
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

    pub fn vertex_buffer_data<'a, TVertex, TInstance>(
        &'a mut self,
        vertex_buffer_id: DivisionId,
    ) -> VertexBufferData<TVertex, TInstance> {
        unsafe {
            let mut borrowed = MaybeUninit::uninit();
            division_engine_vertex_buffer_borrow_data(
                &mut *self,
                vertex_buffer_id,
                borrowed.as_mut_ptr(),
            );

            let borrowed = borrowed.assume_init();
            let per_vert_ptr = borrowed.vertex_data_ptr as *mut TVertex;
            let per_inst_ptr = borrowed.instance_data_ptr as *mut TInstance;
            let index_ptr = borrowed.index_data_ptr as *mut u32;
            let vertex_count = borrowed.vertex_count as usize;
            let index_count = borrowed.index_count as usize;
            let instance_count = borrowed.instance_count as usize;

            return VertexBufferData {
                ctx: &mut *self,
                borrowed,
                vertex_buffer_id,
                per_vertex_data: std::slice::from_raw_parts_mut(
                    per_vert_ptr,
                    vertex_count,
                ),
                per_instance_data: std::slice::from_raw_parts_mut(
                    per_inst_ptr,
                    instance_count,
                ),
                vertex_indices: std::slice::from_raw_parts_mut(index_ptr, index_count),
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
