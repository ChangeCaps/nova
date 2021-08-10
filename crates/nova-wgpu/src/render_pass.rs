use std::ops::Range;

use crate::{BindGroup, BufferSlice, RenderPipeline, texture::TextureView};

#[derive(Clone, Debug)]
pub enum LoadOp<V> {
    Clear(V),
    Load,
}

#[derive(Clone, Debug)]
pub struct Operations<V> {
    pub load: LoadOp<V>,
    pub store: bool,
}

pub struct RenderPassColorAttachment<'a> {
    pub view: &'a TextureView<'a>,
    pub resolve_target: Option<&'a TextureView<'a>>,
    pub ops: Operations<wgpu_types::Color>,
}

pub struct RenderPassDepthStencilAttachment<'a> {
    pub view: &'a TextureView<'a>,
    pub depth_ops: Option<Operations<f32>>,
    pub stencil_ops: Option<Operations<u32>>,
}

pub struct RenderPassDescriptor<'a, 'b> {
    pub label: Option<&'a str>,
    pub color_attachments: &'b [RenderPassColorAttachment<'a>],
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
}

pub trait RenderPassTrait<'a> {
    fn set_bind_group(&mut self, index: u32, bind_group: &'a BindGroup, offsets: &[u32]);
    fn set_pipeline(&mut self, pipeline: &'a RenderPipeline);
    fn set_blend_constant(&mut self, color: wgpu_types::Color);
    fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>, index_format: wgpu_types::IndexFormat);
    fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>);
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);
}

pub struct RenderPass<'a>(pub(crate) Box<dyn RenderPassTrait<'a> + 'a>);

impl<'a> RenderPass<'a> {
    #[inline]
    pub fn set_bind_group(&mut self, index: u32, bind_group: &'a BindGroup, offsets: &[u32]) {
        self.0.set_bind_group(index, bind_group, offsets);
    }

    #[inline]
    pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        self.0.set_pipeline(pipeline);
    }

    #[inline]
    pub fn set_blend_constant(&mut self, color: wgpu_types::Color) {
        self.0.set_blend_constant(color);
    }

    #[inline]
    pub fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>, index_format: wgpu_types::IndexFormat) {
        self.0.set_index_buffer(buffer_slice, index_format);
    }

    #[inline]
    pub fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
        self.0.set_vertex_buffer(slot, buffer_slice);
    }
    
    #[inline] 
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.0.draw(vertices, instances);
    }

    #[inline]
    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.0.draw_indexed(indices, base_vertex, instances);
    }
}