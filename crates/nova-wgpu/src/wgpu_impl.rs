use std::{
    any::{Any, TypeId},
    ops::Bound,
    sync::Arc,
};

use wgpu::util::DeviceExt;

use crate::{
    buffer::{BufferSlice, BufferSliceTrait, BufferTrait},
    command_encoder::CommandEncoderTrait,
    render_pass::RenderPassTrait,
    texture::{SwapChainTextureTrait, TextureTrait},
    *,
};

pub struct WgpuInstance {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl WgpuInstance {
    #[inline]
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }
}

pub struct WgpuSwapChain {
    pub(crate) surface: wgpu::Surface,
    pub(crate) desc: wgpu::SwapChainDescriptor,
    pub(crate) swapchain: wgpu::SwapChain,
}

impl WgpuSwapChain {
    #[inline]
    pub fn new(
        device: &wgpu::Device,
        surface: wgpu::Surface,
        desc: wgpu::SwapChainDescriptor,
    ) -> Self {
        let swapchain = device.create_swap_chain(&surface, &desc);

        Self {
            surface,
            desc,
            swapchain,
        }
    }
}

impl From<wgpu::SwapChainError> for SwapChainError {
    fn from(err: wgpu::SwapChainError) -> Self {
        match err {
            wgpu::SwapChainError::Timeout => Self::Timeout,
            wgpu::SwapChainError::Outdated => Self::Outdated,
            wgpu::SwapChainError::Lost => Self::Lost,
            wgpu::SwapChainError::OutOfMemory => Self::OutOfMemory,
        }
    }
}

fn buffer_binding<'a>(buffer: &BufferBinding<'a>) -> wgpu::BufferBinding<'a> {
    wgpu::BufferBinding {
        buffer: buffer.buffer.0.any().downcast_ref().unwrap(),
        offset: buffer.offset,
        size: buffer.size,
    }
}

impl SwapChain for WgpuSwapChain {
    #[inline]
    fn format(&self) -> wgpu_types::TextureFormat {
        self.desc.format
    }

    #[inline]
    fn size(&self) -> (u32, u32) {
        (self.desc.width, self.desc.height)
    }

    #[inline]
    fn recreate(&mut self, instance: &dyn Instance, width: u32, height: u32) {
        self.desc.width = width;
        self.desc.height = height;
        self.swapchain = instance
            .any()
            .downcast_ref::<WgpuInstance>()
            .unwrap()
            .device
            .create_swap_chain(&self.surface, &self.desc);
    }

    #[inline]
    fn get_current_frame(&self) -> Result<SwapChainFrame, SwapChainError> {
        let frame = self.swapchain.get_current_frame()?;

        Ok(SwapChainFrame {
            output: SwapChainTexture(Box::new(frame.output)),
            suboptimal: frame.suboptimal,
        })
    }
}

impl Instance for WgpuInstance {
    #[inline]
    fn create_buffer(&self, desc: &wgpu_types::BufferDescriptor<Option<&str>>) -> Buffer {
        let buffer = self.device.create_buffer(desc);
        Buffer(Box::new(buffer))
    }

    #[inline]
    fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> Buffer {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: desc.label,
                contents: desc.contents,
                usage: desc.usage,
            });
        Buffer(Box::new(buffer))
    }

    #[inline]
    fn create_texture(&self, desc: &wgpu_types::TextureDescriptor<Option<&str>>) -> Texture {
        let texture = self.device.create_texture(desc);
        Texture(Box::new(texture))
    }

    #[inline]
    fn create_command_encoder(
        &self,
        desc: &wgpu_types::CommandEncoderDescriptor<Option<&str>>,
    ) -> CommandEncoder {
        let command_encoder = self.device.create_command_encoder(desc);
        CommandEncoder(Box::new(command_encoder))
    }

    #[inline]
    fn create_bind_group_layout(&self, desc: &BindGroupLayoutDescriptor) -> BindGroupLayout {
        BindGroupLayout(Box::new(self.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: desc.label,
                entries: desc.entries,
            },
        )))
    }

    #[inline]
    fn create_bind_group(&self, desc: &BindGroupDescriptor) -> BindGroup {
        let entries = desc
            .entries
            .iter()
            .map(|entry| wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: match &entry.resource {
                    BindingResource::Buffer(buffer) => {
                        wgpu::BindingResource::Buffer(buffer_binding(buffer))
                    }
                    BindingResource::TextureView(view) => {
                        wgpu::BindingResource::TextureView(view.any().downcast_ref().unwrap())
                    }
                    _ => unreachable!(),
                },
            })
            .collect::<Vec<_>>();

        BindGroup(Arc::new(self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: desc.label,
                layout: desc.layout.0.downcast_ref().unwrap(),
                entries: &entries,
            },
        )))
    }

    #[inline]
    fn create_shader_module(&self, desc: &ShaderModuleDescriptor) -> ShaderModule {
        ShaderModule(Box::new(self.device.create_shader_module(
            &wgpu::ShaderModuleDescriptor {
                label: desc.label,
                source: match &desc.source {
                    ShaderSource::SpirV(spirv) => wgpu::ShaderSource::SpirV(spirv.clone()),
                    ShaderSource::Wgsl(wgsl) => wgpu::ShaderSource::Wgsl(wgsl.clone()),
                },
                flags: desc.flags,
            },
        )))
    }

    #[inline]
    fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor) -> PipelineLayout {
        let layouts = desc
            .bind_group_layouts
            .iter()
            .map(|layout| layout.0.downcast_ref().unwrap())
            .collect::<Vec<_>>();

        PipelineLayout(Box::new(self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: desc.label,
                bind_group_layouts: &layouts,
                push_constant_ranges: desc.push_constant_ranges,
            },
        )))
    }

    #[inline]
    fn create_render_pipeline(&self, desc: &RenderPipelineDescriptor) -> RenderPipeline {
        let vertex_buffers = desc
            .vertex
            .buffers
            .iter()
            .map(|buffer| wgpu::VertexBufferLayout {
                array_stride: buffer.array_stride,
                step_mode: buffer.step_mode,
                attributes: buffer.attributes,
            })
            .collect::<Vec<_>>();

        RenderPipeline(Box::new(self.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: desc.label,
                layout: desc.layout.map(|layout| layout.0.downcast_ref().unwrap()),
                vertex: wgpu::VertexState {
                    module: desc.vertex.module.0.downcast_ref().unwrap(),
                    entry_point: desc.vertex.entry_point,
                    buffers: &vertex_buffers,
                },
                fragment: desc.fragment.as_ref().map(|fragment| wgpu::FragmentState {
                    module: fragment.module.0.downcast_ref().unwrap(),
                    entry_point: fragment.entry_point,
                    targets: fragment.targets,
                }),
                depth_stencil: desc.depth_stencil.clone(),
                primitive: desc.primitive,
                multisample: desc.multisample,
            },
        )))
    }

    #[inline]
    fn submit(&self, command_encoder: CommandEncoder) {
        if command_encoder.0.any().type_id() == TypeId::of::<wgpu::CommandEncoder>() {
            // SAFETY: we just checked that TypeIds are equal, so pointer casting is safe.
            let command_encoder = unsafe {
                Box::from_raw(
                    Box::into_raw(command_encoder.0) as *mut _ as *mut wgpu::CommandEncoder
                )
            };
            self.queue.submit(std::iter::once(command_encoder.finish()));
        } else {
            unreachable!();
        }
    }

    #[inline]
    fn write_buffer(&self, buffer: &Buffer, offset: u64, data: &[u8]) {
        let buffer = buffer.0.any().downcast_ref().unwrap();

        self.queue.write_buffer(buffer, offset, data);
    }

    #[inline]
    fn any(&self) -> &dyn Any {
        self
    }
}

impl BufferTrait for wgpu::Buffer {
    #[inline]
    fn slice(&self, start: Bound<&u64>, end: Bound<&u64>) -> BufferSlice<'_> {
        BufferSlice(Box::new(self.slice((start, end))))
    }

    #[inline]
    fn any(&self) -> &dyn Any {
        self
    }
}

unsafe impl<'a> BufferSliceTrait<'a> for wgpu::BufferSlice<'a> {}

fn downcast_buffer_slice<'a, 'b>(slice: BufferSlice<'a>) -> wgpu::BufferSlice<'a> {
    // SAFETY: wgpu::BufferSlice is the ONLY implementer of BufferSliceTrait, meaning casting is safe.
    unsafe { *Box::from_raw(Box::into_raw(slice.0) as *mut _ as *mut wgpu::BufferSlice<'a>) }
}

impl TextureTrait for wgpu::Texture {
    #[inline]
    fn view(&self) -> TextureView {
        TextureView::Owned(Box::new(self.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: None,
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            array_layer_count: None,
            base_array_layer: 0,
            mip_level_count: None,
            base_mip_level: 0,
        })))
    }

    #[inline]
    fn any(&self) -> &dyn Any {
        self
    }
}

#[inline]
fn ops<V: Copy>(ops: &Operations<V>) -> wgpu::Operations<V> {
    wgpu::Operations {
        load: match ops.load {
            LoadOp::Clear(v) => wgpu::LoadOp::Clear(v),
            LoadOp::Load => wgpu::LoadOp::Load,
        },
        store: ops.store,
    }
}

impl CommandEncoderTrait for wgpu::CommandEncoder {
    #[inline]
    fn begin_render_pass<'a>(&'a mut self, desc: &RenderPassDescriptor<'a, '_>) -> RenderPass<'a> {
        let attachments = desc
            .color_attachments
            .iter()
            .map(|attachment| wgpu::RenderPassColorAttachment {
                view: attachment.view.any().downcast_ref().unwrap(),
                resolve_target: attachment
                    .resolve_target
                    .map(|v| v.any().downcast_ref().unwrap()),
                ops: ops(&attachment.ops),
            })
            .collect::<Vec<_>>();

        let desc = wgpu::RenderPassDescriptor {
            label: desc.label,
            color_attachments: &attachments,
            depth_stencil_attachment: desc.depth_stencil_attachment.as_ref().map(|attachment| {
                wgpu::RenderPassDepthStencilAttachment {
                    view: attachment.view.any().downcast_ref().unwrap(),
                    depth_ops: attachment.depth_ops.as_ref().map(|ops| self::ops(ops)),
                    stencil_ops: attachment.stencil_ops.as_ref().map(|ops| self::ops(ops)),
                }
            }),
        };
        let render_pass = self.begin_render_pass(&desc);
        RenderPass(Box::new(render_pass))
    }

    #[inline]
    fn any(&self) -> &dyn Any {
        self
    }
}

impl<'a> RenderPassTrait<'a> for wgpu::RenderPass<'a> {
    #[inline]
    fn set_bind_group(&mut self, index: u32, bind_group: &'a BindGroup, offsets: &[u32]) {
        self.set_bind_group(index, bind_group.0.downcast_ref().unwrap(), offsets);
    }

    #[inline]
    fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        self.set_pipeline(pipeline.0.downcast_ref().unwrap())
    }

    #[inline]
    fn set_blend_constant(&mut self, color: wgpu_types::Color) {
        self.set_blend_constant(color);
    }

    #[inline]
    fn set_index_buffer(
        &mut self,
        buffer_slice: BufferSlice<'a>,
        index_format: wgpu_types::IndexFormat,
    ) {
        self.set_index_buffer(downcast_buffer_slice(buffer_slice), index_format);
    }

    #[inline]
    fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
        self.set_vertex_buffer(slot, downcast_buffer_slice(buffer_slice));
    }

    #[inline]
    fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        self.draw(vertices, instances);
    }

    #[inline]
    fn draw_indexed(
        &mut self,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        self.draw_indexed(indices, base_vertex, instances);
    }
}

impl SwapChainTextureTrait for wgpu::SwapChainTexture {
    fn view(&self) -> TextureView {
        TextureView::Borrowed(&self.view)
    }
}
