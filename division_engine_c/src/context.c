#include "division_engine/context.h"

#include <stdlib.h>
#include "division_engine/renderer.h"
#include "division_engine/vertex_buffer.h"

bool division_engine_context_alloc(const DivisionSettings* settings, DivisionContext** output_context)
{
    DivisionContext* ctx = (DivisionContext*) malloc(sizeof(DivisionContext));
    ctx->error_callback = settings->error_callback;
    ctx->state.delta_time = 0;
    *output_context = ctx;

    if (!division_engine_internal_renderer_context_alloc(ctx, settings)) return false;
    if (!division_engine_internal_vertex_buffer_context_alloc(ctx, settings)) return false;

    return true;
}

void division_engine_context_free(DivisionContext* ctx)
{
    division_engine_internal_vertex_buffer_context_free(ctx);
    division_engine_internal_renderer_context_free(ctx);
    free(ctx);
}
