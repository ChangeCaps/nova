use std::{
    any::Any,
    ops::{Bound, RangeBounds},
};

pub struct BufferInitDescriptor<'a> {
    pub label: Option<&'a str>,
    pub contents: &'a [u8],
    pub usage: wgpu_types::BufferUsage,
}

pub trait BufferTrait {
    fn slice(&self, start: Bound<&u64>, end: Bound<&u64>) -> BufferSlice<'_>;

    fn any(&self) -> &dyn Any;
}

pub struct Buffer(pub(crate) Box<dyn BufferTrait + Send + Sync>);

impl Buffer {
    #[inline]
    pub fn slice(&self, bounds: impl RangeBounds<u64>) -> BufferSlice<'_> {
        self.0.slice(bounds.start_bound(), bounds.end_bound())
    }
}

// SAFETY: this trait is used for ptr casting so only wgpu::BufferSlice should impl this
pub(crate) unsafe trait BufferSliceTrait<'a> {}

pub struct BufferSlice<'a>(pub(crate) Box<dyn BufferSliceTrait<'a> + Send + Sync + 'a>);
