pub trait Context {
	fn rate(&self) -> u64;
	fn tick(&self);
}
