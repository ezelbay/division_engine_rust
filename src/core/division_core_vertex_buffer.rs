use std::ffi::c_void;

use super::{
    interface::{
        context::DivisionContext,
        vertex_buffer::{
            division_engine_vertex_buffer_alloc, division_engine_vertex_buffer_borrow_data_pointer,
            division_engine_vertex_buffer_free, division_engine_vertex_buffer_return_data_pointer,
            DivisionVertexBufferDataBorrowInfo, VertexBufferDescriptor,
        },
    },
    DivisionCore, DivisionError, DivisionId,
};

pub use super::interface::vertex_buffer::{RenderTopology, VertexAttributeDescriptor};

pub struct VertexBufferData<'a, TX, TY> {
    ctx: *mut DivisionContext,
    ptr: *mut c_void,
    vertex_buffer_id: u32,
    pub per_vertex_data: &'a mut [TX],
    pub per_instance_data: &'a mut [TY],
}

impl DivisionCore {
    pub fn create_vertex_buffer(
        &mut self,
        per_vertex_attributes: &[VertexAttributeDescriptor],
        per_instance_attributes: &[VertexAttributeDescriptor],
        vertex_count: usize,
        instance_count: usize,
        topology: RenderTopology,
    ) -> Result<DivisionId, DivisionError> {
        let mut id = 0;

        unsafe {
            if !division_engine_vertex_buffer_alloc(
                self.ctx,
                &VertexBufferDescriptor {
                    per_vertex_attributes: per_vertex_attributes.as_ptr(),
                    per_vertex_attribute_count: per_vertex_attributes.len() as i32,
                    per_instance_attributes: per_instance_attributes.as_ptr(),
                    per_instance_attribute_count: per_instance_attributes.len() as i32,
                    vertex_count: vertex_count as i32,
                    instance_count: instance_count as i32,
                    topology,
                },
                &mut id,
            ) {
                return Err(DivisionError::Core(String::from(
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
            let mut borrow_info = DivisionVertexBufferDataBorrowInfo {
                vertex_data_offset: 0,
                instance_data_offset: 0,
                vertex_count: 0,
                instance_count: 0,
            };
            let ptr = division_engine_vertex_buffer_borrow_data_pointer(
                self.ctx,
                vertex_buffer_id,
                &mut borrow_info,
            );

            let per_vert_ptr = ptr.add(borrow_info.vertex_data_offset as usize) as *mut TVertex;
            let per_inst_ptr = ptr.add(borrow_info.instance_data_offset as usize) as *mut TInstance;

            return VertexBufferData {
                ctx: self.ctx,
                ptr,
                vertex_buffer_id,
                per_vertex_data: std::slice::from_raw_parts_mut(
                    per_vert_ptr,
                    borrow_info.vertex_count as usize,
                ),
                per_instance_data: std::slice::from_raw_parts_mut(
                    per_inst_ptr,
                    borrow_info.instance_count as usize,
                ),
            };
        }
    }

    pub fn delete_vertex_buffer(&mut self, vertex_buffer_id: DivisionId) {
        unsafe {
            division_engine_vertex_buffer_free(self.ctx, vertex_buffer_id);
        }
    }
}

impl<'a, TX, TY> Drop for VertexBufferData<'a, TX, TY> {
    fn drop(&mut self) {
        unsafe {
            division_engine_vertex_buffer_return_data_pointer(
                self.ctx,
                self.vertex_buffer_id,
                self.ptr,
            )
        }
    }
}
