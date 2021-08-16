use glam::UVec2;
use libloading::{Error, Library, Symbol};
use nova_assets::Assets;
use nova_core::world::{SystemWorld, World, WorldData};
use nova_input::InputPlugin;
use nova_render::{
    camera::{Camera, MainCamera},
    component::MeshInstance,
    light::{AmbientLight, PointLight},
    render_settings::RenderSettings,
    render_target::RenderTarget,
    render_texture::RenderTexture,
};
use nova_scene::Scene;
use nova_transform::component::{GlobalTransform, Parent, Transform};
use nova_type::TypeRegistry;
use nova_wgpu::Instance;
use nova_window::Windows;
use std::{fs::read_to_string, path::Path};

use crate::{
    view::{View, ViewWindow, PRIMARY_VIEW},
    world_system::{WorldInstance, WorldSystem},
};

pub struct LoadedGame {
    library: Library,
}

impl LoadedGame {
    #[inline]
    pub unsafe fn load(path: &Path) -> Result<Self, Error> {
        Ok(Self {
            library: unsafe { Library::new(path)? },
        })
    }

    #[inline]
    pub unsafe fn new(&self, world: &mut World) -> Result<(), Error> {
        let new: Symbol<unsafe fn(&mut World)> = unsafe { self.library.get(b"export_world")? };
        Ok(unsafe { new(world) })
    }

    #[inline]
    pub unsafe fn register_types(&self, type_registry: &mut TypeRegistry) -> Result<(), Error> {
        let register: Symbol<unsafe fn(&mut TypeRegistry)> =
            unsafe { self.library.get(b"register_types")? };
        Ok(unsafe { register(type_registry) })
    }
}

#[derive(Default)]
pub struct Game {
    pub loaded: Option<LoadedGame>,
}

impl Game {
    #[inline]
    pub unsafe fn load(&mut self, path: &Path) -> Result<(), Error> {
        self.loaded = Some(unsafe { LoadedGame::load(path)? });

        Ok(())
    }

    /// # Safety
    /// Insure everything created by [`Game`] is dropped before calling unload.
    #[inline]
    pub unsafe fn unload(&mut self) {
        if self.loaded.is_some() {
            log::info!("unloading game lib");
            self.loaded = None;
        }
    }

    #[inline]
    pub unsafe fn init(&self, world: &SystemWorld, scene: Option<&Path>) -> Result<(), Error> {
        if let Some(loaded) = &self.loaded {
            let mut type_registry = TypeRegistry::default();

            type_registry.register_serde_component::<Transform>();
            type_registry.register_serde_component::<GlobalTransform>();
            type_registry.register_serde_component::<MeshInstance>();
            type_registry.register_serde_component::<Camera>();
            type_registry.register_serde_component::<MainCamera>();
            type_registry.register_serde_component::<PointLight>();
            type_registry.register_serde_component::<Parent>();

            type_registry.register_serde_resource::<RenderSettings>();
            type_registry.register_serde_resource::<AmbientLight>();

            if let Err(err) = unsafe { loaded.register_types(&mut type_registry) } {
                log::info!("failed to register types '{}'", err);
            }

            let mut world_data = WorldData::new();

            let instance = world.read_resource::<Instance>().unwrap();

            let views = world.read_system::<Assets<View>>().unwrap();
            let view = views.get(&PRIMARY_VIEW).unwrap();
            let render_textures = world.read_system::<Assets<RenderTexture>>().unwrap();
            let render_texture = render_textures.get(&view.texture).unwrap();

            if let Some(path) = scene {
                if let Ok(ron_string) = read_to_string(path) {
                    let mut deserializer = ron::Deserializer::from_str(&ron_string).unwrap();
                    let scene = Scene::deserialize(&type_registry, &mut deserializer).unwrap();

                    scene.apply(&mut world_data.system_world());
                }
            }

            let mut new_world = world_data.world();

            unsafe { loaded.new(&mut new_world)? };
            new_world.with_plugin(InputPlugin);

            new_world.insert_resource((*instance).clone());
            new_world.insert_resource(RenderTarget::Texture {
                view: render_texture.texture.view(),
                desc: render_texture.desc.clone(),
            });
            new_world.insert_resource(Windows::new(ViewWindow::new(UVec2::new(32, 32))));

            // init
            world_data.init();

            let mut world_system = world.write_system::<WorldSystem>().unwrap();
            world_system.instance = Some(WorldInstance::new(type_registry, world_data));

            Ok(())
        } else {
            panic!("game not loaded");
        }
    }
}

impl Drop for Game {
    #[inline]
    fn drop(&mut self) {
        if self.loaded.is_some() {
            panic!("LoadedGame must be unloaded manually");
        }
    }
}
