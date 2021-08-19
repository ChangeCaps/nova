use bytemuck::{bytes_of, cast_slice};
use nova_core::{App, IntoQuery, Resources, World};
use nova_transform::component::GlobalTransform;
use nova_wgpu::{Buffer, BufferDescriptor, BufferUsage, Instance};

use crate::{
    light::{AmbientLight, PointLight, PointLightRaw},
    render_node::{RenderData, RenderNode, Target},
};

#[derive(Default)]
pub struct LightNode {
    pub point_lights: Vec<PointLightRaw>,
}

impl LightNode {
    pub const BUFFER: &'static str = "light_stage_buffer";
}

impl RenderNode for LightNode {
    fn run(
        &mut self,
        world: &World,
        resources: &Resources,
        _target: &Target,
        data: &mut RenderData,
    ) {
        self.point_lights.clear();

        for (point_light, transform) in <(&PointLight, &GlobalTransform)>::query().iter(world) {
            self.point_lights.push(PointLightRaw {
                position: transform.translation.into(),
                intensity: point_light.intensity,
                color: point_light.color.into(),
            });
        }

        let ambient = resources.get::<AmbientLight>().unwrap();
        let instance = resources.get::<Instance>().unwrap();

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
