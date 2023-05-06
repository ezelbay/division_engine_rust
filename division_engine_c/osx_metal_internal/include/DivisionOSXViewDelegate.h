#pragma once

#include <MetalKit/MetalKit.hpp>

#include "division_engine/context.h"
#include "division_engine/settings.h"
#include "division_engine/shader.h"
#import "osx_shader_context.h"

class DivisionOSXViewDelegate : public MTK::ViewDelegate {
public:
    DivisionOSXViewDelegate(MTL::Device* device, const DivisionSettings* settings, DivisionContext* context);
    ~DivisionOSXViewDelegate() override;

    void drawInMTKView(MTK::View* pView) override;
    void drawableSizeWillChange(MTK::View* pView, CGSize size) override;

    const DivisionSettings* settings;
    DivisionContext* context;

    MTL::Buffer* createBuffer(size_t dataSize);
    void deleteBuffer(MTL::Buffer* buffer);

    bool createShaderProgram(
        const DivisionShaderSettings* shaderSettings, int32_t source_count, DivisionMetalShaderProgram* out_program);
    void deleteShaderProgram(DivisionMetalShaderProgram* program);

private:
    MTL::Device* _device;
    MTL::CommandQueue* _commandQueue;
};
