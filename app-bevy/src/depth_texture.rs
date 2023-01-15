use bevy::{math::Affine3A, prelude::*};
use bevy_render::{mesh::VertexAttributeValues, render_resource::PrimitiveTopology};
use bytemuck::checked::cast_slice_mut;
use genmesh::{
    generators::{IndexedPolygon, SharedVertex},
    MapVertex, Triangulate,
};


use crate::{
    dock_ui::MainCamera,
    frame_visualization_util::{get_buffer, FrameBufferDescriptor, FrameBufferImageHandle},
    receiver::{KinectDepthTransformer, KinectFrameBuffers},
    util::draw_debug_axes,
    COLOR_HEIGHT, COLOR_WIDTH, DEPTH_HEIGHT, DEPTH_WIDTH,
};

pub struct DepthTexturePlugin;
impl Plugin for DepthTexturePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_startup_system(spawn_depth_texture)
            .add_system(update_depth_texture);
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
    // let plane = genmesh::generators::Plane::subdivide(DEPTH_WIDTH, DEPTH_HEIGHT);
    // let vertices = plane.shared_vertex_iter().collect_vec();
    // let f1 = plane
    //     .indexed_polygon_iter()
    //     .triangulate()
    //     .map(|f| f.map_vertex(|u| vertices[u]))
    //     .collect_vec();
    // let f0: Vec<_> = plane.triangulate().collect();

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

    let quad_handle = meshes.add(make_subdivided_quad(DEPTH_WIDTH, DEPTH_HEIGHT));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: true,
        ..default()
    });

    commands.spawn((
        Name::new("depth_texture"),
        DepthTextureMarker,
        // FrameBufferImageHandle(FrameBufferDescriptor::CurrentColor, image_handle.clone()),
        PbrBundle {
            mesh: quad_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}

fn make_subdivided_quad(width: usize, height: usize) -> Mesh {
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
    mesh.set_indices(Some(bevy_render::mesh::Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

fn update_depth_texture(
    buffers: Res<KinectFrameBuffers>,
    depth_texture: Query<(&DepthTextureMarker, &Handle<Mesh>, &Handle<StandardMaterial>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    if buffers.derived_frame.depth.len() == 0 {
        info!("depth frame empty");
        return;
    }

    let (_marker, mesh_handle, material_handle) = depth_texture.single();
    let Some(material) = materials.get_mut(&material_handle) else {
        info!("material not found");
        return;
    };
    let Some(image_handle) = &material.base_color_texture else {
        info!("image handle not found");
        return;
    };
    if let Some(image) = images.get_mut(image_handle) {
        // get_buffer(&FrameBufferDescriptor::CurrentColor, &buffers, &mut image.data);
        let image_data = cast_slice_mut::<_, [u8; 4]>(&mut image.data);
        for (flat_index, (&rgba, &depth)) in buffers
            .current_frame
            .rgba
            .iter()
            .zip(buffers.current_frame.depth.iter())
            .enumerate()
        {
            image_data[flat_index] = if depth == 0 { [0, 0, 0, 0] } else { rgba };
        }
    } else {
        info!("image not found");
    }

    let Some(mesh) = meshes.get_mut(&mesh_handle) else {
        info!("mesh not found");
        return;
    };
    let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        info!("position attribute missing from mesh");
        return;
    };
    for (flat_index, &sk_point) in buffers.current_frame.skeleton_points.iter().enumerate() {
        if sk_point == Vec3::ZERO {
            positions[flat_index] = depth_transformer.index_depth_to_world(flat_index, 4_000).to_array();
        } else {
            positions[flat_index] = depth_transformer.skeleton_point_to_world(sk_point).to_array();
        }
    }
}
