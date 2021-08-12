use glam::{Vec2, Vec3, Vec4};
use nova_assets::{Assets, Handle};
use nova_core::{plugin::Plugin, system::System, world::World};
use nova_render::Vertex;
use nova_wgpu::*;

pub const PBR_PIPELINE_HANDLE: Handle<RenderPipeline> = Handle::new_from_u64(1246823428346);

#[repr(C)]
#[derive(Clone, Copy, Vertex)]
pub struct Vertex3d {
    pub position: Vec3,
    pub uv: Vec2,
    pub color: Vec4,
}

unsafe impl bytemuck::Zeroable for Vertex3d {}
unsafe impl bytemuck::Pod for Vertex3d {}

#[derive(Clone, Debug, Default)]
pub struct D3System;

impl System for D3System {
    fn init(&mut self, world: &World) {
        let gpu = world.system::<GpuSystem>().unwrap();

        let shader_module = gpu.instance.create_shader_module(&ShaderModuleDescriptor {
            label: Some("pbr"),
            source: ShaderSource::Wgsl(include_str!("pbr.wgsl").into()),
            flags: ShaderFlags::all(),
        });

        let transform_view_proj =
            gpu.instance
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("pbr"),
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStage::VERTEX_FRAGMENT,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStage::VERTEX_FRAGMENT,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let layout = gpu
            .instance
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("pbr"),
                bind_group_layouts: &[&transform_view_proj],
                push_constant_ranges: &[],
            });

        let pipeline = gpu
            .instance
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("pbr"),
                layout: Some(&layout),
                vertex: VertexState {
                    module: &shader_module,
                    buffers: &[Vertex3d::layout()],
                    entry_point: "main",
                },
                fragment: Some(FragmentState {
                    module: &shader_module,
                    targets: &[ColorTargetState {
                        format: gpu.swapchain.format(),
                        blend: None,
                        write_mask: ColorWrite::ALL,
                    }],
                    entry_point: "main",
                }),
                primitive: PrimitiveState::default(),
                multisample: MultisampleState::default(),
                depth_stencil: None,
            });

        world
            .system_mut::<Assets<RenderPipeline>>()
            .unwrap()
            .insert_untracked(PBR_PIPELINE_HANDLE, pipeline);
    }
}

pub struct D3Plugin;

impl Plugin for D3Plugin {
    fn build(self, world: &mut World) {
        world.register_system::<D3System>();
    }
}
