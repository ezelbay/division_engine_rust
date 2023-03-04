#include "division_engine/context.h"

#include <stdlib.h>
#include "division_engine/renderer.h"
#include "division_engine/vertex_buffer.h"

bool division_engine_context_create(const DivisionEngineSettings* settings, DivisionContext** output_context)
{
    DivisionContext* ctx = (DivisionContext*) malloc(sizeof(DivisionContext));
    ctx->error_callback = settings->error_callback;
    *output_context = ctx;

    if (!division_engine_internal_renderer_create_context(ctx, settings)) return false;
    if (!division_engine_internal_vertex_buffer_create_context(ctx)) return false;

    return true;
}

void division_engine_context_destroy(DivisionContext* ctx)
{
    division_engine_internal_vertex_buffer_destroy_context(ctx);
    division_engine_internal_renderer_destroy_context(ctx);
    free(ctx);
}
