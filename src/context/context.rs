use std::sync::{Arc, RwLock};

use crossbeam::sync::{MsQueue};
use glium::glutin::{WindowBuilder, CursorState, get_primary_monitor};
use glium::{DisplayBuild};

use render::{RenderContext, RenderProcessor, RenderFrame};
use input::{InputContext, InputFrame};
use physics::{PhysicsContext, PhysicsFrame};
use context::context_state::{ContextState};


// TODO: try to remove Arc dependency
// TODO: maybe rename ContextType->Context and Context->IsContext or something like that?
//

unsafe impl Send for ContextType {}
unsafe impl Sync for ContextType {}

pub trait Context: Send + Sync {
	fn frequency(&self) -> u64;
	fn is_ready(&self, context: Arc<ContextType>) -> bool;
	fn tick(&self,     context: Arc<ContextType>);
}

pub fn create() -> (Arc<ContextType>, RenderProcessor) {
	let window_size = get_primary_monitor().get_dimensions();
	let window_size = (window_size.0/2, window_size.1/2);

	let glium_context = WindowBuilder::new()
		.with_dimensions(window_size.0, window_size.1)
		.with_title(format!("SG"))
		.with_depth_buffer(24)
		.with_decorations(false)
		.build_glium().unwrap();

	glium_context.get_window().unwrap().set_cursor_state(CursorState::Grab).ok();

	let q = Arc::new(MsQueue::new());

	let input   = InputContext  ::new();
	let physics = PhysicsContext::new();
	let render  = RenderContext ::new(q.clone(), window_size);

	let frame_input   = InputFrame  ::frame_zero();
	let frame_physics = PhysicsFrame::frame_zero(window_size);
	let frame_render  = RenderFrame ::frame_zero();

	(
		Arc::new(ContextType::new(input, physics, render, frame_input, frame_physics, frame_render)),
		RenderProcessor::new(q, glium_context),
	)
}

macro_rules! register_contexts {
	($({ $context_type:ty, $frame_type:ident, $kind:ident, $name:ident, $fn_frame:ident, $fn_counter:ident, $frequency:expr }),* ) => {
		enum ContextKind {
			$( $kind(Arc<$context_type>), )*
		}

		fn to_context(context: &ContextKind) -> Arc<Context> {
			match *context {
				$( ContextKind::$kind(ref $name) => $name.clone(), )*
			}
		}

		pub struct ContextType {
			$( $name: (Arc<$context_type>, ContextState, RwLock<Arc<$frame_type>>), )*
		}

		impl ContextType {
			pub fn new(
				$($name: $context_type,)*
				$($fn_frame: $frame_type,)*
			) -> ContextType
			{
				ContextType {
					$($name: (
						Arc::new($name),
						ContextState::new(),
						RwLock::new(Arc::new($fn_frame)),
					),)*
				}
			}

			pub fn contexts(&self) -> Box<[Arc<Context>]> {
				[$(
					ContextKind::$kind(self.$name.0.clone()),
				)*].into_iter().map(to_context).collect::<Vec<Arc<Context>>>().into_boxed_slice()
			}

			$(
				#[allow(dead_code)]
				pub fn $name(&self) -> Arc<$context_type> { self.$name.0.clone() }

				#[allow(dead_code)]
				pub fn $fn_frame(&self) -> Arc<$frame_type> {
					(self.$name.2.read().unwrap()).clone()
				}

				#[allow(dead_code)]
				pub fn $fn_counter(&self) -> u64 {
					self.$name.1.frame_counter()
				}
			)*
		}

		$(
			impl Context for $context_type {
				fn frequency(&self) -> u64 { $frequency }

				fn tick(&self, context: Arc<ContextType>) {
					let last_frame = (*context.$name.2.read().unwrap()).clone();
					context.$name.1.increment();

					let new_frame = $frame_type::new(context.clone(), last_frame);

					(*context.$name.2.write().unwrap()) = Arc::new(new_frame);

					context.$name.1.release_ready_lock();
				}

				fn is_ready(&self, context: Arc<ContextType>) -> bool {
					context.$name.1.is_ready()
				}
			}
		)*
	};
}

register_contexts!(
	{ InputContext,   InputFrame,   Input,   input,   frame_input,   counter_input,   120 },
	{ PhysicsContext, PhysicsFrame, Physics, physics, frame_physics, counter_physics, 120 },
	{ RenderContext,  RenderFrame,  Render,  render,  frame_render,  counter_render,   60 }
);
