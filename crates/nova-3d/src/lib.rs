pub mod shape;
pub mod stage;

use glam::{Vec2, Vec3};
use nova_assets::{Assets, Handle};
use nova_core::{
    plugin::Plugin,
    system::System,
    world::{SystemWorld, World},
};
use nova_render::{
    camera_stage::CameraStage, color::Color, depth_stage::DepthStage, light_stage::LightStage,
    msaa_stage::MsaaStage, render_settings::RenderSettings, render_target::RenderTarget,
    renderer::RendererSystem, Vertex,
};
use nova_wgpu::*;

use crate::stage::D3PassStage;

pub const PBR_PIPELINE_HANDLE: Handle<RenderPipeline> = Handle::from_u64(1246823428346);

#[repr(C)]
#[derive(Clone, Copy, Vertex)]
pub struct Vertex3d {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub color: Color,
}

unsafe impl bytemuck::Zeroable for Vertex3d {}
unsafe impl bytemuck::Pod for Vertex3d {}

#[derive(Clone, Debug, Default)]
pub struct D3System {
    pub msaa: u32,
}

impl System for D3System {
    fn init(&mut self, world: &mut SystemWorld) {
        let instance = world.read_resource::<Instance>().unwrap();
        let target = world.read_resource::<RenderTarget>().unwrap();
        let mut render_system = world.write_system::<RendererSystem>().unwrap();
        let settings = world.read_resource::<RenderSettings>().unwrap();

        render_system.add_stage(DepthStage);
        render_system.add_stage(MsaaStage);
        render_system.add_stage(CameraStage);
        render_system.add_stage(LightStage::default());
        render_system.add_stage(D3PassStage::default());

        let shader_module = instance.create_shader_module(&ShaderModuleDescriptor {
            label: Some("pbr"),
            source: ShaderSource::Wgsl(include_str!("pbr.wgsl").into()),
            flags: ShaderFlags::all(),
        });

        let transform_view_proj = instance.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let layout = instance.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pbr"),
            bind_group_layouts: &[&transform_view_proj],
            push_constant_ranges: &[],
        });

        let pipeline = instance.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("pbr"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader_module,
                buffers: &[
                    Vertex3d::layout(),
                    VertexBufferLayout {
                        array_stride: 64,
                        step_mode: InputStepMode::Instance,
                        attributes: &[
                            VertexAttribute {
                                offset: 0,
                                shader_location: 4,
                                format: VertexFormat::Float32x4,
                            },
                            VertexAttribute {
                                offset: 16,
                                shader_location: 5,
                                format: VertexFormat::Float32x4,
                            },
                            VertexAttribute {
                                offset: 32,
                                shader_location: 6,
                                format: VertexFormat::Float32x4,
                            },
                            VertexAttribute {
                                offset: 48,
                                shader_location: 7,
                                format: VertexFormat::Float32x4,
                            },
                        ],
                    },
                ],
                entry_point: "main",
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                targets: &[ColorTargetState {
                    format: target.format(),
                    blend: None,
                    write_mask: ColorWrite::ALL,
                }],
                entry_point: "main",
            }),
            primitive: PrimitiveState {
                cull_mode: Some(Face::Back),
                ..Default::default()
            },
            multisample: MultisampleState {
                count: settings.msaa,
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
        });

        world
            .write_system::<Assets<RenderPipeline>>()
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
