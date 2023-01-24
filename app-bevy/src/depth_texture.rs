use bevy::prelude::*;
use bevy_reflect::TypeUuid;
use bevy_render::{
    mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
    primitives::Aabb,
    render_resource::{
        self, AsBindGroup, BlendState, ColorTargetState, ColorWrites, PolygonMode, PrimitiveTopology, ShaderRef,
        TextureFormat, TextureUsages, VertexFormat,
    },
};
use bytemuck::checked::{cast_slice, cast_slice_mut};
use itertools::Itertools;

use crate::{
    receiver::{KinectDepthTransformer, KinectFrameBuffers},
    COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH,
};

pub struct DepthTexturePlugin;
impl Plugin for DepthTexturePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugin(MaterialPlugin::<CustomMaterial>::default())
            // .add_startup_system(spawn_depth_texture)
            // .add_system(update_depth_texture);
            .add_startup_system(spawn_custom_depth_texture)
            .add_system(update_custom_depth_texture)
            .register_type::<CustomMaterial>();
    }
}

#[derive(Component, Default, Debug, Reflect)]
pub struct DepthTextureMarker;

fn spawn_depth_texture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let image_handle = images.add(Image::new_fill(
        bevy_render::render_resource::Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        bevy_render::render_resource::TextureFormat::Rgba8UnormSrgb,
    ));

    let quad_handle = meshes.add(make_subdivided_quad(DEPTH_WIDTH, DEPTH_HEIGHT, true));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        alpha_mode: AlphaMode::Mask(1.0),
        unlit: true,
        double_sided: true,
        ..default()
    });

    commands.spawn((
        Name::new("depth_texture"),
        DepthTextureMarker,
        PbrBundle {
            mesh: quad_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Aabb::default(),
    ));
}

fn make_subdivided_quad(width: usize, height: usize, add_normals: bool) -> Mesh {
    let mut positions = Vec::with_capacity(width * height);
    let mut normals = Vec::with_capacity(width * height);
    let mut uvs = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let xf = (x as f32) / ((width - 1) as f32);
            let yf = (y as f32) / ((height - 1) as f32);
            positions.push([xf, yf, 0.0]);
            normals.push([0.0, 0.0, 1.0]);
            uvs.push([xf, yf]);
        }
    }
    let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);
    let xy_to_index = |x, y| (x + y * width) as u32;
    for y in 1..height {
        for x in 1..width {
            // first triangle, upper half
            indices.push(xy_to_index(x, y));
            indices.push(xy_to_index(x - 1, y));
            indices.push(xy_to_index(x - 1, y - 1));
            // second triangle, lower half
            indices.push(xy_to_index(x, y));
            indices.push(xy_to_index(x - 1, y - 1));
            indices.push(xy_to_index(x, y - 1));
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    if add_normals {
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

fn update_depth_texture(
    buffers: Res<KinectFrameBuffers>,
    mut depth_texture: Query<(&DepthTextureMarker, &Handle<Mesh>, &Handle<StandardMaterial>, &mut Aabb)>,
    // mut depth_texture: Query<(&DepthTextureMarker, &Handle<Mesh>, &Handle<CustomMaterial>, &mut Aabb)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut materials: ResMut<Assets<CustomMaterial>>,
    mut images: ResMut<Assets<Image>>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    // if buffers.derived_frame.depth.len() == 0 {
    //     info!("depth frame empty");
    //     return;
    // }
    let rgba = &buffers.rgba;
    let skeleton_points = &buffers.skeleton_points;
    let depths = &buffers.depth;
    let player_indexes = &buffers.player_index;
    // TODO: refactor and put this somewhere else
    let depths = if player_indexes.iter().any(|player_index| *player_index > 0) {
        depths
            .iter()
            .zip(player_indexes.iter())
            .map(|(depth, player_index)| if *player_index > 0 { *depth } else { 0 })
            .collect_vec()
    } else {
        depths.clone()
    };

    let (_marker, mesh_handle, material_handle, mut aabb) = depth_texture.single_mut();
    let Some(material) = materials.get_mut(&material_handle) else {
        info!("material not found");
        return;
    };
    let Some(image_handle) = &material.base_color_texture else {
        info!("image handle not found");
        return;
    };
    // let image_handle = &material.texture;
    if let Some(image) = images.get_mut(image_handle) {
        // get_buffer(&FrameBufferDescriptor::CurrentColor, &buffers, &mut image.data);
        let image_data = cast_slice_mut::<_, [u8; 4]>(&mut image.data);
        for (flat_index, (&rgba, &depth)) in rgba.iter().zip(depths.iter()).enumerate() {
            image_data[flat_index] = if depth == 0 { [0, 0, 0, 0] } else { rgba };
        }
    } else {
        info!("image not found");
    }

    let Some(mesh) = meshes.get_mut(&mesh_handle) else {
        info!("mesh not found");
        return;
    };
    let Some(Indices::U32(indices)) = mesh.indices().map(|indices| indices.clone()) else {
        info!("mesh has bad indices");
        return;
    };

    let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        info!("position attribute missing from mesh");
        return;
    };
    for (flat_index, &sk_point) in skeleton_points.iter().enumerate() {
        if sk_point == Vec3::ZERO {
            // positions[flat_index] = depth_transformer.index_depth_to_world(flat_index, 4_000).to_array();
            // positions[flat_index] = Vec3::ZERO.to_array();
        } else {
            positions[flat_index] = depth_transformer.skeleton_point_to_world(sk_point).to_array();
        }
    }

    // // fixup dangling triangles to have all-or-none valid vertex positions
    // let indices = cast_slice::<_, [u32; 3]>(&indices);
    // for [i0, i1, i2] in indices.iter() {
    //     let i0 = *i0 as usize;
    //     let i1 = *i1 as usize;
    //     let i2 = *i2 as usize;
    //     for [j0, j1, j2] in [[i0, i1, i2], [i1, i2, i0], [i2, i0, i1]] {
    //         // if player_indexes[j0] > 0 && (player_indexes[j1] == 0 || player_indexes[j2] == 0) {
    //         if depths[j0] > 0 && (depths[j1] == 0 || depths[j2] == 0) {
    //             positions[j1] = positions[j0];
    //             positions[j2] = positions[j0];
    //             break;
    //         }
    //         // if skeleton_points[j0] != Vec3::ZERO
    //         //     && (skeleton_points[j1] == Vec3::ZERO || skeleton_points[j2] == Vec3::ZERO)
    //         // {
    //         //     positions[j1] = positions[j0];
    //         //     positions[j2] = positions[j0];
    //         //     break;
    //         // }
    //         // if positions[j0] != [0.0, 0.0, 0.0]
    //         //     && (positions[j1] == [0.0, 0.0, 0.0] || positions[j2] == [0.0, 0.0, 0.0])
    //         // {
    //         //     positions[j1] = positions[j0];
    //         //     positions[j2] = positions[j0];
    //         //     break;
    //         // }
    //     }
    // }
    // recomputing aabb fixes the issue where it disappeared when zoomed in
    if let Some(new_aabb) = mesh.compute_aabb() {
        *aabb = new_aabb;
    }
}

fn spawn_custom_depth_texture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let rgba_handle = images.add(Image::new_fill(
        bevy_render::render_resource::Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        bevy_render::render_resource::TextureFormat::Rgba8UnormSrgb,
    ));
    let coordinates_handle = images.add(Image::new_fill(
        bevy_render::render_resource::Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        cast_slice(&[0.0, 0.0, 0.0, 0.0]),
        bevy_render::render_resource::TextureFormat::Rgba32Float,
    ));
    let player_index_handle = images.add(Image::new_fill(
        bevy_render::render_resource::Extent3d {
            width: COLOR_WIDTH as u32,
            height: COLOR_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0],
        bevy_render::render_resource::TextureFormat::R8Unorm,
    ));

    let quad_handle = meshes.add(make_subdivided_quad_with_pixel_coords(DEPTH_WIDTH, DEPTH_HEIGHT));

    let material_handle = materials.add(CustomMaterial {
        texture: rgba_handle,
        coordinates: coordinates_handle,
        player_index: player_index_handle,
        max_adj_dist: 0.1,
        point_transform_matrix: Mat4::IDENTITY,
        use_player_index_mask: 0,
    });

    commands.spawn((
        Name::new("depth_texture"),
        DepthTextureMarker,
        // PbrBundle {
        MaterialMeshBundle {
            mesh: quad_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Aabb::default(),
    ));
}

fn update_custom_depth_texture(
    buffers: Res<KinectFrameBuffers>,
    // mut depth_texture: Query<(&DepthTextureMarker, &Handle<Mesh>, &Handle<CustomMaterial>, &mut Aabb)>,
    mut depth_texture: Query<(&DepthTextureMarker, &Handle<CustomMaterial>)>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    if !buffers.is_changed() {
        return;
    }
    // if buffers.derived_frame.depth.len() == 0 {
    //     info!("depth frame empty");
    //     return;
    // }
    let rgba = &buffers.rgba;
    let skeleton_points = &buffers.skeleton_points;
    let depths = &buffers.depth;
    let player_indexes = &buffers.player_index;
    // TODO: refactor and put this somewhere else
    let depths = if player_indexes.iter().any(|player_index| *player_index > 0) {
        depths
            .iter()
            .zip(player_indexes.iter())
            .map(|(depth, player_index)| if *player_index > 0 { *depth } else { 0 })
            .collect_vec()
    } else {
        depths.clone()
    };

    let (_marker, material_handle) = depth_texture.single_mut();
    let Some(material) = materials.get_mut(&material_handle) else {
        info!("material not found");
        return;
    };
    let image_handle = &material.texture;
    if let Some(image) = images.get_mut(image_handle) {
        // get_buffer(&FrameBufferDescriptor::CurrentColor, &buffers, &mut image.data);
        let image_data = cast_slice_mut::<_, [u8; 4]>(&mut image.data);
        for (flat_index, (&rgba, &depth)) in rgba.iter().zip(depths.iter()).enumerate() {
            image_data[flat_index] = if depth == 0 { [0, 0, 0, 0] } else { rgba };
        }
    } else {
        info!("image not found");
    }

    let coordinates_handle = &material.coordinates;
    if let Some(coordinates) = images.get_mut(coordinates_handle) {
        let coordinates_data = cast_slice_mut::<_, [f32; 4]>(&mut coordinates.data);
        for (flat_index, &sk_point) in skeleton_points.iter().enumerate() {
            coordinates_data[flat_index] = if sk_point == Vec3::ZERO {
                // sk_point.extend(0.0).to_array()
                Vec4::ZERO.to_array()
            } else {
                sk_point.extend(1.0).to_array()
            };
            // if sk_point == Vec3::ZERO {
            //     // coordinates_data[flat_index] = depth_transformer
            //     //     .index_depth_to_world(flat_index, 4_000)
            //     //     .extend(0.0)
            //     //     .to_array();
            //     coordinates_data[flat_index] = Vec4::ZERO.to_array();
            // } else {
            //     coordinates_data[flat_index] = depth_transformer
            //         .skeleton_point_to_world(sk_point)
            //         .extend(1.0)
            //         .to_array();
            // }
        }
    } else {
        info!("coordinates not found");
    }

    material.point_transform_matrix = depth_transformer.point_transform_matrix.into();
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid, Reflect)]
#[uuid = "b62bb455-a72c-4b56-87bb-81e0554e234f"]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,

    #[texture(2, visibility(all))]
    coordinates: Handle<Image>,
    #[texture(3, visibility(all))]
    player_index: Handle<Image>,

    #[uniform(4)]
    max_adj_dist: f32,
    #[uniform(5)]
    point_transform_matrix: Mat4,
    #[uniform(6)]
    use_player_index_mask: u32,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material_depth_texture.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_depth_texture.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
        // AlphaMode::Mask(1.0)
    }
    fn specialize(
        pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy_render::render_resource::RenderPipelineDescriptor,
        layout: &bevy_render::mesh::MeshVertexBufferLayout,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy_render::render_resource::SpecializedMeshPipelineError> {
        // descriptor.primitive.polygon_mode = PolygonMode::Line;
        descriptor.primitive.cull_mode = None;
        // descriptor.primitive.unclipped_depth = true;
        // descriptor.primitive.conservative = true;
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

        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            ATTRIBUTE_VERTEX_PIXEL_COORD.at_shader_location(2),
            ATTRIBUTE_PIXEL_COORD_0.at_shader_location(3),
            ATTRIBUTE_PIXEL_COORD_1.at_shader_location(4),
            ATTRIBUTE_PIXEL_COORD_2.at_shader_location(5),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

const ATTRIBUTE_VERTEX_PIXEL_COORD: MeshVertexAttribute =
    MeshVertexAttribute::new("vertex_pixel_coord", 988540923, VertexFormat::Sint32x2);
const ATTRIBUTE_PIXEL_COORD_0: MeshVertexAttribute =
    MeshVertexAttribute::new("pixel_coord_0", 988540920, VertexFormat::Sint32x2);
const ATTRIBUTE_PIXEL_COORD_1: MeshVertexAttribute =
    MeshVertexAttribute::new("pixel_coord_1", 988540921, VertexFormat::Sint32x2);
const ATTRIBUTE_PIXEL_COORD_2: MeshVertexAttribute =
    MeshVertexAttribute::new("pixel_coord_2", 988540922, VertexFormat::Sint32x2);

fn make_subdivided_quad_with_pixel_coords(width: usize, height: usize) -> Mesh {
    let mut positions = Vec::with_capacity(width * height);
    let mut uvs = Vec::with_capacity(width * height);
    let mut vertex_pixel_coord = Vec::with_capacity(width * height);
    let mut pixel_coord_0 = Vec::with_capacity(width * height);
    let mut pixel_coord_1 = Vec::with_capacity(width * height);
    let mut pixel_coord_2 = Vec::with_capacity(width * height);
    for y in 1..height {
        for x in 1..width {
            let x = x as i32;
            let y = y as i32;
            let triangles = [
                // first triangle, upper half
                [[x, y], [x - 1, y], [x - 1, y - 1]],
                // second triangle, lower half
                [[x, y], [x - 1, y - 1], [x, y - 1]],
            ];
            for triangle in triangles {
                for [x, y] in triangle {
                    vertex_pixel_coord.push([x, y]);
                    pixel_coord_0.push(triangle[0]);
                    pixel_coord_1.push(triangle[1]);
                    pixel_coord_2.push(triangle[2]);
                    let xf = (x as f32) / ((width - 1) as f32);
                    let yf = (y as f32) / ((height - 1) as f32);
                    positions.push([xf, yf, 0.0]);
                    uvs.push([xf, yf]);
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(ATTRIBUTE_VERTEX_PIXEL_COORD, vertex_pixel_coord);
    mesh.insert_attribute(ATTRIBUTE_PIXEL_COORD_0, pixel_coord_0);
    mesh.insert_attribute(ATTRIBUTE_PIXEL_COORD_1, pixel_coord_1);
    mesh.insert_attribute(ATTRIBUTE_PIXEL_COORD_2, pixel_coord_2);
    mesh
}
