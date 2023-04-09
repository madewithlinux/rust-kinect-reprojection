use std::{
    fs,
    time::{Instant, SystemTime},
};

use anyhow::Result;
use glam::Vec3;
use openvr::{TrackedDeviceIndex, TrackedDevicePoses, MAX_TRACKED_DEVICE_COUNT};

fn main() -> Result<()> {
    println!("hello world");

    let context = unsafe { openvr::init(openvr::ApplicationType::Utility)? };
    println!("inited");
    let system = context.system()?;
    println!("system");

    println!("# Poses ");
    let poses = system.device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::Standing, 0.0);
    for i in 0..MAX_TRACKED_DEVICE_COUNT {
        let pose = poses[i];
        if !pose.pose_is_valid() {
            continue;
        }
        tracked_device_info(&system, i as u32, &poses);
    }

    // tracked_device_info(&system, 0, &poses);

    // let left_controller_index = system
    //     .tracked_device_index_for_controller_role(openvr::TrackedControllerRole::LeftHand)
    //     .unwrap();
    // // let right_controller_index = system
    // //     .tracked_device_index_for_controller_role(openvr::TrackedControllerRole::RightHand)
    // //     .unwrap();

    // let mut avg_position = Vec3::splat(0.0);
    // let sample_count = 256;
    // for _i in 0..sample_count {
    //     let poses = system.device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::Standing, 0.0);
    //     let device_to_absolute_tracking = poses[left_controller_index as usize].device_to_absolute_tracking();
    //     avg_position += Vec3::new(
    //         device_to_absolute_tracking[0][3],
    //         device_to_absolute_tracking[1][3],
    //         device_to_absolute_tracking[2][3],
    //     );
    // }
    // avg_position /= sample_count as f32;
    // println!("avg_position={:?}", avg_position);
    // let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    // let path = format!("logged_coordinates/left_controller_{}.txt", timestamp.as_secs_f64());
    // fs::write(&path, format!("{:?}", avg_position)).unwrap();
    // println!("wrote to {}", path);

    // loop {
    //     let poses = system.device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::Standing, 0.0);
    //     let pose = poses[left_controller_index as usize];
    //     println!(
    //         // "velocity [{:8.5}, {:8.5}, {:8.5}], [{:8.5}, {:8.5}, {:8.5}], tracking {:?}",
    //         "velocity [{:8.5}, {:8.5}, {:8.5}], [{:8.5}, {:8.5}, {:8.5}]",
    //         pose.velocity()[0],
    //         pose.velocity()[1],
    //         pose.velocity()[2],
    //         pose.angular_velocity()[0],
    //         pose.angular_velocity()[1],
    //         pose.angular_velocity()[2],
    //         // pose.device_to_absolute_tracking()
    //     );
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    // }

    Ok(())
}

fn tracked_device_info(system: &openvr::System, i: TrackedDeviceIndex, poses: &TrackedDevicePoses) {
    println!("tracked device {}", i);
    println!("\t is connected     {:?}", system.is_tracked_device_connected(i));
    println!("\t device class     {:?}", system.tracked_device_class(i));
    println!("\t controller state {:?}", system.controller_state(i));

    println!("\t properties:");
    let props = [
        (openvr::property::TrackingSystemName_String, "tracking system name"),
        (openvr::property::ModelNumber_String, "model number"),
        (openvr::property::SerialNumber_String, "serial number"),
        (openvr::property::RenderModelName_String, "render model name"),
    ];
    for (prop, label) in props.iter() {
        println!(
            "\t\t {:30} {:?}",
            label,
            system.string_tracked_device_property(i, *prop)
        );
    }

    println!("\t pose:");
    let pose = poses[i as usize];
    println!("\t\t pose_is_valid        {:?}", pose.pose_is_valid());
    if !pose.pose_is_valid() {
        return;
    }
    println!("\t\t velocity                    {:?}", pose.velocity());
    println!("\t\t angular_velocity            {:?}", pose.angular_velocity());
    println!("\t\t tracking_result             {:?}", pose.tracking_result());
    println!("\t\t device_is_connected         {:?}", pose.device_is_connected());
    // println!(
    //     "\t\t device_to_absolute_tracking {:?}",
    //     pose.device_to_absolute_tracking()
    // );
    print!("\t\t device_to_absolute_tracking: ");
    print_matrix(46, pose.device_to_absolute_tracking());
}

// https://github.com/rust-openvr/rust-openvr/blob/master/examples/test.rs
fn print_matrix<M, N>(offset: u32, mat: M)
where
    M: AsRef<[N]>,
    N: AsRef<[f32]>,
{
    let offset: String = (0..offset).map(|_| ' ').collect();
    let mut is_first_row = true;
    for row in mat.as_ref() {
        if !is_first_row {
            print!("{}", offset);
        }
        let row_interior: Vec<_> = row.as_ref().iter().map(|x| format!("{:7.4}", x)).collect();
        println!("[{}]", row_interior.join(", "));
        is_first_row = false;
    }
}
