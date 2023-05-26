use super::{
    interface::{vertex_buffer::{
        division_engine_vertex_buffer_alloc, division_engine_vertex_buffer_free,
        VertexBufferDescriptor,
    }, context::DivisionContext},
    DivisionCore, DivisionError,
};

pub use super::interface::vertex_buffer::{RenderTopology, VertexAttributeDescriptor};

pub struct VertexBuffer {
    ctx: *mut DivisionContext,
    id: u32,
}

impl DivisionCore {
    pub fn create_vertex_buffer(
        &self,
        per_vertex_attributes: &[VertexAttributeDescriptor],
        per_instance_attributes: &[VertexAttributeDescriptor],
        vertex_count: usize,
        instance_count: usize,
        topology: RenderTopology,
    ) -> Result<VertexBuffer, DivisionError> {
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

        Ok(VertexBuffer { ctx: self.ctx, id })
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            division_engine_vertex_buffer_free(self.ctx, self.id);
        }
    }
}
