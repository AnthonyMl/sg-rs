use glium::{DrawParameters, Program};
use glium::backend::{Facade};


pub struct ImageProgram {
	pub program:    Program,
	pub parameters: DrawParameters<'static>,
}

impl ImageProgram {
	pub fn new<F: Facade>(facade: &F) -> ImageProgram {
		let program = {
			let vertex_source = r#"
				#version 140

				in vec2 position;
				in vec2 texture_coordinates;

				out vec2 v_texture_coordinates;

				void main() {
					v_texture_coordinates = texture_coordinates;
					gl_Position  = vec4(position, 0.0, 1.0);
				}
			"#;
			let fragment_source = r#"
				#version 140

				in vec2 v_texture_coordinates;

				out vec4 color;

				uniform sampler2D texture_sampler;

				void main() {
					color = texture(texture_sampler, v_texture_coordinates);
				}
			"#;
			Program::from_source(facade, vertex_source, fragment_source, None).expect("Unable to compile image shader")
		};

		ImageProgram {
			program: program,
			parameters: Default::default(),
		}
	}
}