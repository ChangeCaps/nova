use std::{any::Any, num::NonZeroU64};

use crate::{Buffer, TextureView};

pub struct BindGroupLayoutDescriptor<'a> {
    pub label: Option<&'a str>,
    pub entries: &'a [wgpu_types::BindGroupLayoutEntry],
}

pub struct BindGroupLayout(pub(crate) Box<dyn Any>);

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

pub struct BindGroup(pub(crate) Box<dyn Any>);
