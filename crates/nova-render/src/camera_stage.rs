use bytemuck::bytes_of;
use glam::Mat4;
use nova_core::world::SystemWorld;
use nova_transform::component::{GlobalTransform, Transform};
use nova_wgpu::{Buffer, BufferInitDescriptor, BufferUsage, Instance};

use crate::{
    camera::{Camera, CameraSystem},
    render_stage::{RenderData, RenderStage, Target},
};

pub struct CameraStage;

impl CameraStage {
    pub const MATRIX: &'static str = "camera_matrix";
    pub const BUFFER: &'static str = "camera_matrix_buffer";
}

impl RenderStage for CameraStage {
    #[inline]
    fn render(&mut self, world: &mut SystemWorld, target: &Target, data: &mut RenderData) {
        let camera_system = world.read_system::<CameraSystem>().unwrap();

        let matrix = if let Some(main) = camera_system.main {
            if let Some(node) = world.node(&main) {
                let view = if let Some(transform) = node.read_component::<GlobalTransform>() {
                    transform.matrix()
                } else {
                    Transform::IDENTITY.matrix()
                };

                let proj = if let Some(camera) = node.read_component::<Camera>() {
                    let mut camera = camera.clone();
                    let aspect = target.size.x as f32 / target.size.y as f32;
                    camera.set_aspect(aspect);

                    camera.proj_matrix()
                } else {
                    Mat4::IDENTITY
                };

                proj * view.inverse()
            } else {
                Mat4::IDENTITY
            }
        } else {
            Mat4::IDENTITY
        };

        data.insert(Self::MATRIX, matrix);

        drop(camera_system);

        let instance = world.read_resource::<Instance>().unwrap();
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
