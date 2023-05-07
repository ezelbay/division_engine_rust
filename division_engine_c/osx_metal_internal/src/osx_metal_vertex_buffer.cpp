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

void division_engine_internal_platform_vertex_buffer_context_free(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vert_buffer_ctx = ctx->vertex_buffer_context;
    DivisionOSXWindowContext* window_ctx = static_cast<DivisionOSXWindowContext*>(ctx->renderer_context->window_data);
    DivisionOSXViewDelegate* view_delegate = window_ctx->app_delegate->viewDelegate;

    for (int i = 0; i < vert_buffer_ctx->buffers_count; i++)
    {
        DivisionVertexBufferInternalPlatform_* osx_vert_buffer = &vert_buffer_ctx->buffers_impl[i];
        view_delegate->deleteBuffer(osx_vert_buffer->mtl_buffer);
        view_delegate->deleteVertexDescriptor(osx_vert_buffer->mtl_vertex_descriptor);
    }

    free(vert_buffer_ctx->buffers_impl);
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

    DivisionVertexBuffer* vert_buffer = &vert_buffer_ctx->buffers[buff_id];
    DivisionVertexBufferInternalPlatform_* impl_buffer = &vert_buffer_ctx->buffers_impl[buff_id];

    DivisionOSXViewDelegate* view_delegate = window_context->app_delegate->viewDelegate;
    MTL::Buffer* buffer = view_delegate->createBuffer(
        vert_buffer->per_vertex_data_size * vert_buffer->vertex_count);

    MTL::VertexDescriptor* vertex_descriptor = view_delegate->createVertexDescriptor(vert_buffer);

    impl_buffer->mtl_buffer = buffer;
    impl_buffer->mtl_vertex_descriptor = vertex_descriptor;
}

void* division_engine_internal_platform_vertex_buffer_borrow_data_pointer(DivisionContext* ctx, int32_t vertex_buffer)
{
    return ctx->vertex_buffer_context->buffers_impl[vertex_buffer].mtl_buffer->contents();
}

void division_engine_internal_platform_vertex_buffer_return_data_pointer(
    DivisionContext* ctx, int32_t vertex_buffer, void* data_pointer)
{
    MTL::Buffer* mtl_buffer = ctx->vertex_buffer_context->buffers_impl[vertex_buffer].mtl_buffer;
    mtl_buffer->didModifyRange(NS::Range::Make(0, mtl_buffer->length()));
}
