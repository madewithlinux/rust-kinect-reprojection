use three_d::*;

use crate::util::default;

pub struct DepthModel {
    pub angle: f32,
    pub model: Gm<Mesh, ColorMaterial>,
}

impl DepthModel {
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
            angle: default(),
            model,
        }
    }

    pub(crate) fn update_self(&mut self, frame_input: &mut FrameInput) {
        self.model
            .set_transformation(Mat4::from_angle_y(radians((self.angle) as f32)));
    }

    pub(crate) fn render(&mut self, render_target: &RenderTarget, camera: &Camera) {
        render_target.render(&camera, &[&self.model], &[]);
    }
}
