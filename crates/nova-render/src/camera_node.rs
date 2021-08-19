use bytemuck::bytes_of;
use glam::Mat4;
use nova_core::{component, Entity, IntoQuery, Resources, Runnable, SystemBuilder, World};
use nova_transform::component::{GlobalTransform, Transform};
use nova_wgpu::{Buffer, BufferInitDescriptor, BufferUsage, Instance};

use crate::{
    camera::{Camera, Cameras, MainCamera},
    render_node::{RenderData, RenderNode, Target},
};

pub fn camera_system() -> impl Runnable {
    SystemBuilder::new("camera_system")
        .write_resource::<Cameras>()
        .with_query(<Entity>::query().filter(component::<MainCamera>() & component::<Camera>()))
        .build(|_commands, world, cameras, query| {
            cameras.main = query.iter(world).next().cloned();
        })
}

pub struct CameraNode;

impl CameraNode {
    pub const MATRIX: &'static str = "camera_matrix";
    pub const BUFFER: &'static str = "camera_matrix_buffer";
}

impl RenderNode for CameraNode {
    #[inline]
    fn run(
        &mut self,
        world: &World,
        resources: &Resources,
        target: &Target,
        data: &mut RenderData,
    ) {
        let camera_system = resources.get::<Cameras>().unwrap();

        let matrix = if let Some(main) = camera_system.main {
            let view = if let Ok(transform) = <&GlobalTransform>::query().get(world, main) {
                transform.matrix()
            } else {
                Transform::IDENTITY.matrix()
            };

            let proj = if let Ok(camera) = <&Camera>::query().get(world, main) {
                let mut camera = (*camera).clone();
                let aspect = target.size.x as f32 / target.size.y as f32;
                camera.set_aspect(aspect);

                camera.proj_matrix()
            } else {
                Mat4::IDENTITY
            };

            proj * view.inverse()
        } else {
            Mat4::IDENTITY
        };

        data.insert(Self::MATRIX, matrix);

        let instance = resources.get::<Instance>().unwrap();
        if let Some(buffer) = data.get::<Buffer>(Self::BUFFER) {
            instance.write_buffer(buffer, 0, bytes_of(&matrix));
        } else {
            let buffer = instance.create_buffer_init(&BufferInitDescriptor {
                label: Some("camera_buffer"),
                contents: bytes_of(&matrix),
                usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
            });

            data.insert(Self::BUFFER, buffer);
        }
    }
}
