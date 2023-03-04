#include <stdio.h>

#include "context.h"
#include "renderer.h"

void error_callback(int error_code, const char *message);
void update_callback(DivisionEngineState state);

int main()
{
    DivisionEngineSettings settings = {
        .error_callback = error_callback,
        .window_title = "New window",
        .window_width = 512,
        .window_height = 512
    };

    DivisionContext* context = NULL;
    division_engine_context_create(&settings, &context);
    division_engine_renderer_run_loop(context, update_callback);
    division_engine_context_destroy(context);
}

void error_callback(int error_code, const char *message)
{
    fprintf(stderr, "Error code: %d, error message: %s", error_code,  message);
}
void update_callback(DivisionEngineState state)
{

}
