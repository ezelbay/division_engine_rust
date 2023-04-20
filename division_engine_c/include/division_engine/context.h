#pragma once

#include <stdbool.h>

#include "color.h"
#include "settings.h"
#include "state.h"

#include <division_engine_c_export.h>

struct DivisionRendererSystemContext;
struct DivisionShaderSystemContext;
struct DivisionVertexBufferSystemContext;

#ifdef __cpluspus
extern "C" {
#endif

typedef struct DivisionContext {
    DivisionState state;

    DivisionErrorFunc error_callback;
    struct DivisionRendererSystemContext* renderer_context;
    struct DivisionShaderSystemContext* shader_context;
    struct DivisionVertexBufferSystemContext* vertex_buffer_context;

    void* user_data;
} DivisionContext;

DIVISION_EXPORT bool division_engine_context_alloc(const DivisionSettings* settings, DivisionContext** output_context);
DIVISION_EXPORT void division_engine_context_free(DivisionContext* ctx);

#ifdef __cpluspus
}
#endif