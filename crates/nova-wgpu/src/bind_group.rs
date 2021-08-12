use std::{any::Any, num::NonZeroU64, sync::Arc};

use crate::{Buffer, TextureView};

pub struct BindGroupLayoutDescriptor<'a> {
    pub label: Option<&'a str>,
    pub entries: &'a [wgpu_types::BindGroupLayoutEntry],
}

pub struct BindGroupLayout(pub(crate) Box<dyn Any + Send + Sync>);

pub struct BufferBinding<'a> {
    pub buffer: &'a Buffer,
    pub offset: u64,
    pub size: Option<NonZeroU64>,
}

pub enum BindingResource<'a> {
    Buffer(BufferBinding<'a>),
    BufferArray(&'a [BufferBinding<'a>]),
    TextureView(&'a TextureView<'a>),
    TextureViewArray(&'a [&'a TextureView<'a>]),
}

pub struct BindGroupEntry<'a> {
    pub binding: u32,
    pub resource: BindingResource<'a>,
}

pub struct BindGroupDescriptor<'a> {
    pub label: Option<&'a str>,
    pub layout: &'a BindGroupLayout,
    pub entries: &'a [BindGroupEntry<'a>],
}

#[derive(Clone)]
pub struct BindGroup(pub(crate) Arc<dyn Any + Send + Sync>);
