#![deny(unsafe_op_in_unsafe_fn)]

mod build;
mod egui_system;
mod load;
mod main_ui;
mod project;
mod view;
mod world_system;

use std::path::PathBuf;

use build::BuildSystem;
use egui_system::EguiSystem;
use load::Game;
use nova_assets::Assets;
use nova_engine::app::App;
use nova_render::render_texture::RenderTexture;
use nova_wgpu::TextureView;
use project::Project;
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

    let mut app = App::new();
    let mut world = app.world();

    world.insert_resource(Project::load(&opts.path)?);
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
