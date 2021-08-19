use std::collections::HashMap;

use bytemuck::cast_slice;
use glam::Mat4;
use nova_assets::{Assets, Handle};
use nova_core::{App, IntoQuery, Resources, World};
use nova_render::{
    camera_node::CameraNode,
    component::MeshInstance,
    depth_node::DepthNode,
    light_node::LightNode,
    mesh::MeshData,
    msaa_node::MsaaNode,
    render_node::{RenderData, RenderNode, Target},
    render_settings::RenderSettings,
    render_texture::RenderTexture,
};
use nova_transform::component::GlobalTransform;
use nova_wgpu::*;

#[derive(Clone, PartialEq, Eq, Hash)]
struct InstanceHandle {
    pipeline: Handle<RenderPipeline>,
    mesh_data: Handle<MeshData>,
}

#[derive(Default)]
struct InstanceGroup {
    transform: Vec<Mat4>,
}

struct InstanceData {
    buffer: Buffer,
    buffer_data: Vec<u8>,
    bind_group: BindGroup,
}

#[derive(Default)]
pub struct D3PassNode {
    groups: HashMap<InstanceHandle, InstanceGroup>,
    data: HashMap<InstanceHandle, InstanceData>,
}

impl RenderNode for D3PassNode {
    #[inline]
    fn run(
        &mut self,
        world: &World,
        resources: &Resources,
        target: &Target,
        render_data: &mut RenderData,
    ) {
        self.groups.clear();

        let settings = resources.get_mut::<RenderSettings>().unwrap().clone();
        let instance = resources.get::<Instance>().unwrap();

        let mut meshes = resources.get_mut::<Assets<MeshData>>().unwrap();
        let pipelines = resources.get::<Assets<RenderPipeline>>().unwrap();

        let color_attachment = if settings.msaa > 1 {
            let msaa_texture = render_data.get::<RenderTexture>(MsaaNode::TEXTURE).unwrap();

            RenderPassColorAttachment {
                view: &msaa_texture.view,
                resolve_target: Some(target.view),
                ops: Operations {
                    load: LoadOp::Clear(settings.clear.into()),
                    store: true,
                },
            }
        } else {
            RenderPassColorAttachment {
                view: target.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(settings.clear.into()),
                    store: true,
                },
            }
        };

        let depth_texture = render_data
            .get::<RenderTexture>(DepthNode::TEXTURE)
            .unwrap();

        let mut encoder = instance.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("3d pass"),
        });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("3d pass"),
            color_attachments: &[color_attachment],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_texture.view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        for (mesh_instance, global_transform) in
            <(&MeshInstance, &GlobalTransform)>::query().iter(world)
        {
            let handle = InstanceHandle {
                pipeline: mesh_instance.pipeline.clone(),
                mesh_data: mesh_instance.mesh_data.clone(),
            };

            self.groups
                .entry(handle)
                .or_default()
                .transform
                .push(global_transform.matrix());
        }

        for (handle, group) in &self.groups {
            let mesh_data = meshes.get_mut(&handle.mesh_data).unwrap();

            if mesh_data.vertex_buffer.is_none() {
                let buffer = instance.create_buffer_init(&BufferInitDescriptor {
                    label: Some("mesh_data_vertex"),
                    contents: &mesh_data.vertices,
                    usage: BufferUsage::COPY_DST | BufferUsage::VERTEX,
                });

                mesh_data.vertex_buffer = Some(buffer);
            }

            if mesh_data.index_buffer.is_none() {
                let buffer = instance.create_buffer_init(&BufferInitDescriptor {
                    label: Some("mesh_data_index"),
                    contents: cast_slice(&mesh_data.indices),
                    usage: BufferUsage::COPY_DST | BufferUsage::INDEX,
                });

                mesh_data.index_buffer = Some(buffer);
            }

            let data: &[u8] = cast_slice(&group.transform);

            if let Some(instance_data) = self.data.get_mut(handle) {
                if data.len() != instance_data.buffer_data.len() {
                    let buffer = instance.create_buffer_init(&BufferInitDescriptor {
                        label: Some("instance_group_vertex"),
                        contents: data,
                        usage: BufferUsage::COPY_DST | BufferUsage::VERTEX,
                    });

                    instance_data.buffer = buffer;
                    instance_data.buffer_data = data.to_vec();
                } else if data != instance_data.buffer_data {
                    instance.write_buffer(&instance_data.buffer, 0, data);
                    instance_data.buffer_data = data.to_vec();
                }
            } else {
                let buffer = instance.create_buffer_init(&BufferInitDescriptor {
                    label: Some("instance_group_vertex"),
                    contents: data,
                    usage: BufferUsage::COPY_DST | BufferUsage::VERTEX,
                });

                let layout = instance.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("instance_group_bind"),
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

                let bind_group = instance.create_bind_group(&BindGroupDescriptor {
                    label: Some("instance_group_bind"),
                    layout: &layout,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::Buffer(BufferBinding {
                                buffer: render_data.get::<Buffer>(CameraNode::BUFFER).unwrap(),
                                offset: 0,
                                size: None,
                            }),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Buffer(BufferBinding {
                                buffer: render_data.get::<Buffer>(LightNode::BUFFER).unwrap(),
                                offset: 0,
                                size: None,
                            }),
                        },
                    ],
                });

                self.data.insert(
                    handle.clone(),
                    InstanceData {
                        buffer,
                        buffer_data: data.to_vec(),
                        bind_group,
                    },
                );
            };
        }

        for (handle, group) in &self.groups {
            let data = self.data.get(handle).unwrap();
            let pipeline = pipelines.get(&handle.pipeline).unwrap();
            let mesh_data = meshes.get(&handle.mesh_data).unwrap();

            render_pass.set_pipeline(pipeline);

            let vertex_buffer = mesh_data.vertex_buffer.as_ref().unwrap().slice(..);
            render_pass.set_vertex_buffer(0, vertex_buffer);
            render_pass.set_vertex_buffer(1, data.buffer.slice(..));

            let index_buffer = mesh_data.index_buffer.as_ref().unwrap().slice(..);
            render_pass.set_index_buffer(index_buffer, IndexFormat::Uint32);

            render_pass.set_bind_group(0, &data.bind_group, &[]);

            render_pass.draw_indexed(
                0..mesh_data.indices.len() as u32,
                0,
                0..group.transform.len() as u32,
            );
        }

        drop(render_pass);

        instance.submit(encoder);
    }
}
