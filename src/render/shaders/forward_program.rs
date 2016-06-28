use glium::{BackfaceCullingMode, Depth, DepthTest, DrawParameters, Program};
use glium::backend::{Facade};


pub struct ForwardProgram {
	pub program:    Program,
	pub parameters: DrawParameters<'static>,
}

impl ForwardProgram {
	pub fn new<F: Facade>(facade: &F) -> ForwardProgram {
		let program = {
			let vertex_source = r#"
				#version 140

				in vec3 position;
				in vec3 normal;

				out vec3 v_normal;
				out vec4 v_shadow_pos;

				uniform mat4 shadow;
				uniform mat4 model;
				uniform mat4 model_view_projection;

				void main() {
					v_normal = normalize((model * vec4(normal, 0.0)).xyz);

					vec4 v4_position = vec4(position, 1.0);
					v_shadow_pos = shadow                * v4_position;
					gl_Position  = model_view_projection * v4_position;
				}
			"#;
			let fragment_source = r#"
				#version 140

				in vec3 v_normal;
				in vec4 v_shadow_pos;

				out vec4 color;

				uniform vec3 reverse_light_direction;

				uniform sampler2D shadow_map;

				void main() {
					vec3 shadow_pos = 0.5 + 0.5 * (v_shadow_pos.xyz / v_shadow_pos.w); // TODO: may not be necessary
					float closest_depth = texture(shadow_map, shadow_pos.xy).r;
					float shadow = (shadow_pos.z - 0.0005) > closest_depth ? 0.1 : 1.0;

					float value = dot(v_normal, reverse_light_direction);
					float intensity = shadow * max(0.1, 0.9 * value);
					color = vec4(intensity, intensity, intensity, 1.0);
				}
			"#;
			Program::from_source(facade, vertex_source, fragment_source, None).expect("Unable to compile forward shader")
		};

		ForwardProgram {
			program: program,
			parameters: DrawParameters {
				depth: Depth {
					test: DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				backface_culling: BackfaceCullingMode::CullClockwise,
				.. Default::default()
			},
		}
	}
}