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
    do_lookback: u32,
    do_lookahead: u32,
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

@group(1) @binding(5)
var prev_coordinates: texture_2d<f32>;
@group(1) @binding(6)
var prev_player_index: texture_2d<u32>;

@group(1) @binding(7)
var next_coordinates: texture_2d<f32>;
@group(1) @binding(8)
var next_player_index: texture_2d<u32>;

let WIDTH: f32 = 640.0;
let HEIGHT: f32 = 480.0;

fn is_coordinate_valid(coordinate: vec4<f32>) -> bool {
    return coordinate.x != 0.0 || coordinate.y != 0.0 || coordinate.z != 0.0 || coordinate.w != 0.0;
}

struct PlayerIndexAndCoordinate {
    coordinate: vec4<f32>,
    player_index: u32,
};
fn load_player_index_coordinate_with_lookaround(pixel_coord: vec2<i32>) -> PlayerIndexAndCoordinate {
    // we only allow lookaround when it's for a vertex with a player index (when player index is used)
    let prev = textureLoad(prev_coordinates, pixel_coord, 0);
    let current = textureLoad(coordinates, pixel_coord, 0);
    let next = textureLoad(next_coordinates, pixel_coord, 0);
    let pi_prev = textureLoad(prev_player_index, pixel_coord, 0).x;
    let pi_current = textureLoad(player_index, pixel_coord, 0).x;
    let pi_next = textureLoad(next_player_index, pixel_coord, 0).x;

    if is_coordinate_valid(current) {
        return PlayerIndexAndCoordinate(current, pi_current);
    }
    if (uniforms.do_lookback > 0u) && is_coordinate_valid(prev) && (pi_prev > 0u || uniforms.use_player_index_mask == 0u) {
        return PlayerIndexAndCoordinate(prev, pi_prev);
    }
    if (uniforms.do_lookahead > 0u) && is_coordinate_valid(next) && (pi_next > 0u || uniforms.use_player_index_mask == 0u) {
        return PlayerIndexAndCoordinate(next, pi_next);
    }
    return PlayerIndexAndCoordinate(current, pi_current);
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

    let c_pi_0 = load_player_index_coordinate_with_lookaround(vertex.pixel_coord_0);
    let c_pi_1 = load_player_index_coordinate_with_lookaround(vertex.pixel_coord_1);
    let c_pi_2 = load_player_index_coordinate_with_lookaround(vertex.pixel_coord_2);
    let c0 = c_pi_0.coordinate;
    let c1 = c_pi_1.coordinate;
    let c2 = c_pi_2.coordinate;
    let pi0 = c_pi_0.player_index;
    let pi1 = c_pi_1.player_index;
    let pi2 = c_pi_2.player_index;
    let kinect_position = load_player_index_coordinate_with_lookaround(vertex.vertex_pixel_coord).coordinate;

    let world_position = uniforms.point_transform_matrix * kinect_position;
    out.clip_position = mesh_position_world_to_clip(world_position);
    out.uv = vertex.uv;

    let triangle_valid = is_triangle_valid(c0, c1, c2);
    let triangle_max_side_len = find_triangle_max_side_len(c0, c1, c2);
    var is_valid = true;
    is_valid = is_valid && triangle_valid;
    is_valid = is_valid && triangle_max_side_len < uniforms.triangle_max_side_len;
    if uniforms.use_player_index_mask > 0u {
        is_valid = is_valid && (pi0 > 0u);
        is_valid = is_valid && (pi1 > 0u);
        is_valid = is_valid && (pi2 > 0u);
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
    // // for debugging weird is_valid values:
    // if in.is_valid < 0.1 {
    //     discard;
    // }
    // if in.is_valid >= 0.1 && in.is_valid <= 0.9 {
    //     return vec4<f32>(1.0, 0.0, 0.0, 0.0);
    // }
    return color;
}
