#include <MetalKit/MetalKit.hpp>

#include "division_engine/platform_internal/platform_vertex_buffer.h"
#include "division_engine/vertex_buffer.h"
#include "division_engine/renderer.h"
#include "osx_window_context.h"
#include "osx_vertex_buffer.h"

bool division_engine_internal_platform_vertex_buffer_context_alloc(
    DivisionContext* ctx, const DivisionSettings* settings)
{
    ctx->vertex_buffer_context->buffers_impl = nullptr;
    return true;
}

void division_engine_internal_platform_vertex_buffer_alloc(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vert_buffer_ctx = ctx->vertex_buffer_context;
    auto* window_context = static_cast<DivisionOSXWindowContext*>(ctx->renderer_context->window_data);

    size_t buff_id = vert_buffer_ctx->buffers_count - 1;
    vert_buffer_ctx->buffers_impl = static_cast<DivisionVertexBufferInternalPlatform_*>(realloc(
        vert_buffer_ctx->buffers_impl,
        sizeof(DivisionVertexBufferInternalPlatform_) * vert_buffer_ctx->buffers_count
    ));

    DivisionVertexBuffer* d_buffer = &vert_buffer_ctx->buffers[buff_id];
    DivisionVertexBufferInternalPlatform_* impl_buffer = &vert_buffer_ctx->buffers_impl[buff_id];

    MTL::Buffer* buffer = window_context->app_delegate->viewDelegate->createBuffer(
        d_buffer->per_vertex_data_size * d_buffer->vertex_count);
    impl_buffer->metal_buffer = buffer;
}

void division_engine_internal_platform_vertex_buffer_context_free(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vert_buffer_ctx = ctx->vertex_buffer_context;
    auto* window_ctx = static_cast<DivisionOSXWindowContext*>(ctx->renderer_context->window_data);
    for (int i = 0; i < vert_buffer_ctx->buffers_count; i++)
    {
        window_ctx->app_delegate->viewDelegate->deleteBuffer(vert_buffer_ctx->buffers_impl[i].metal_buffer);
    }
    free(vert_buffer_ctx->buffers_impl);
}

void division_engine_internal_platform_vertex_buffer_set_vertex_data(
    DivisionContext* ctx,
    int32_t vertex_buffer,
    int32_t object_index,
    int32_t attribute_index,
    const void* data_ptr,
    size_t first_vertex_index,
    size_t vertex_count
)
{
    DivisionVertexBufferSystemContext* vertex_buffer_context = ctx->vertex_buffer_context;
    DivisionVertexBuffer vb = vertex_buffer_context->buffers[vertex_buffer];
    MTL::Buffer* metal_buffer = vertex_buffer_context->buffers_impl[vertex_buffer].metal_buffer;
    DivisionVertexBufferObjects vb_objs = vertex_buffer_context->buffers_objects[vertex_buffer];

    size_t obj_first_index = vb_objs.objects_start_vertex[object_index] + first_vertex_index;
    DivisionVertexAttribute attr = vb.attributes[attribute_index];
    int32_t attr_size = attr.base_size * attr.component_count;

    void* buffer_data_ptr = metal_buffer->contents();

    for (size_t i = 0; i < vertex_count; i++)
    {
        size_t src_vi = first_vertex_index + i;
        size_t obj_vi = obj_first_index + i;

        const void* src_data_ptr = static_cast<const int8_t*>(data_ptr) + attr_size * src_vi;

        size_t dst_offset = vb.per_vertex_data_size * obj_vi + attr.offset;
        void* dst_data_ptr = static_cast<int8_t*>(buffer_data_ptr) + dst_offset;
        memcpy(dst_data_ptr, src_data_ptr, attr_size);

        metal_buffer->didModifyRange(NS::Range::Make(dst_offset, attr_size));
    }
}

void* division_engine_internal_platform_vertex_buffer_borrow_data_pointer(DivisionContext* ctx, int32_t vertex_buffer)
{
    return ctx->vertex_buffer_context->buffers_impl[vertex_buffer].metal_buffer->contents();
}

void division_engine_internal_platform_vertex_buffer_return_data_pointer(
    DivisionContext* ctx, int32_t vertex_buffer, void* data_pointer)
{
    MTL::Buffer* mtl_buffer = ctx->vertex_buffer_context->buffers_impl[vertex_buffer].metal_buffer;
    mtl_buffer->didModifyRange(NS::Range::Make(0, mtl_buffer->length()));
}

// TODO: This is works only for GLFW. Remove it
void division_engine_internal_platform_vertex_buffer_draw(DivisionContext* ctx)
{

}
