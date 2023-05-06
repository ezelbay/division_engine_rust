#include <metal_stdlib>
using namespace metal;

struct v2f {
    float4 position [[position]];
    half4 color;
};

#pragma pack(1)
struct vert {
    packed_float3 position;
    float4 color;
};


v2f vertex vertexMain(uint vertexId [[vertex_id]],
                      device const vert* vd [[buffer(0)]]
) {
    v2f o;
    o.position = float4(vd[vertexId].position, 1.0);
    o.color = half4(vd[vertexId].color);

    return o;
}

half4 fragment fragmentMain(v2f in [[stage_in]],
                            constant float4& testColor [[buffer(1)]]
)
{
    return in.color * (half4) testColor;
}
