use rust_kinect_reprojection_app_bevy::camera2_vmc_osc_receiver::read_camera2_info_from_config_files;

fn main() {
    let camera2_settings_folder = r#"C:\Program Files (x86)\Steam\steamapps\common\Beat Saber\UserData\Camera2\"#;
    let vmc_camera_config = read_camera2_info_from_config_files(camera2_settings_folder).unwrap();
    let s = serde_json::to_string_pretty(&vmc_camera_config).unwrap();
    println!("{}", s);
}
