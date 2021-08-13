use bytemuck::{bytes_of, cast_slice};
use nova_core::{component::Component, node::Node, system::System, world::World};
use nova_transform::component::GlobalTransform;
use nova_wgpu::{Buffer, BufferDescriptor, BufferUsage, GpuSystem};

use crate::color::Color;

#[derive(Clone, Debug, Default)]
pub struct PointLight {
    pub color: Color,
    pub intensity: f32,
}

impl Component for PointLight {
    #[inline]
    fn pre_update(&mut self, node: &Node, world: &World) {
        let position = node
            .component::<GlobalTransform>()
            .map(|t| t.0.translation)
            .unwrap_or_default();

        if let Some(mut lights) = world.system_mut::<LightsSystem>() {
            let light = PointLightRaw {
                position: position.into(),
                intensity: self.intensity,
                color: self.color.into(),
            };

            lights.point_lights.push(light);
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PointLightRaw {
    pub position: [f32; 3],
    pub intensity: f32,
    pub color: [f32; 4],
}

unsafe impl bytemuck::Zeroable for PointLightRaw {}
unsafe impl bytemuck::Pod for PointLightRaw {}

#[derive(Default)]
pub struct LightsSystem {
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub point_lights: Vec<PointLightRaw>,
    pub lights_buffer: Option<Buffer>,
}

impl System for LightsSystem {
    #[inline]
    fn post_update(&mut self, _world: &World) {
        self.point_lights.clear();
    }

    #[inline]
    fn update(&mut self, world: &World) {
        let gpu = world.system::<GpuSystem>().unwrap();

        let mut point_light_data = bytes_of(&self.ambient_color).to_vec();
        point_light_data.append(&mut bytes_of(&self.ambient_intensity).to_vec());
        point_light_data.append(&mut bytes_of(&(self.point_lights.len() as u32)).to_vec());
        point_light_data.append(&mut vec![0u8; 8]);

        point_light_data.append(&mut cast_slice(&self.point_lights).to_vec());

        if let Some(buffer) = &self.lights_buffer {
            gpu.instance.write_buffer(buffer, 0, &point_light_data);
        } else {
            let buffer = gpu.instance.create_buffer(&BufferDescriptor {
                label: Some("point_lights"),
                size: 16 + std::mem::size_of::<PointLightRaw>() as u64 * 64,
                usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation: false,
            });

            gpu.instance.write_buffer(&buffer, 0, &point_light_data);

            self.lights_buffer = Some(buffer);
        };
    }
}
