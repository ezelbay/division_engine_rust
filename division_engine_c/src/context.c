#include "context.h"

#include <stdlib.h>
#include "renderer.h"

bool division_engine_context_create(const DivisionEngineSettings* settings, DivisionContext** output_context) {
    DivisionContext* ctx = (DivisionContext*) malloc(sizeof(DivisionContext));
    ctx->error_callback = settings->error_callback;
    *output_context = ctx;

    division_engine_internal_renderer_create(&ctx->renderer_context, settings);
}

void division_engine_context_destroy(DivisionContext* context) {
    division_engine_renderer_destroy(&context->renderer_context);
    free(context);
}
