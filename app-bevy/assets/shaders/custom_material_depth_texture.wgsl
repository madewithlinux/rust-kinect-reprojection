#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils

#import bevy_pbr::mesh_functions

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@group(1) @binding(2)
var coordinates: texture_2d<f32>;
@group(1) @binding(3)
var player_index: texture_2d<f32>;

@group(1) @binding(4)
var<uniform> max_adj_dist: f32;
@group(1) @binding(5)
var<uniform> point_transform_matrix: mat4x4<f32>;
@group(1) @binding(6)
var<uniform> use_player_index_mask: u32;


let WIDTH: f32 = 640.0;
let HEIGHT: f32 = 480.0;
// note: this is in meters!
let MAX_TRIANGLE_MAX_SIDE_LEN: f32 = 0.3;

fn is_coordinate_valid(coordinate: vec4<f32>) -> bool {
    return coordinate.x != 0.0 || coordinate.y != 0.0 || coordinate.z != 0.0 || coordinate.w != 0.0;
}

fn is_triangle_valid(c0: vec4<f32>, c1: vec4<f32>, c2: vec4<f32>) -> bool {
    return is_coordinate_valid(c0) && is_coordinate_valid(c1) && is_coordinate_valid(c2);
}

fn find_triangle_max_side_len(c0: vec4<f32>, c1: vec4<f32>, c2: vec4<f32>) -> f32 {
    var dist = 0.0;
    dist = max(dist, distance(c0.xyz, c1.xyz));
    dist = max(dist, distance(c0.xyz, c2.xyz));
    dist = max(dist, distance(c1.xyz, c2.xyz));
    return dist;
}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) vertex_pixel_coord: vec2<i32>,
    @location(3) pixel_coord_0: vec2<i32>,
    @location(4) pixel_coord_1: vec2<i32>,
    @location(5) pixel_coord_2: vec2<i32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
    @location(1) is_valid: f32,
    @location(2) uv: vec2<f32>,
};


@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32, vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var model = mesh.model;

    out.blend_color = textureLoad(texture, vertex.vertex_pixel_coord, 0);
    let world_position = point_transform_matrix * textureLoad(coordinates, vertex.vertex_pixel_coord, 0);
    out.clip_position = mesh_position_world_to_clip(world_position);
    out.uv = vertex.uv;

    let c0 = textureLoad(coordinates, vertex.pixel_coord_0, 0);
    let c1 = textureLoad(coordinates, vertex.pixel_coord_1, 0);
    let c2 = textureLoad(coordinates, vertex.pixel_coord_2, 0);
    let pi0 = textureLoad(player_index, vertex.pixel_coord_0, 0);
    let pi1 = textureLoad(player_index, vertex.pixel_coord_1, 0);
    let pi2 = textureLoad(player_index, vertex.pixel_coord_2, 0);

    let triangle_valid = is_triangle_valid(c0, c1, c2);
    let triangle_max_side_len = find_triangle_max_side_len(c0, c1, c2);
    var is_valid = true;
    is_valid = is_valid && triangle_valid;
    is_valid = is_valid && triangle_max_side_len < MAX_TRIANGLE_MAX_SIDE_LEN;
    if use_player_index_mask > 0u {
        is_valid = is_valid && is_triangle_valid(pi0, pi1, pi2);
    }
    if is_valid {
        out.is_valid = 1.0;
    } else {
        out.is_valid = 0.0;
    }
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // let color = textureSample(texture, texture_sampler, in.uv);
    let color = in.blend_color;
    if in.is_valid < 0.5 {
        discard;
    }
    return color;
}
