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

fn is_coordinate_valid(coordinate: vec4<f32>) -> bool {
    return coordinate.x != 0.0 || coordinate.y != 0.0 || coordinate.z != 0.0 || coordinate.w != 0.0;
}

fn is_pixel_coords_valid(pixel_coords: vec2<f32>) -> bool {
    let px = pixel_coords.x;
    let py = pixel_coords.y;
    let px0 = i32(floor(px));
    let py0 = i32(floor(py));
    let px1 = i32(ceil(px));
    let py1 = i32(ceil(py));
    let p0 = vec2<i32>(px0, py0);
    let p1 = vec2<i32>(px1, py0);
    let p2 = vec2<i32>(px1, py1);
    let p3 = vec2<i32>(px0, py1);
    var c0 = textureLoad(coordinates, p0, 0);
    var c1 = textureLoad(coordinates, p1, 0);
    var c2 = textureLoad(coordinates, p2, 0);
    var c3 = textureLoad(coordinates, p3, 0);
    return is_coordinate_valid(c0) && is_coordinate_valid(c1) && is_coordinate_valid(c2) && is_coordinate_valid(c3);
}

fn max_adjacent_dist(pixel_coords: vec2<f32>) -> f32 {
    let px = pixel_coords.x;
    let py = pixel_coords.y;
    let px0 = i32(floor(px));
    let py0 = i32(floor(py));
    let px1 = i32(ceil(px));
    let py1 = i32(ceil(py));
    let p0 = vec2<i32>(px0, py0);
    let p1 = vec2<i32>(px1, py0);
    let p2 = vec2<i32>(px1, py1);
    let p3 = vec2<i32>(px0, py1);
    let c0 = textureLoad(coordinates, p0, 0);
    let c1 = textureLoad(coordinates, p1, 0);
    let c2 = textureLoad(coordinates, p2, 0);
    let c3 = textureLoad(coordinates, p3, 0);

    var dist = 0.0;
    dist = max(dist, distance(c0, c1));
    dist = max(dist, distance(c0, c2));
    dist = max(dist, distance(c0, c3));
    dist = max(dist, distance(c1, c2));
    dist = max(dist, distance(c1, c3));
    dist = max(dist, distance(c2, c3));
    return dist;
}

struct Vertex {
    @location(0) position: vec3<f32>,
    // @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) pixel_coords: vec2<f32>,
    // #import bevy_pbr::mesh_vertex_output
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var model = mesh.model;

    out.pixel_coords = round(vertex.uv * vec2<f32>(WIDTH, HEIGHT));
    out.world_position = point_transform_matrix * textureLoad(coordinates, vec2<i32>(out.pixel_coords), 0);
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture, texture_sampler, in.uv);

    let valid = is_pixel_coords_valid(in.pixel_coords);
    let adj_dist = max_adjacent_dist(in.pixel_coords);

    if !valid {
        discard;
    }
    if adj_dist > 0.1 {
        discard;
    }

    return color;
}
