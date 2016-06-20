use glium::{BackfaceCullingMode, Depth, DepthTest, DrawParameters, Program};
use glium::backend::{Facade};


pub struct ShadowProgram {
	pub program:    Program,
	pub parameters: DrawParameters<'static>,
}

impl ShadowProgram {
	pub fn new<F: Facade>(facade: &F) -> ShadowProgram {
		let program = {
			let vertex_source = r#"
				#version 140

				in vec3 position;

				uniform mat4 shadow;

				void main() {
					gl_Position = shadow * vec4(position, 1.0);
				}
			"#;
			let fragment_source = r#"
				#version 140
				void main() { }
			"#;
			Program::from_source(facade, vertex_source, fragment_source, None).expect("Unable to compile shadow shader")
		};

		ShadowProgram {
			program: program,
			parameters: DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				color_mask: (false, false, false, false),
				backface_culling: BackfaceCullingMode::CullCounterClockwise,
				.. Default::default()
			},
		}
	}
}