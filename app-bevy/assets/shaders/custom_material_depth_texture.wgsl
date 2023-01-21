#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils

#import bevy_pbr::mesh_functions

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;
@group(1) @binding(2)
// var coordinates: texture_storage_2d<rgba32float, read>;
var coordinates: texture_2d<f32>;
// @group(1) @binding(3)
// var coordinates_sampler: sampler;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var model = mesh.model;
    // var position = textureSample(coordinates, coordinates_sampler, vertex.uv);
    // var position = textureLoad(coordinates, vertex.uv);
    var position = textureLoad(coordinates, vec2<i32>(vertex.uv * vec2<f32>(640.0, 480.0)), 0);

    // var position = textureSample(coordinates, coordinates_sampler, vec2<f32>(0.5, 0.5));
    // out.clip_position = mesh_position_local_to_clip(mesh.model, position);

    // out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));

    // out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    // out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.clip_position = mesh_position_world_to_clip(position);

    // var xy = vertex.uv*4.0;
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    // let uv = coords_to_viewport_uv(position.xy, view.viewport);
    // let color = textureSample(texture, texture_sampler, uv);
    // return color;
    // return textureSample(texture, texture_sampler, uv);
    let color = textureSample(texture, texture_sampler, uv);
    if color.a < 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    } else {
        return color;
    }
}
