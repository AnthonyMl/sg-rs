pub struct PhysicsContext;

impl PhysicsContext {
	pub fn new() -> PhysicsContext { PhysicsContext }
}

unsafe impl Send for PhysicsContext {}
unsafe impl Sync for PhysicsContext {}
