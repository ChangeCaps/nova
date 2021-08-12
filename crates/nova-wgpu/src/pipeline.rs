use std::{any::Any, borrow::Cow};

use crate::BindGroupLayout;

pub enum ShaderSource<'a> {
    SpirV(Cow<'a, [u32]>),
    Wgsl(Cow<'a, str>),
}

pub struct ShaderModuleDescriptor<'a> {
    pub label: Option<&'a str>,
    pub source: ShaderSource<'a>,
    pub flags: wgpu_types::ShaderFlags,
}

pub struct ShaderModule(pub(crate) Box<dyn Any + Send + Sync>);

pub struct PipelineLayoutDescriptor<'a> {
    pub label: Option<&'a str>,
    pub bind_group_layouts: &'a [&'a BindGroupLayout],
    pub push_constant_ranges: &'a [wgpu_types::PushConstantRange],
}

pub struct PipelineLayout(pub(crate) Box<dyn Any + Send + Sync>);

#[derive(Clone, Debug)]
pub struct VertexBufferLayout<'a> {
    pub array_stride: u64,
    pub step_mode: wgpu_types::InputStepMode,
    pub attributes: &'a [wgpu_types::VertexAttribute],
}

#[derive(Clone)]
pub struct VertexState<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: &'a str,
    pub buffers: &'a [VertexBufferLayout<'a>],
}

#[derive(Clone)]
pub struct FragmentState<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: &'a str,
    pub targets: &'a [wgpu_types::ColorTargetState],
}

pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub layout: Option<&'a PipelineLayout>,
    pub vertex: VertexState<'a>,
    pub fragment: Option<FragmentState<'a>>,
    pub depth_stencil: Option<wgpu_types::DepthStencilState>,
    pub primitive: wgpu_types::PrimitiveState,
    pub multisample: wgpu_types::MultisampleState,
}

pub struct RenderPipeline(pub(crate) Box<dyn Any + Send + Sync>);
