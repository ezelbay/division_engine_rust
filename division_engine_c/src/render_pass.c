#include <division_engine/render_pass.h>

#include <glad/gl.h>
#include <stdio.h>
#include "division_engine/vertex_buffer.h"

bool division_engine_internal_render_pass_context_alloc(DivisionContext* ctx)
{
    ctx->render_pass_context = malloc(sizeof(DivisionVertexBufferSystemContext));
    *ctx->render_pass_context = (DivisionRenderPassSystemContext) {
        .render_passes = NULL,
        .render_pass_count = 0
    };

    return true;
}

void division_engine_internal_render_pass_context_free(DivisionContext* ctx)
{
    free(ctx->render_pass_context->render_passes);
    free(ctx->render_pass_context);
}

int32_t division_engine_render_pass_alloc(DivisionContext* ctx, DivisionRenderPass render_pass)
{
    DivisionRenderPassSystemContext* pass_ctx = ctx->render_pass_context;
    pass_ctx->render_passes = realloc(
        pass_ctx->render_passes, sizeof(DivisionRenderPass) * (pass_ctx->render_pass_count + 1));
    pass_ctx->render_passes[pass_ctx->render_pass_count++] = render_pass;

    return pass_ctx->render_pass_count - 1;
}
