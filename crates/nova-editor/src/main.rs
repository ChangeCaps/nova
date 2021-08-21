#![deny(unsafe_op_in_unsafe_fn)]

mod build;
mod egui_system;
mod load;
mod main_ui;
mod project;
mod scenes;
mod view;

use std::path::PathBuf;

use build::{build_system, Builder};
use egui_system::EguiPlugin;
use load::Game;
use main_ui::SelectedEntity;
use nova_assets::AssetsAppExt;
use nova_core::stage;
use nova_engine::run;
use nova_input::InputPlugin;
use nova_render::render_texture::RenderTexture;
use nova_wgpu::TextureView;
use project::{Project, ProjectPath};
use scenes::{scenes_system, Scenes};
use view::{View, ViewPlugin};

use clap::{crate_authors, crate_version, Clap};

#[derive(Clap)]
#[clap(author = crate_authors!(), version = crate_version!())]
struct Opts {
    /// Path to the root of the project or 'Nova.toml'.
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

    run("Nova Editor", |mut app| {
        app.add_thread_local_to_stage(stage::PRE_UPDATE, build_system);
        app.add_thread_local_to_stage(stage::UPDATE, scenes_system);

        app.insert_resource(project);
        app.insert_resource(ProjectPath(path));
        app.register_resource::<Game>();
        app.register_resource::<Scenes>();
        app.register_resource::<Builder>();
        app.register_resource::<SelectedEntity>();
        app.register_asset::<TextureView>();
        app.register_asset::<RenderTexture>();
        app.register_asset::<View>();

        app.with_plugin(ViewPlugin);
        app.with_plugin(InputPlugin);
        app.with_plugin(EguiPlugin::new(main_ui::main_ui));

        app.build()
    })
}
