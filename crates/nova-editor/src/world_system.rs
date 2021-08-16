use nova_assets::Assets;
use nova_core::{
    system::System,
    world::{SystemWorld, WorldData},
};
use nova_input::{Input, Mouse, TextInput, key::Key, mouse_button::MouseButton};
use nova_render::{
    camera::CameraSystem, render_stage::Target, render_texture::RenderTexture,
    renderer::RendererSystem,
};
use nova_transform::system::TransformSystem;
use nova_type::TypeRegistry;

use crate::view::{View, ViewType};

pub struct WorldInstance {
    pub type_registry: TypeRegistry,
    pub world: WorldData,
    pub running: bool,
}

impl WorldInstance {
    #[inline]
    pub fn new(type_registry: TypeRegistry, world: WorldData) -> Self {
        Self {
            type_registry,
            world,
            running: false,
        }
    }
}

#[derive(Default)]
pub struct WorldSystem {
    pub instance: Option<WorldInstance>,
}

impl WorldSystem {
    #[inline]
    pub fn unload(&mut self) {
        if self.instance.is_some() {
            log::info!("unloading world instance");
            self.instance = None;
        }
    }
}

impl System for WorldSystem {
    #[inline]
    fn pre_update(&mut self, world: &mut SystemWorld) {
        let render_textures = world.read_system::<Assets<RenderTexture>>().unwrap();
        let views = world.read_system::<Assets<View>>().unwrap();

        if let Some(world_instance) = &mut self.instance {
            if world_instance.running {
                let key_input = world.read_resource::<Input<Key>>().unwrap();
                let mouse_input = world.read_resource::<Input<MouseButton>>().unwrap();
                let text_input = world.read_resource::<TextInput>().unwrap();
                let mouse = world.read_resource::<Mouse>().unwrap();

                let mut world = world_instance.world.world();

                if let Some(resource) = world.resource_mut::<Input<Key>>() {
                    *resource = key_input.clone();
                }

                if let Some(resource) = world.resource_mut::<Input<MouseButton>>() {
                    *resource = mouse_input.clone();
                }

                if let Some(resource) = world.resource_mut::<TextInput>() {
                    *resource = text_input.clone();
                }

                if let Some(resource) = world.resource_mut::<Mouse>() {
                    *resource = mouse.clone();
                }

                world_instance.world.pre_update();
                world_instance.world.update();
                world_instance.world.post_update();
            }

            let mut world = world_instance.world.system_world();

            if let Some(mut transform_system) = world.systems.write::<TransformSystem>() {
                transform_system.post_update(&mut world);
            }

            if let Some(mut render_system) = world.systems.write::<RendererSystem>() {
                for view in views.iter() {
                    let target = render_textures.get(&view.texture).unwrap();

                    let target = Target {
                        view: &target.view,
                        size: target.size(),
                        format: target.desc.format,
                    };

                    match view.ty {
                        ViewType::Camera(id) => {
                            let mut camera_system = world.systems.write::<CameraSystem>().unwrap();

                            let main = camera_system.main;
                            camera_system.main = Some(id);

                            drop(camera_system);

                            render_system.render_view(&mut world, &target);

                            let mut camera_system = world.systems.write::<CameraSystem>().unwrap();
                            camera_system.main = main;
                        }
                        ViewType::MainCamera => {
                            render_system.render_view(&mut world, &target);
                        }
                    }
                }
            }
        }
    }
}
