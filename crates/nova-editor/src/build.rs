use crate::{
    load::Game,
    project::{Project, ProjectPath},
};
use cargo_toml::Manifest;
use libloading::library_filename;
use nova_core::{system::System, world::SystemWorld};
use std::{
    io,
    path::Path,
    process::{Child, Command, Stdio},
};

fn verify_crate_type(manifest: &Manifest) -> Result<(), ()> {
    let lib = manifest.lib.as_ref().ok_or(())?;
    let crate_type = lib.crate_type.as_ref().ok_or(())?;
    println!("{:?}", crate_type);

    if crate_type.iter().find(|ty| *ty == "rlib").is_some() {
        Ok(())
    } else {
        Err(())
    }
}

#[derive(Default)]
pub struct BuildSystem {
    process: Option<Child>,
    pub release: bool,
}

impl BuildSystem {
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

impl System for BuildSystem {
    #[inline]
    fn update(&mut self, world: &mut SystemWorld) {
        if let Some(process) = &mut self.process {
            if let Some(exit_status) = process.try_wait().unwrap() {
                let output = self.process.take().unwrap().wait_with_output().unwrap();

                if !exit_status.success() {
                    log::error!("failed to build game");
                    log::error!("{}", String::from_utf8_lossy(&output.stderr));
                    return;
                }

                let project_path = world.read_resource::<ProjectPath>().unwrap();
                let mut project = world.write_resource::<Project>().unwrap();

                if !project.update(&project_path.0) {
                    return;
                }

                let mut game = world.write_resource::<Game>().unwrap();

                let manifest = match Manifest::from_path(
                    project_path.dir().join(&project.build.manifest_path),
                ) {
                    Ok(manifest) => manifest,
                    Err(e) => {
                        log::error!("failed to load Cargo.toml '{}'", e);
                        return;
                    }
                };

                let project_name = &manifest.package.as_ref().unwrap().name;
                let lib_name = project_name.replace('-', "_");

                log::info!("loading game lib");

                let target = if self.release { 
                    project_path
                        .dir()
                        .join(&project.build.target_dir)
                        .join("release")
                } else {
                    project_path
                        .dir()
                        .join(&project.build.target_dir)
                        .join("debug")
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

                match unsafe { game.init(world, Some(&project_path.dir().join("scene.scn"))) } {
                    Ok(_) => log::info!("initialized game"),
                    Err(err) => log::error!("failed to initialize game '{}'", err),
                }
            }
        }
    }
}
