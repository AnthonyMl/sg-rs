use std::sync::{Arc};

use render::{RenderFrame};
use physics::{PhysicsFrame};
use input::{InputFrame};


#[derive(Clone)]
pub enum Frame {
	Render(Arc<RenderFrame>),
	Physics(Arc<PhysicsFrame>),
	Input(Arc<InputFrame>),
}
