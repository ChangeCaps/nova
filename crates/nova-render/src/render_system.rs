use crate::{
    component::MeshInstance,
    mesh::MeshData,
    render_commands::{RenderCommand, RenderCommands},
    render_texture::RenderTexture,
    renderable::Renderable,
};
use bytemuck::cast_slice;
use nova_assets::{Assets, Handle};
use nova_core::{component::Component, node::Node, system::System, world::World};
use nova_wgpu::*;
use nova_window::WindowSystem;
use std::{any::TypeId, collections::BTreeMap};

pub const MAIN_PASS_DEPTH: Handle<RenderTexture> = Handle::from_u64(28346982346);

#[derive(Default)]
pub struct RenderSystem {
    pre_render: BTreeMap<TypeId, Box<dyn Fn(&mut dyn Component, &Node, &World) + Send + Sync>>,
    render: BTreeMap<
        TypeId,
        Box<dyn Fn(&mut dyn Component, &Node, &World, &mut RenderCommands) + Send + Sync>,
    >,
}

impl RenderSystem {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn register_renderable<T: Renderable>(&mut self) {
        self.pre_render.insert(
            TypeId::of::<T>(),
            Box::new(|renderable, node, world| {
                renderable
                    .as_any_mut()
                    .downcast_mut::<T>()
                    .unwrap()
                    .pre_render(node, world);
            }),
        );

        self.render.insert(
            TypeId::of::<T>(),
            Box::new(|renderable, node, world, render_commands| {
                renderable.as_any_mut().downcast_mut::<T>().unwrap().render(
                    node,
                    world,
                    render_commands,
                );
            }),
        );
    }

    #[inline]
    pub fn render_view(&self, instance: &dyn Instance, world: &World, target: &TextureView) {
        world.nodes().for_each(|node| {
            for mut component in node.components.iter_mut_filtered() {
                if let Some(render_func) =
                    self.pre_render.get(&component.as_ref().as_any().type_id())
                {
                    render_func(component.as_mut(), &node, world);
                }
            }
        });

        let mut render_commands = RenderCommands::default();

        for node in world.nodes() {
            for mut component in node.components.iter_mut_filtered() {
                if let Some(render_func) = self.render.get(&component.as_ref().as_any().type_id()) {
                    render_func(component.as_mut(), &node, world, &mut render_commands);
                }
            }
        }

        let pipelines = world.system::<Assets<RenderPipeline>>().unwrap();
        let mut meshes = world.system_mut::<Assets<MeshData>>().unwrap();
        let mut textures = world.system_mut::<Assets<RenderTexture>>().unwrap();
        let window = world.system_mut::<WindowSystem>().unwrap();

        let depth_texture = textures.get_mut(&MAIN_PASS_DEPTH).unwrap();

        if depth_texture.size != window.window.size() {
            depth_texture.resize(instance, window.window.size());
        }

        let mut encoder = instance.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("render_system_encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_system_main_pass"),
            color_attachments: &[RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_texture.view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        for command in &render_commands.commands {
            match command {
                RenderCommand::SetMesh(mesh_handle) => { 
                    if let Some(mesh) = meshes.get_mut(mesh_handle) {
                        if mesh.vertex_buffer.is_none() {
                            let vertex_buffer =
                                instance.create_buffer_init(&BufferInitDescriptor {
                                    label: Some("mesh_data"),
                                    contents: &mesh.vertices,
                                    usage: BufferUsage::COPY_DST | BufferUsage::VERTEX,
                                });

                            let index_buffer = instance.create_buffer_init(&BufferInitDescriptor {
                                label: Some("mesh_data"),
                                contents: cast_slice(&mesh.indices),
                                usage: BufferUsage::COPY_DST | BufferUsage::INDEX,
                            });

                            mesh.vertex_buffer = Some(vertex_buffer);
                            mesh.index_buffer = Some(index_buffer);
                        }
                    }
                }
                _ => {}
            }
        }

        for command in &render_commands.commands {
            match command {
                RenderCommand::SetPipeline(pipeline_handle) => {
                    let pipeline = if let Some(pipeline) = pipelines.get(pipeline_handle) {
                        pipeline
                    } else {
                        continue;
                    };

                    render_pass.set_pipeline(pipeline);
                }
                RenderCommand::SetMesh(mesh_handle) => {
                    let mesh_data = if let Some(mesh) = meshes.get(&mesh_handle) {
                        mesh
                    } else {
                        continue;
                    };

                    let vertex_buffer = mesh_data.vertex_buffer.as_ref().unwrap();
                    let index_buffer = mesh_data.index_buffer.as_ref().unwrap();

                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
                }
                RenderCommand::SetBindGroup(index, bind_group, offsets) => {
                    render_pass.set_bind_group(*index, bind_group, offsets);
                }
                RenderCommand::DrawIndexed(indices, base_vertex, instances) => {
                    render_pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
                }
            }
        }

        drop(render_pass);

        instance.submit(encoder);
    }
}

impl System for RenderSystem {
    #[inline]
    fn init(&mut self, world: &World) {
        self.register_renderable::<MeshInstance>();

        let gpu = world.system::<GpuSystem>().unwrap();
        let window = world.system::<WindowSystem>().unwrap();
        let size = window.window.size();

        let texture = RenderTexture::new(gpu.instance.as_ref(), TextureFormat::Depth24Plus, size);

        world
            .system_mut::<Assets<RenderTexture>>()
            .unwrap()
            .insert_untracked(MAIN_PASS_DEPTH, texture);
    }

    #[inline]
    fn post_update(&mut self, world: &World) {
        let instance = world.system::<GpuSystem>().unwrap().instance.clone();
        let frame = world
            .system::<GpuSystem>()
            .unwrap()
            .swapchain
            .get_current_frame()
            .unwrap();

        let target_view = frame.output.view();

        self.render_view(instance.as_ref(), world, &target_view);
    }
}
