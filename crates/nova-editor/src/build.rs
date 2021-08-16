use crate::{load::Game, project::Project};
use libloading::library_filename;
use nova_core::{system::System, world::SystemWorld};
use std::{
    io,
    path::Path,
    process::{Child, Command, Stdio},
};

#[derive(Default)]
pub struct BuildSystem {
    process: Option<Child>,
    pub release: bool,
}

impl BuildSystem {
    #[inline]
    pub fn build(&mut self, path: &Path) -> Result<(), io::Error> {
        log::info!("running build command");

        let mut command = Command::new("cargo");
        command.arg("build").arg("--lib");

        if self.release {
            command.arg("--release");
        }

        command
            .current_dir(path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = command.spawn()?;

        self.process = Some(child);

        log::info!("game building...");

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

                let project = world.read_resource::<Project>().unwrap();
                let mut game = world.write_resource::<Game>().unwrap();

                let project_name = &project.manifest.package.as_ref().unwrap().name;
                let lib_name = project_name.replace('-', "_");

                log::info!("loading game lib");

                let target = if self.release {
                    "target/release"
                } else {
                    "target/debug"
                };

                let lib_path = project.path.join(target).join(library_filename(lib_name));

                let res = unsafe { game.load(&lib_path) };

                match res {
                    Ok(_) => log::info!("loaded game lib"),
                    Err(err) => {
                        log::error!("failed to load game lib: '{:?}'", lib_path);
                        log::error!("'{}'", err);
                        return;
                    }
                }

                match unsafe { game.init(world, Some(&project.path.join("scene.scn"))) } {
                    Ok(_) => log::info!("initialized game"),
                    Err(err) => log::error!("failed to initialize game '{}'", err),
                }
            }
        }
    }
}
