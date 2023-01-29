use three_d::*;

use crate::util::default;

pub struct DebugModels {
    pub axes: Axes,
    // test triangle
    pub angle: f32,
    pub model: Gm<Mesh, ColorMaterial>,
    pub show_test_triangle: bool,
    pub show_axes: bool,
}

impl DebugModels {
    pub fn new(context: &Context) -> Self {
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
        let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

        Self {
            axes: Axes::new(&context, 0.05, 0.8),
            angle: default(),
            model,
            show_test_triangle: false,
            show_axes: true,
        }
    }

    pub(crate) fn update_self(&mut self, frame_input: &mut FrameInput) {
        self.model
            .set_transformation(Mat4::from_angle_y(radians((self.angle) as f32)));
    }

    pub(crate) fn update_app(app: &mut crate::app::App, frame_input: &mut FrameInput) {
        // let this = &mut app.debug_models;
        // this.axes
        //     .set_transformation(Mat4::from_angle_y(radians((app.depth_model.angle) as f32)));
    }

    pub(crate) fn render(&mut self, render_target: &RenderTarget, camera: &Camera) {
        if self.show_test_triangle {
            render_target.render(&camera, &[&self.model], &[]);
        }
        if self.show_axes {
            render_target.render(&camera, &[&self.axes], &[]);
        }
    }
}
