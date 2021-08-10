use std::{any::Any, ops::Range};

pub trait BufferTrait {
	fn slice(&self, bounds: Range<u64>) -> BufferSlice<'_>;

	fn any(&self) -> &dyn Any;
}

pub struct Buffer(pub(crate) Box<dyn BufferTrait>);

impl Buffer {
	#[inline]
	pub fn slice(&self, bounds: Range<u64>) -> BufferSlice<'_> {
		self.0.slice(bounds)
	}
}

// SAFETY: this trait is used for ptr casting so only wgpu::BufferSlice should impl this
pub(crate) unsafe trait BufferSliceTrait<'a> {}

pub struct BufferSlice<'a>(pub(crate) Box<dyn BufferSliceTrait<'a> + 'a>);