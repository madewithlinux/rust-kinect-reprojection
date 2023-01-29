use std::io::Read;

use three_d::*;

use crate::util::default;

// reference: image_cube_effect.rs in three-d (even though that's apparently deprecated?)
pub struct DepthModel {
    vertex_shader_source: String,
    fragment_shader_source: String,
    pub should_reload_shader: bool,

    program: Program,
    positions: VertexBuffer,
    render_states: RenderStates,
}

impl DepthModel {
    pub fn new(context: &Context) -> Self {
        let vertex_shader_source = include_str!("../shaders/depth_model.vert").to_owned();
        let fragment_shader_source = include_str!("../shaders/depth_model.frag").to_owned();
        let program_result = Program::from_source(context, &vertex_shader_source, &fragment_shader_source);
        if let Err(err) = &program_result {
            print_shader_error(&err);
            println!();
        }
        let program = program_result.unwrap();

        let positions = vec![
            vec4(0.5, -0.5, 0.0, 1.0),  // bottom right
            vec4(-0.5, -0.5, 0.0, 1.0), // bottom left
            vec4(0.0, 0.5, 0.0, 1.0),   // top
        ];

        let positions = VertexBuffer::new_with_data(context, &positions);
        Self {
            should_reload_shader: false,
            vertex_shader_source,
            fragment_shader_source,
            program,
            positions,
            render_states: RenderStates::default(),
            // render_states: RenderStates {
            //     write_mask: WriteMask::COLOR,
            //     blend: Blend::TRANSPARENCY,
            //     ..default()
            // },
        }
    }

    fn read_file(file_path: impl AsRef<std::path::Path>) -> String {
        let mut s = String::new();
        std::fs::File::open(file_path).unwrap().read_to_string(&mut s).unwrap();
        s
    }
    pub(crate) fn reload_shaders(&mut self, context: &Context) {
        println!("reloading shaders...");
        self.vertex_shader_source = Self::read_file("shaders/depth_model.vert");
        self.fragment_shader_source = Self::read_file("shaders/depth_model.frag");
        match Program::from_source(context, &self.vertex_shader_source, &self.fragment_shader_source) {
            Ok(program) => {
                self.program = program;
                println!("success!");
            }
            Err(err) => print_shader_error(&err),
        }
    }

    pub(crate) fn update_self(&mut self, frame_input: &mut FrameInput) {
        if self.should_reload_shader {
            self.reload_shaders(&frame_input.context);
            self.should_reload_shader = false;
        }
    }

    pub(crate) fn render(&mut self, render_target: &RenderTarget, camera: &Camera) {
        // render_target.render(&camera, &[&self.model], &[]);
        render_target.write(|| {
            let projection = camera.projection();
            let viewport = camera.viewport();

            self.program.use_uniform("viewProjection", projection);
            self.program.use_vertex_attribute("position", &self.positions);
            self.program
                // .draw_arrays(self.render_states, viewport, self.positions.count() / 3);
                // .draw_arrays(self.render_states, viewport, self.positions.count());
                .draw_arrays(self.render_states, viewport, 12);
        });
    }
}

fn print_shader_error(err: &CoreError) {
    println!("print_shader_error: {:?}", err);
    match err {
        CoreError::ShaderLink(log) | CoreError::ShaderCompilation(_, log, _) => {
            println!("log: {}", log.replace("\\r\\n", "\r\n"));
        }
        _ => (),
    }
}
