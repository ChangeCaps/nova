use bytemuck::{bytes_of, cast_slice};
use glam::Vec3;
use nova_core::world::SystemWorld;
use nova_transform::component::GlobalTransform;
use nova_wgpu::{Buffer, BufferDescriptor, BufferUsage, Instance};

use crate::{
    light::{AmbientLight, PointLight, PointLightRaw},
    render_stage::{RenderData, RenderStage, Target},
};

#[derive(Default)]
pub struct LightStage {
    pub point_lights: Vec<PointLightRaw>,
}

impl LightStage {
    pub const BUFFER: &'static str = "light_stage_buffer";
}

impl RenderStage for LightStage {
    fn render(&mut self, world: &mut SystemWorld, _target: &Target, data: &mut RenderData) {
        self.point_lights.clear();

        for node in world.nodes() {
            if let Some(point_light) = node.read_component::<PointLight>() {
                let position = if let Some(transform) = node.read_component::<GlobalTransform>() {
                    transform.translation
                } else {
                    Vec3::ZERO
                };

                self.point_lights.push(PointLightRaw {
                    position: position.into(),
                    intensity: point_light.intensity,
                    color: point_light.color.into(),
                });
            }
        }

        let ambient = world.resource_mut::<AmbientLight>().unwrap().clone();
        let instance = world.read_resource::<Instance>().unwrap();

        let mut point_light_data = bytes_of(&ambient.color).to_vec();
        point_light_data.append(&mut bytes_of(&ambient.intensity).to_vec());
        point_light_data.append(&mut bytes_of(&(self.point_lights.len() as u32)).to_vec());
        point_light_data.append(&mut vec![0u8; 8]);

        point_light_data.append(&mut cast_slice(&self.point_lights).to_vec());

        if let Some(buffer) = data.get::<Buffer>(Self::BUFFER) {
            instance.write_buffer(buffer, 0, &point_light_data);
        } else {
            let buffer = instance.create_buffer(&BufferDescriptor {
                label: Some("point_lights"),
                size: 16 + std::mem::size_of::<PointLightRaw>() as u64 * 64,
                usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation: false,
            });

            instance.write_buffer(&buffer, 0, &point_light_data);

            data.insert(Self::BUFFER, buffer);
        };
    }
}
