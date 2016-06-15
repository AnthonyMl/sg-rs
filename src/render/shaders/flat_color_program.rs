use glium::{Depth, DepthTest, DrawParameters, Program};
use glium::backend::{Facade};

pub struct FlatColorProgram {
	pub program:    Program,
	pub parameters: DrawParameters<'static>,
}

impl FlatColorProgram {
	pub fn new<F: Facade>(facade: &F) -> FlatColorProgram {
		let program = {
			let vertex_source = r#"
				#version 140

				in vec4 position;
				in vec3 color;

				flat out vec3 v_color;

				void main() {
					v_color = color;
					gl_Position = position;
				}
			"#;
			let fragment_source = r#"
				#version 140

				flat in vec3 v_color;

				out vec4 color;

				void main() {
					color = vec4(v_color, 1.0);
				}
			"#;
			Program::from_source(facade, vertex_source, fragment_source, None).expect("Unable to compile flat color shader")
		};

		FlatColorProgram {
			program: program,
			parameters: DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				.. Default::default()
			},
		}
	}
}