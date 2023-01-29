#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils

// #region: bevy_pbr::mesh_functions
// #import bevy_pbr::mesh_functions
// manually copied in here just to appease the vscode wgsl extension
fn mesh_position_world_to_clip(world_position: vec4<f32>) -> vec4<f32> {
    return view.view_proj * world_position;
}
// #endregion

struct CustomMaterialUniforms {
    triangle_max_side_len: f32,
    use_player_index_mask: u32,
    point_transform_matrix: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> uniforms: CustomMaterialUniforms;

@group(1) @binding(1)
var color: texture_2d<f32>;
@group(1) @binding(2)
var color_sampler: sampler;

@group(1) @binding(3)
var coordinates: texture_2d<f32>;
@group(1) @binding(4)
var player_index: texture_2d<u32>;

let WIDTH: f32 = 640.0;
let HEIGHT: f32 = 480.0;

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

    out.blend_color = textureLoad(color, vertex.vertex_pixel_coord, 0);
    let vertex_player_index = textureLoad(player_index, vertex.vertex_pixel_coord, 0);
    let kinect_position = textureLoad(coordinates, vertex.vertex_pixel_coord, 0);

    let c0 = textureLoad(coordinates, vertex.pixel_coord_0, 0);
    let c1 = textureLoad(coordinates, vertex.pixel_coord_1, 0);
    let c2 = textureLoad(coordinates, vertex.pixel_coord_2, 0);
    let pi0 = textureLoad(player_index, vertex.pixel_coord_0, 0);
    let pi1 = textureLoad(player_index, vertex.pixel_coord_1, 0);
    let pi2 = textureLoad(player_index, vertex.pixel_coord_2, 0);

    let world_position = uniforms.point_transform_matrix * kinect_position;
    out.clip_position = mesh_position_world_to_clip(world_position);
    out.uv = vertex.uv;

    let triangle_valid = is_triangle_valid(c0, c1, c2);
    let triangle_max_side_len = find_triangle_max_side_len(c0, c1, c2);
    var is_valid = true;
    is_valid = is_valid && triangle_valid;
    is_valid = is_valid && triangle_max_side_len < uniforms.triangle_max_side_len;
    if uniforms.use_player_index_mask > 0u {
        is_valid = is_valid && (pi0.x > 0u);
        is_valid = is_valid && (pi1.x > 0u);
        is_valid = is_valid && (pi2.x > 0u);
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
    // let color = textureSample(color, color_sampler, in.uv);
    let color = in.blend_color;
    if in.is_valid < 0.5 {
        discard;
    }
    return color;
}
