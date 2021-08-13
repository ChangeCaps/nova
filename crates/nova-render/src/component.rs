use std::collections::BTreeMap;

use bytemuck::cast_slice;
use glam::Mat4;
use nova_assets::{Assets, Handle};
use nova_core::{
    component::Component,
    node::{Node, NodeId},
    world::World,
};
use nova_transform::component::{GlobalTransform, Transform};
use nova_wgpu::*;

use crate::{
    camera::{Camera, CameraSystem},
    light::LightsSystem,
    mesh::MeshData,
    render_commands::RenderCommands,
    renderable::Renderable,
};

#[derive(Default)]
pub struct MeshInstance {
    pub mesh_data: Handle<MeshData>,
    pub pipeline: Handle<RenderPipeline>,
    pub bindings: BTreeMap<u32, BindGroup>,
    pub camera: Option<NodeId>,
    pub buffer: Option<Buffer>,
}

impl Component for MeshInstance {}

impl Renderable for MeshInstance {
    #[inline]
    fn pre_render(&mut self, node: &Node, world: &World) {
        let gpu = world.system::<GpuSystem>().unwrap();

        let transform = if let Some(transform) = node.component::<GlobalTransform>() {
            transform.0.clone()
        } else {
            Transform::IDENTITY
        };

        let camera = if let Some(camera) = self.camera {
            camera
        } else {
            world.system::<CameraSystem>().unwrap().main.unwrap()
        };

        let view_proj = if let Some(node) = world.node(&camera) {
            let view = if let Some(transform) = node.component::<GlobalTransform>() {
                transform.matrix()
            } else {
                Transform::IDENTITY.matrix()
            };

            let proj = if let Some(camera) = node.component::<Camera>() {
                camera.proj_matrix()
            } else {
                Mat4::perspective_infinite_rh(std::f32::consts::PI / 2.0, 1.0, 1.0)
            };

            proj * view.inverse()
        } else {
            Mat4::perspective_infinite_rh(std::f32::consts::PI / 2.0, 1.0, 1.0)
        };

        let buffer = if let Some(buffer) = &self.buffer {
            buffer
        } else {
            let buffer = gpu.instance.create_buffer(&BufferDescriptor {
                label: Some("mesh_instance_transform_view_proj"),
                size: 256 + 64,
                usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation: false,
            });

            self.buffer = Some(buffer);

            &self.buffer.as_ref().unwrap()
        };

        let data = &[
            transform.matrix(),
            Mat4::IDENTITY,
            Mat4::IDENTITY,
            Mat4::IDENTITY,
            view_proj,
        ];

        gpu.instance.write_buffer(buffer, 0, cast_slice(data));

        if !self.bindings.contains_key(&0) {
            let layout = gpu
                .instance
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("mesh_instance_transform_view_proj"),
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
                        BindGroupLayoutEntry {
                            binding: 2,
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

            let lights = world.system::<LightsSystem>().unwrap();

            let bind_group = gpu.instance.create_bind_group(&BindGroupDescriptor {
                label: Some("mesh_instance_transform_view_proj"),
                layout: &layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::Buffer(BufferBinding {
                            buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Buffer(BufferBinding {
                            buffer,
                            offset: 256,
                            size: None,
                        }),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::Buffer(BufferBinding {
                            buffer: lights.lights_buffer.as_ref().unwrap(),
                            offset: 0,
                            size: None,
                        }),
                    },
                ],
            });

            self.bindings.insert(0, bind_group);
        }
    }

    #[inline]
    fn render(&mut self, _node: &Node, world: &World, render_commands: &mut RenderCommands) {
        let meshes = world.system::<Assets<MeshData>>().unwrap();

        let mesh = if let Some(mesh) = meshes.get(&self.mesh_data) {
            mesh
        } else {
            return;
        };

        render_commands.set_pipeline(self.pipeline.clone());

        for (i, bind_group) in self.bindings.iter() {
            render_commands.set_bind_group(*i as u32, bind_group.clone(), &[] as &[u32]);
        }

        render_commands.set_mesh(self.mesh_data.clone());
        render_commands.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
    }
}
