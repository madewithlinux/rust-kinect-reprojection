use bevy::{math::Affine3A, prelude::*};
use bevy_prototype_debug_lines::*;
use iyes_loopless::prelude::*;

use kinect1::skeleton::{SkeletonPositionIndex, SkeletonPositionTrackingState, SkeletonTrackingState};

use crate::{
    app_settings::{debug_axes_enabled, debug_coordinate_matchup_enabled, debug_skeleton_enabled, AppSettings},
    receiver::{KinectDepthTransformer, KinectFrameBuffers},
    util::draw_debug_axes,
};

pub struct DebugCoordinatesPlugin;
impl Plugin for DebugCoordinatesPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_system(skeleton_lines.run_if(debug_skeleton_enabled))
            .add_system(axis_references.run_if(debug_axes_enabled))
            .add_system(debug_coordinate_matchup.run_if(debug_coordinate_matchup_enabled));
    }
}

fn skeleton_lines(
    buffers: Res<KinectFrameBuffers>,
    mut lines: ResMut<DebugLines>,
    depth_transformer: Res<KinectDepthTransformer>,
) {
    for &skeleton in buffers.skeleton_frame.skeleton_data.iter() {
        if skeleton.tracking_state == SkeletonTrackingState::NotTracked {
            info!("skip skeleton");
            continue;
        }

        for bone in skeleton.get_skeleton_bones() {
            let Some((start_xyz, end_xyz)) = depth_transformer.skeleton_bone_to_world(&bone, &buffers.depth) else {
                continue;
            };
            let [start_color, end_color] = bone.map(|p| match p.tracking_state {
                SkeletonPositionTrackingState::NotTracked => Color::RED,
                SkeletonPositionTrackingState::Inferred => Color::YELLOW,
                SkeletonPositionTrackingState::Tracked => Color::WHITE,
            });
            let [start_color, end_color] = match bone[1].index {
                SkeletonPositionIndex::HandLeft => [Color::RED, Color::RED],
                SkeletonPositionIndex::HandRight => [Color::BLUE, Color::BLUE],
                _ => [start_color, end_color],
            };
            lines.line_gradient(start_xyz, end_xyz, 0.0, start_color, end_color);
        }
    }
}

fn axis_references(
    mut lines: ResMut<DebugLines>,
    dt: Res<KinectDepthTransformer>,
    // settings: Res<AppSettings>,
) {
    let scale = 1.0;
    lines.line_colored(Vec3::ZERO, Vec3::X * scale, 0.0, Color::RED);
    lines.line_colored(Vec3::ZERO, Vec3::Y * scale, 0.0, Color::GREEN);
    lines.line_colored(Vec3::ZERO, Vec3::Z * scale, 0.0, Color::BLUE);

    // references for the kinect itself
    let start = dt.skeleton_point_to_world(Vec3::ZERO);
    lines.line_colored(
        start,
        dt.skeleton_point_to_world(Vec3::X * scale / 2.0),
        0.0,
        Color::RED,
    );
    lines.line_colored(
        start,
        dt.skeleton_point_to_world(Vec3::Y * scale / 2.0),
        0.0,
        Color::GREEN,
    );
    lines.line_colored(
        start,
        dt.skeleton_point_to_world(Vec3::Z * scale / 2.0),
        0.0,
        Color::BLUE,
    );

    // let transform = settings.kinect_transform.to_affine();
    // draw_debug_axes(&mut lines, &transform, scale / 3.0);
}

pub const REFERENCE_POINTS: [(Vec3, (usize, usize)); 3] = [
    (Vec3::new(1.2902215, -0.021568049, -0.59659046), (136, 346)),
    (Vec3::new(-1.4719752, 0.45078325, -0.96842957), (570, 278)),
    (Vec3::new(1.1872808, 1.5832841, -0.95948), (134, 89)),
];
pub const MORE_REFERENCE_POINTS: [(Vec3, Vec3); 5] = [
    (
        Vec3::new(-0.44902867, 1.2476542, 0.010715961),
        Vec3::new(0.6713257, 0.042388733, 2.3023503),
    ),
    (
        Vec3::new(-0.66511905, 1.476749, 0.38921714),
        Vec3::new(0.8464379, 0.036796886, 1.826863),
    ),
    (
        Vec3::new(0.39814186, 1.4179764, 0.61133754),
        Vec3::new(-0.29460278, -0.062339563, 1.6987133),
    ),
    (
        Vec3::new(0.61787796, 1.363985, -0.26557398),
        Vec3::new(-0.5433431, 0.36454648, 2.476277),
    ),
    (
        Vec3::new(-0.4848734, 0.6623882, -0.33309472),
        Vec3::new(0.691764, -0.29973462, 2.8916762),
    ),
];

fn debug_coordinate_matchup(
    buffers: Res<KinectFrameBuffers>,
    depth_transformer: Res<KinectDepthTransformer>,
    mut lines: ResMut<DebugLines>,
) {
    let depth = &buffers.depth;
    let skeleton_points = &buffers.skeleton_points;
    if skeleton_points.len() < 1 {
        return;
    }

    for (vr_point, kinect_image_point) in REFERENCE_POINTS.iter() {
        let vr_point = *vr_point;
        let flat_index = depth_transformer.ij_to_flat_index(kinect_image_point.0, kinect_image_point.1);
        let pixel_pos = if depth_transformer.point_cloud_skel {
            depth_transformer.skeleton_point_to_world(skeleton_points[flat_index])
        } else {
            depth_transformer.index_depth_to_world(flat_index, depth[flat_index])
        };

        lines.line_colored(vr_point, pixel_pos, 0.0, Color::YELLOW);
        draw_debug_axes(&mut lines, &Affine3A::from_translation(vr_point), 0.1);
    }

    for &(vr_point, sk_point) in MORE_REFERENCE_POINTS.iter() {
        let pixel_pos = depth_transformer.skeleton_point_to_world(sk_point);
        lines.line_colored(vr_point, pixel_pos, 0.0, Color::YELLOW);
        draw_debug_axes(&mut lines, &Affine3A::from_translation(vr_point), 0.1);
    }
}
