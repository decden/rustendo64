pub trait Sink<T> {
	fn append(&mut self, value: T);
}

pub trait SinkRef<T: ?Sized> {
	fn append(&mut self, value: &T);
}

pub struct VideoFrame {
	pub argb_data: Box<[u32]>,
	pub width: u32,
	pub height: u32,
}