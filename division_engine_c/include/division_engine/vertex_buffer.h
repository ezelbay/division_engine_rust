#pragma once

#include <stdlib.h>
#include <stdbool.h>

#include "context.h"
#include "vertex_attribute.h"

bool division_engine_internal_vertex_buffer_create_context(DivisionContext* ctx);
void division_engine_internal_vertex_buffer_destroy_context(DivisionContext* ctx);