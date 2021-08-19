use libloading::{Error, Library, Symbol};
use nova_core::{App, AppBuilder, Resources, World};
use nova_render::{
    render_node::Target, render_target::RenderTarget, render_texture::RenderTexture,
};
use nova_wgpu::Instance;
use std::path::Path;

use crate::view::ViewType;

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
    pub unsafe fn init(
        &self,
        app: AppBuilder,
        instance: Instance,
        render_target: RenderTarget,
    ) -> Result<App, Error> {
        let export_app: Symbol<unsafe fn(AppBuilder, Instance, RenderTarget) -> App> =
            unsafe { self.library.get(b"export_app")? };
        Ok(unsafe { export_app(app, instance, render_target) })
    }

    #[inline]
    pub unsafe fn render_view(
        &self,
        world: &mut World,
        resources: &mut Resources,
        target: &Target,
    ) -> Result<(), Error> {
        let render_view: Symbol<unsafe fn(&World, &Resources, &Target)> =
            unsafe { self.library.get(b"render_view")? };

        Ok(unsafe { render_view(world, resources, target) })
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
}

impl Drop for Game {
    #[inline]
    fn drop(&mut self) {
        if self.loaded.is_some() {
            panic!("LoadedGame must be unloaded manually");
        }
    }
}
