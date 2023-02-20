use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
    primitives::Aabb,
    render_resource::{self, AsBindGroup, PrimitiveTopology, ShaderRef, ShaderType, TextureFormat, VertexFormat},
};
use bevy_reflect::TypeUuid;
use bytemuck::checked::{cast_slice, cast_slice_mut};
use itertools::Itertools;

use crate::{
    app_settings::AppSettings,
    receiver::{KinectDepthTransformer, KinectFrameBuffers, KinectFrameDataDelayBufferV2},
    COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH,
};

pub struct DepthModel2Plugin;
impl Plugin for DepthModel2Plugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(MaterialPlugin::<DepthModelMaterial>::default())
            // .add_startup_system(spawn_depth_texture)
            // .add_system(update_depth_texture);
            .add_startup_system(spawn_depth_model)
            .add_system(update_depth_model)
            .register_type::<DepthModelMaterial>();
    }
}

#[derive(Component, Default, Debug, Reflect)]
pub struct DepthModelMarker;

fn spawn_depth_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DepthModelMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let rgba_handle = images.add(Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
    ));
    let coordinates = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        cast_slice(&[0.0, 0.0, 0.0, 0.0]),
        bevy::render::render_resource::TextureFormat::Rgba32Float,
    );
    let player_indexes = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0],
        bevy::render::render_resource::TextureFormat::R8Uint,
    );
    let quad_handle = meshes.add(make_subdivided_quad(DEPTH_WIDTH, DEPTH_HEIGHT));

    let uniforms = DepthModelUniforms {
        triangle_max_side_len: 0.3,
        point_transform_matrix: Mat4::IDENTITY,
        use_player_index_mask: 0,
        do_lookback: 1,
        do_lookahead: 1,
    };
    let material_handle = materials.add(DepthModelMaterial {
        uniforms,
        color: rgba_handle,
        coordinates: images.add(coordinates.clone()),
        player_index: images.add(player_indexes.clone()),
        prev_coordinates: images.add(coordinates.clone()),
        prev_player_index: images.add(player_indexes.clone()),
        next_coordinates: images.add(coordinates.clone()),
        next_player_index: images.add(player_indexes.clone()),
    });

    commands.spawn((
        Name::new("depth_model"),
        DepthModelMarker,
        // PbrBundle {
        MaterialMeshBundle {
            mesh: quad_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        // just make it really big so it always renders
        Aabb::from_min_max(Vec3::splat(-4.0), Vec3::splat(4.0)),
    ));
}

fn update_depth_model(
    buffers: Res<KinectFrameBuffers>,
    depth_transformer: Res<KinectDepthTransformer>,
    frame_delay_buffer: Res<KinectFrameDataDelayBufferV2>,
    settings: Res<AppSettings>,
    mut depth_texture: Query<(&DepthModelMarker, &Handle<DepthModelMaterial>)>,
    mut materials: ResMut<Assets<DepthModelMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    if !buffers.is_changed() {
        return;
    }
    let Some(next_frame) = frame_delay_buffer.0.front() else {
        return;
    };
    let rgba = &buffers.rgba;

    let (_marker, material_handle) = depth_texture.single_mut();
    let Some(material) = materials.get_mut(&material_handle) else {
        info!("material not found");
        return;
    };

    let color_handle = &mut material.color;
    let color_image = images.get_mut(&color_handle).unwrap();
    cast_slice_mut::<_, [u8; 4]>(&mut color_image.data).copy_from_slice(&rgba);

    // cycle the handles to advance next to current and current to prev (and re-use prev as the new next)
    std::mem::swap(&mut material.prev_player_index, &mut material.next_player_index);
    std::mem::swap(&mut material.prev_player_index, &mut material.player_index);
    std::mem::swap(&mut material.prev_coordinates, &mut material.next_coordinates);
    std::mem::swap(&mut material.prev_coordinates, &mut material.coordinates);

    // so that we can get by with populating only the next frame, not the current one
    let skeleton_points = &next_frame.skeleton_points;
    let player_indexes = &next_frame.player_index;
    let coordinates_handle = &material.next_coordinates;
    let player_index_handle = &material.next_player_index;

    if let Some(coordinates) = images.get_mut(&coordinates_handle) {
        let coordinates_data = cast_slice_mut::<_, Vec4>(&mut coordinates.data);
        for (coordinate, &sk_point) in coordinates_data.iter_mut().zip(skeleton_points.iter()) {
            *coordinate = if sk_point == Vec3::ZERO {
                Vec4::ZERO
            } else {
                sk_point.extend(1.0)
            };
        }
    } else {
        info!("coordinates not found");
    }

    let player_index_image = images.get_mut(&player_index_handle).unwrap();
    player_index_image.data.copy_from_slice(&player_indexes);

    // if there's any player index detected, then tell the shader to use the player index
    let any_nonzero_player_index = player_indexes.iter().any(|player_index| *player_index > 0);
    material.uniforms.use_player_index_mask =
        (settings.depth_texture_always_use_player_index || any_nonzero_player_index) as u32;
    material.uniforms.point_transform_matrix = depth_transformer.point_transform_matrix.into();
    material.uniforms.do_lookback = settings.depth_texture_do_lookback as u32;
    material.uniforms.do_lookahead = settings.depth_texture_do_lookahead as u32;
}

#[derive(ShaderType, Debug, Clone, Reflect)]
#[repr(C)]
pub struct DepthModelUniforms {
    // NOTE: this is in meters! (and also note that the kinect depth sensor has a lot of noise lol)
    triangle_max_side_len: f32,
    use_player_index_mask: u32,
    point_transform_matrix: Mat4,
    do_lookback: u32,
    do_lookahead: u32,
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid, Reflect)]
#[uuid = "b62bb455-a72c-4b56-87bb-81e0554e234f"]
pub struct DepthModelMaterial {
    #[uniform(0)]
    uniforms: DepthModelUniforms,

    #[texture(1)]
    #[sampler(2)]
    color: Handle<Image>,

    #[texture(3)]
    coordinates: Handle<Image>,
    #[texture(4, sample_type = "u_int")]
    player_index: Handle<Image>,

    // textures for lookback/lookahead
    #[texture(5)]
    prev_coordinates: Handle<Image>,
    #[texture(6, sample_type = "u_int")]
    prev_player_index: Handle<Image>,
    #[texture(7)]
    next_coordinates: Handle<Image>,
    #[texture(8, sample_type = "u_int")]
    next_player_index: Handle<Image>,
}

impl Material for DepthModelMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material_depth_model.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_depth_model.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
        // AlphaMode::Mask(1.0)
    }
    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        // descriptor.primitive.polygon_mode = render_resource::PolygonMode::Point;
        // descriptor.primitive.polygon_mode = PolygonMode::Line;
        descriptor.primitive.cull_mode = None;
        // descriptor.primitive.unclipped_depth = true;
        descriptor.primitive.conservative = true;
        descriptor.depth_stencil = Some(render_resource::DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: render_resource::CompareFunction::Greater,
            stencil: render_resource::StencilState {
                front: render_resource::StencilFaceState::IGNORE,
                back: render_resource::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: render_resource::DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
        });

        Ok(())
    }
}

fn make_subdivided_quad(width: usize, height: usize) -> Mesh {
    let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);
    let xy_to_index = |x, y| (x + y * width) as u32;
    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            // first triangle, upper half
            indices.push(xy_to_index(x, y));
            indices.push(xy_to_index(x + 1, y));
            indices.push(xy_to_index(x + 1, y + 1));
            // second triangle, lower half
            indices.push(xy_to_index(x, y));
            indices.push(xy_to_index(x + 1, y + 1));
            indices.push(xy_to_index(x, y + 1));
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
