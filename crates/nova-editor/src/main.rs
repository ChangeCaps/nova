#![deny(unsafe_op_in_unsafe_fn)]

mod build;
mod egui_system;
mod load;
mod main_ui;
mod project;
mod view;
mod world_system;
mod scene;

use std::path::PathBuf;

use build::BuildSystem;
use egui_system::EguiSystem;
use load::Game;
use nova_assets::Assets;
use nova_engine::app::App;
use nova_render::render_texture::RenderTexture;
use nova_wgpu::TextureView;
use project::{Project, ProjectPath};
use view::{View, ViewSystem};
use world_system::WorldSystem;

use clap::{crate_authors, crate_version, Clap};

#[derive(Clap)]
#[clap(author = crate_authors!(), version = crate_version!())]
struct Opts {
    /// Path to the root of the project.
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    simple_logger::SimpleLogger::new()
        .with_module_level("gfx", log::LevelFilter::Error)
        .with_module_level("wgpu", log::LevelFilter::Error)
        .with_module_level("winit", log::LevelFilter::Error)
        .with_module_level("naga", log::LevelFilter::Error)
        .init()?;

    let mut app = App::new();
    let mut world = app.world();

    let mut path = std::fs::canonicalize(opts.path).unwrap();

    if path.is_dir() {
        path = path.join("Nova.toml");
    }

    let project = match Project::load(&path) {
        Ok(project) => match project {
            Some(project) => {
                log::info!("loaded project: {}", path.display());
                project
            }
            None => {
                log::info!("create new project: {}", path.display());
                Project::default()
            }
        },
        Err(e) => {
            log::error!("failed to load project: {}", e);
            return Ok(());
        }
    };

    project.write(&path)?;

    world.insert_resource(project);
    world.insert_resource(ProjectPath(path));
    world.register_resource::<Game>();
    world.register_system::<ViewSystem>();
    world.register_system::<BuildSystem>();
    world.register_system::<WorldSystem>();
    world.register_system::<Assets<TextureView>>();
    world.register_system::<Assets<RenderTexture>>();
    world.register_system::<Assets<View>>();

    world.insert_system(EguiSystem::new(main_ui::main_ui));

    app.with_title("Nova Editor").run()
}
