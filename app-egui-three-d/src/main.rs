pub mod camera_orbit_control;

use camera_orbit_control::CameraOrbitControl;
use three_d::*;

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        // vec3(0.0, 0.0, 2.0),
        // vec3(0.0, 0.0, 0.0),
        // vec3(0.0, 1.0, 0.0),
        vec3(0.5, 3.6, 2.6),
        vec3(0.0, 0.0, -0.8),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    let mut orbit_drag_speed = 0.1;
    let mut control = CameraOrbitControl::new(*camera.target(), 0.1, 100.0);
    control.set_orbit_drag_speed(orbit_drag_speed);

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0),   // top
    ];
    let colors = vec![
        Color::new(255, 0, 0, 255), // bottom right
        Color::new(0, 255, 0, 255), // bottom left
        Color::new(0, 0, 255, 255), // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
    let mut axes = Axes::new(&context, 0.05, 0.8);

    // Start the main render loop
    let mut gui = three_d::GUI::new(&context);
    let mut angle = 0.0;
    window.render_loop(
        move |mut frame_input: FrameInput| // Begin a new frame with an updated frame input
    {

        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                Window::new("window").show(gui_context, |ui| {
                    ui.add(Slider::new(&mut angle, 0.01..=10.0).text("angle"));

                    if ui.add(Slider::new(&mut orbit_drag_speed, 0.01..=0.5).text("orbit drag speed")).changed() {
                        control.set_orbit_drag_speed(orbit_drag_speed);
                    }
                });
                // SidePanel::left("side_panel").show(gui_context, |ui| {
                //     use three_d::egui::*;
                //     ui.heading("Debug Panel");
                //     ui.add(Slider::new(&mut viewport_zoom, 0.01..=1.0).text("Viewport"));
                //     ui.add(Slider::new(&mut scissor_zoom, 0.01..=1.0).text("Scissor"));
                // });
                // panel_width = gui_context.used_size().x as f64;
            },
        );
        model.set_transformation(Mat4::from_angle_y(radians((angle) as f32)));
        axes.set_transformation(Mat4::from_angle_y(radians((angle) as f32)));

        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // // Set the current transformation of the triangle
        // model.set_transformation(Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.005) as f32)));

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, &[&model], &[]
            )
            .render(
                &camera, &[&axes], &[]
            )
            .write(|| gui.render());

        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}
