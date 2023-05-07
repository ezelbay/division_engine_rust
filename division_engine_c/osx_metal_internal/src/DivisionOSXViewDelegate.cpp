#include "DivisionOSXViewDelegate.h"
#include <cstdio>

#include <NSUtils.h>

#include "division_engine/render_pass.h"
#include "division_engine/renderer.h"
#include "division_engine/uniform_buffer.h"
#include "division_engine/vertex_buffer.h"
#include "osx_uniform_buffer.h"
#include "osx_shader_context.h"
#include "osx_vertex_buffer.h"

static char* readFromFile(const char* path);
static void fillFunctionAttributes(MTL::Function* func, DivisionMetalAttribute** attributes, int32_t* attributes_count);

DivisionOSXViewDelegate::DivisionOSXViewDelegate(
    MTL::Device* device, const DivisionSettings* settings, DivisionContext* context)
    : MTK::ViewDelegate(), _device(device), _commandQueue(device->newCommandQueue()),
      settings(settings), context(context)
{}

DivisionOSXViewDelegate::~DivisionOSXViewDelegate()
{
    _commandQueue->release();
}

MTL::Buffer* DivisionOSXViewDelegate::createBuffer(size_t dataSize)
{
    MTL::Buffer* buff = _device->newBuffer(dataSize, MTL::ResourceStorageModeManaged);
    return buff;
}

void DivisionOSXViewDelegate::deleteBuffer(MTL::Buffer* buffer)
{
    buffer->release();
}

void DivisionOSXViewDelegate::drawInMTKView(MTK::View* pView)
{
    settings->update_callback(context);

    NS::AutoreleasePool* pool = NS::AutoreleasePool::alloc()->init();

    auto clearColor = context->renderer_context->clear_color;
    pView->setClearColor(MTL::ClearColor::Make(clearColor.r, clearColor.g, clearColor.b, clearColor.a));

    MTL::CommandBuffer* cmdBuffer = _commandQueue->commandBuffer();
    MTL::RenderPassDescriptor* renderPassDesc = pView->currentRenderPassDescriptor();
    MTL::RenderCommandEncoder* renderEnc = cmdBuffer->renderCommandEncoder(renderPassDesc);

    DivisionRenderPassSystemContext* render_pass_ctx = context->render_pass_context;
    DivisionVertexBufferSystemContext* vert_buff_ctx = context->vertex_buffer_context;
    DivisionUniformBufferSystemContext* uniform_buff_ctx = context->uniform_buffer_context;
    DivisionShaderSystemContext* shader_ctx = context->shader_context;

    for (int32_t i = 0; i < render_pass_ctx->render_pass_count; i++)
    {
        DivisionRenderPass* pass = &render_pass_ctx->render_passes[i];
        MTL::RenderPipelineState* pipelineState = shader_ctx->shader_programs[pass->shader_program].pipeline_state;
        MTL::Buffer* mtlBuffer = vert_buff_ctx->buffers_impl[pass->vertex_buffer].metal_buffer;

        renderEnc->setRenderPipelineState(pipelineState);
        renderEnc->setVertexBuffer(mtlBuffer, 0, 0);
        
        for (int ubIdx = 0; ubIdx < pass->uniform_buffer_count; ubIdx++)
        {
            int32_t buff_id = pass->uniform_buffers[ubIdx];
            MTL::Buffer* uniformMtlBuffer = uniform_buff_ctx->uniform_buffers_impl[buff_id].mtl_buffer;
            DivisionUniformBuffer buffDesc = uniform_buff_ctx->uniform_buffers[buff_id];
            switch (buffDesc.shaderType)
            {
                case DIVISION_SHADER_VERTEX:
                    renderEnc->setVertexBuffer(uniformMtlBuffer, 0, buffDesc.binding);
                    break;
                case DIVISION_SHADER_FRAGMENT:
                    renderEnc->setFragmentBuffer(uniformMtlBuffer, 0, buffDesc.binding);
                    break;
                default:
                    fprintf(stderr, "Unknown shader type in the pass");
                    break;
            }
        }

        renderEnc->drawPrimitives(
            MTL::PrimitiveType::PrimitiveTypeTriangle,
            NS::UInteger(pass->first_vertex),
            NS::UInteger(pass->vertex_count)
        );
    }

    renderEnc->endEncoding();
    cmdBuffer->presentDrawable(pView->currentDrawable());
    cmdBuffer->commit();

    pool->release();
}

void DivisionOSXViewDelegate::drawableSizeWillChange(MTK::View* pView, CGSize size)
{

}

bool DivisionOSXViewDelegate::createShaderProgram(
    const DivisionShaderSettings* shaderSettings, int32_t source_count, DivisionMetalShaderProgram* out_program)
{
    NS::Error* err = nullptr;
    out_program->attributes = nullptr;

    auto* pipeline_descriptor = MTL::RenderPipelineDescriptor::alloc()->init();

    for (int32_t i = 0; i < source_count; i++)
    {
        DivisionShaderSettings shader = shaderSettings[i];

        char* shaderSrc = readFromFile(shader.file_path);
        MTL::Library* library = _device->newLibrary(NSUtils::createUtf8String(shaderSrc), nullptr, &err);
        free(static_cast<void*>(shaderSrc));

        if (err)
        {
            fprintf(stderr, "%s", err->debugDescription()->utf8String());
            continue;
        }

        if (!library)
        {
            fprintf(stderr, "Created shader library is null");
            continue;
        }

        auto* func = library->newFunction(NSUtils::createUtf8String(shader.entry_point_name));

        switch (shader.type)
        {
            case DIVISION_SHADER_VERTEX:
                pipeline_descriptor->setVertexFunction(func);
                out_program->vertex_function = func;
                break;
            case DIVISION_SHADER_FRAGMENT:
                pipeline_descriptor->setFragmentFunction(func);
                out_program->fragment_function = func;
                break;
            default:
                fprintf(stderr, "Unknown shader function type `%d`", shader.type);
                break;
        }

        library->release();
        fillFunctionAttributes(func, &out_program->attributes, &out_program->attribute_count);
    }
    pipeline_descriptor->colorAttachments()->object(0)->setPixelFormat(MTL::PixelFormat::PixelFormatBGRA8Unorm_sRGB);

    err = nullptr;
    auto* renderPipelineState = _device->newRenderPipelineState(pipeline_descriptor, &err);
    if (err)
    {
        fprintf(stderr, "%s", err->debugDescription()->utf8String());
        return false;
    }

    if (!renderPipelineState)
    {
        fprintf(stderr, "Render pipeline state is null");
        return false;
    }

    pipeline_descriptor->release();

    out_program->pipeline_state = renderPipelineState;

    return out_program;
}

void fillFunctionAttributes(MTL::Function* func, DivisionMetalAttribute** attributes, int32_t* attributes_count)
{
    int32_t origin_attr_count = *attributes_count;
    NS::Array* attrs = func->stageInputAttributes();
    *attributes_count = origin_attr_count + static_cast<int32_t>(attrs->count());
    *attributes = static_cast<DivisionMetalAttribute*>(realloc(
        *attributes,
        sizeof(DivisionMetalAttribute) * (*attributes_count)
    ));

    for (int32_t i = 0; i < attrs->count(); i++)
    {
        auto* at = attrs->object<MTL::Attribute>(i);
        const char* at_name = at->name()->utf8String();

        DivisionMetalAttribute* attr_info = &(*attributes[origin_attr_count + i]);
        attr_info->index = static_cast<uint32_t>(at->attributeIndex());
        attr_info->name = static_cast<char*>(malloc((sizeof(char) * strlen(at_name) + 1)));
        strcpy(attr_info->name, at_name);
    }
}

void DivisionOSXViewDelegate::deleteShaderProgram(DivisionMetalShaderProgram* program)
{
    program->pipeline_state->release();
    program->fragment_function->release();
    program->vertex_function->release();

    for (int32_t i = 0; i < program->attribute_count; i++)
    {
        free(static_cast<void*>(program->attributes[i].name));
    }
}

char* readFromFile(const char* path)
{
    FILE* srcFile = std::fopen(path, "rt");
    if (!srcFile)
    {
        fprintf(stderr, "Cannot open file: %s\n Current dir: %s\n", path, getenv("PWD"));
        return nullptr;
    }

    std::fseek(srcFile, 0, SEEK_END);
    size_t fileSize = std::ftell(srcFile);
    std::fseek(srcFile, 0, SEEK_SET);

    char* shaderSrc = static_cast<char*>(malloc(sizeof(char) * fileSize));
    size_t readSize = fread(shaderSrc, sizeof(char), fileSize, srcFile);
    std::fclose(srcFile);

    if (readSize != fileSize)
    {
        fprintf(stderr, "Error while reading the file by path: %s\n", path);
        return nullptr;
    }

    return shaderSrc;
}