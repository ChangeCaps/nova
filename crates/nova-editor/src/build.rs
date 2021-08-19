use crate::{
    load::Game,
    project::{Project, ProjectPath},
    scenes::{SceneInstance, Scenes},
    view::{View, PRIMARY_VIEW},
};
use cargo_toml::Manifest;
use libloading::library_filename;
use nova_assets::Assets;
use nova_core::{systems::Runnable, AppBuilder, Resources, SystemBuilder, World};
use nova_render::{render_target::RenderTarget, render_texture::RenderTexture};
use nova_wgpu::{Instance, PrimitiveState};
use std::{
    io,
    mem::ManuallyDrop,
    path::Path,
    process::{Child, Command, Stdio},
};

fn verify_crate_type(manifest: &Manifest) -> Result<(), ()> {
    let lib = manifest.lib.as_ref().ok_or(())?;
    let crate_type = lib.crate_type.as_ref().ok_or(())?;

    if crate_type.iter().find(|ty| *ty == "rlib").is_some() {
        Ok(())
    } else {
        Err(())
    }
}

#[derive(Default)]
pub struct Builder {
    process: Option<Child>,
    pub release: bool,
}

impl Builder {
    #[inline]
    pub fn build(&mut self, manifest_path: &Path, target_dir: &Path) -> Result<(), io::Error> {
        let manifest = match Manifest::from_path(manifest_path) {
            Ok(manifest) => manifest,
            Err(e) => {
                log::error!("failed to load Cargo.toml: {}", e);
                return Ok(());
            }
        };

        if verify_crate_type(&manifest).is_err() {
            log::error!("crate type must be \"cdylib\" {}", manifest_path.display());
            return Ok(());
        }

        let mut command = Command::new("cargo");
        command
            .arg("build")
            .arg("--lib")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--target-dir")
            .arg(target_dir);

        log::debug!("running build command: {:?}", command);

        if self.release {
            command.arg("--release");
        }

        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = command.spawn()?;

        self.process = Some(child);

        Ok(())
    }

    #[inline]
    pub fn is_building(&self) -> bool {
        self.process.is_some()
    }
}

pub fn build_system(_world: &mut World, resources: &mut Resources) {
    let mut builder = resources.get_mut::<Builder>().unwrap();
    let project_path = resources.get::<ProjectPath>().unwrap();
    let mut project = resources.get_mut::<Project>().unwrap();
    let mut game = resources.get_mut::<Game>().unwrap();
    let mut scenes = resources.get_mut::<Scenes>().unwrap();
    let instance = resources.get::<Instance>().unwrap();
    let views = resources.get::<Assets<View>>().unwrap();
    let textures = resources.get::<Assets<RenderTexture>>().unwrap();

    if let Some(process) = &mut builder.process {
        if let Some(exit_status) = process.try_wait().unwrap() {
            let output = builder.process.take().unwrap().wait_with_output().unwrap();

            if !exit_status.success() {
                log::error!("failed to build game");
                log::error!("{}", String::from_utf8_lossy(&output.stderr));
                return;
            }

            if !project.update(&project_path.0) {
                return;
            }

            let manifest =
                match Manifest::from_path(project_path.dir().join(project.manifest_path())) {
                    Ok(manifest) => manifest,
                    Err(e) => {
                        log::error!("failed to load Cargo.toml '{}'", e);
                        return;
                    }
                };

            let project_name = &manifest.package.as_ref().unwrap().name;
            let lib_name = project_name.replace('-', "_");

            log::info!("loading game lib");

            let target = if builder.release {
                project_path
                    .dir()
                    .join(project.target_dir())
                    .join("release")
            } else {
                project_path.dir().join(project.target_dir()).join("debug")
            };

            let lib_path = target.join(library_filename(lib_name));

            let res = unsafe { game.load(&lib_path) };

            match res {
                Ok(_) => log::info!("loaded game lib"),
                Err(err) => {
                    log::error!("failed to load game lib: '{:?}'", lib_path);
                    log::error!("'{}'", err);
                    return;
                }
            }

            if let Some(main_scene) = project.main_scene_path() {
                let path = project_path.dir().join(main_scene);

                let app = AppBuilder::new();

                let view = views.get(&PRIMARY_VIEW).unwrap();
                let texture = textures.get(&view.texture).unwrap();

                let target = RenderTarget::Texture {
                    view: texture.texture.view(),
                    desc: texture.desc.clone(),
                };

                let res = unsafe {
                    game.loaded
                        .as_ref()
                        .unwrap()
                        .init(app, instance.clone(), target)
                };

                let app = match res {
                    Ok(app) => app,
                    Err(err) => {
                        log::error!("failed to init App: {}", err);
                        return;
                    }
                };

                let scene_instance = match SceneInstance::load(app, &path) {
                    Ok(scene) => scene,
                    Err(err) => {
                        log::error!("failed to load scene: {}", err);
                        return;
                    }
                };

                scenes
                    .instances
                    .insert(path.clone(), ManuallyDrop::new(scene_instance));
                scenes.open = Some(path);
            }
        }
    }
}
