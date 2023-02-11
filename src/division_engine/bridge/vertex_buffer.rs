use std::ffi::{c_long, c_ulong, c_void};

#[repr(C)]
pub enum AttributeType {
    Float,
    Double,
    Integer
}

#[repr(C)]
pub struct VertexAttribute {
    pub attribute_type: AttributeType,
    pub index: i32,
    pub offset: i32,
    pub stride: i32,
    pub size_of_components: i32,
    pub normalized: bool
}

extern "C" {
    pub fn division_engine_vertex_buffer_create(size: c_ulong) -> c_long;
    pub fn division_engine_vertex_buffer_define_attribute(buffer_id: c_long, attribute: VertexAttribute);
    pub fn division_engine_vertex_buffer_access_ptr_begin(buffer_id: c_long) -> *mut c_void;
    pub fn division_engine_vertex_buffer_access_ptr_end(buffer_id: c_long);
    pub fn division_engine_vertex_buffer_draw_triangles(buffer_id: c_long, first_index: c_ulong, count: c_ulong);
}