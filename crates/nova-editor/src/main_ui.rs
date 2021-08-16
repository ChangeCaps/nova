use std::path::Path;

use egui::*;
use glam::UVec2;
use nova_assets::Assets;
use nova_core::world::SystemWorld;
use nova_render::render_texture::RenderTexture;
use nova_scene::WorldSerializer;
use nova_wgpu::{Instance, TextureView};

use crate::{
    build::BuildSystem,
    load::Game,
    project::{Project, ProjectPath},
    scenes::Scenes,
    view::{View, PRIMARY_VIEW},
    world_system::WorldSystem,
};

fn save(world: &SystemWorld) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = world.read_resource::<ProjectPath>().unwrap();
    let mut project = world.write_resource::<Project>().unwrap();

    project.try_update(&project_path.0)?;
    project.write(&project_path.0)?;

    let mut world_system = world.write_system::<WorldSystem>().unwrap();

    if let Some(world_instance) = &mut world_system.instance {
        let scenes = world.read_resource::<Scenes>().unwrap();
        let path = project_path.dir().join(scenes.open.as_ref().unwrap());

        let scene_string = ron::ser::to_string_pretty(
            &WorldSerializer {
                world: &world_instance.world.ref_world(),
                type_registry: &world_instance.type_registry,
            },
            Default::default(),
        )
        .unwrap();

        std::fs::write(&path, scene_string.as_bytes()).unwrap(); 
    }

    Ok(())
}

pub fn main_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    top_panel_ui(ctx, world);
    left_panel_ui(ctx, world);
    right_panel_ui(ctx, world);
    bottom_panel_ui(ctx, world);
    scene_panel_ui(ctx, world);
    main_panel_ui(ctx, world);

    let input = ctx.input();

    if input.modifiers.ctrl && input.key_pressed(Key::S) {
        if let Err(e) = save(world) {
            log::error!("failed to save: {}", e);
        }
    }
}

pub fn top_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let project_path = world.read_resource::<ProjectPath>().unwrap();
            let mut project = world.write_resource::<Project>().unwrap();

            if ui.button("File").clicked() {}

            let mut builder = world.write_system::<BuildSystem>().unwrap();

            if ui
                .add(Button::new("Build").enabled(!builder.is_building()))
                .clicked()
                && project.update(&project_path.0)
            {
                let mut world_system = world.write_system::<WorldSystem>().unwrap();
                world_system.unload();
                drop(world_system);

                let mut game = world.write_resource::<Game>().unwrap();
                // SAFETY: we just unloaded world_system
                unsafe { game.unload() };
                drop(game);

                let res = builder.build(
                    &project_path.dir().join(&project.build.manifest_path),
                    &project_path.dir().join(&project.build.target_dir),
                );

                if let Err(e) = res {
                    log::error!("failed to build game lib '{}'", e);
                }
            }

            let selected_text = if builder.release { "Release" } else { "Debug" };

            ComboBox::from_id_source("build_type")
                .width(70.0)
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut builder.release, false, "Debug");
                    ui.selectable_value(&mut builder.release, true, "Release");
                });

            let mut world_system = world.write_system::<WorldSystem>().unwrap();

            if let Some(world_instance) = &mut world_system.instance {
                let scenes = world.read_resource::<Scenes>().unwrap();
                let path = project_path.dir().join(scenes.open.as_ref().unwrap());

                if ui
                    .add(Button::new("Run").enabled(!world_instance.running))
                    .clicked()
                {
                    world_instance.running = true;
                }

                if ui
                    .add(Button::new("Stop").enabled(world_instance.running))
                    .clicked()
                {
                    world_instance.running = false;

                    drop(world_system);

                    let game = world.read_resource::<Game>().unwrap();
                    unsafe { game.init(world, Some(&path)).unwrap() };
                }
            }
        });
    });
}

fn show_dir_ui(path: &Path, world: &SystemWorld, ui: &mut Ui) -> Result<(), std::io::Error> {
    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        if name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            dirs.push((name, path));
        } else {
            files.push((name, path));
        }
    }

    for (name, path) in dirs {
        let ret = ui.collapsing(name, |ui| show_dir_ui(&path, world, ui));

        if let Some(ret) = ret.body_returned {
            ret?;
        }
    }

    for (name, _path) in files {
        ui.add(Label::new(name).sense(Sense::click()));
    }

    Ok(())
}

pub fn left_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            let project_path = world.read_resource::<ProjectPath>().unwrap();
            let mut project = world.write_resource::<Project>().unwrap();

            if !project.update(&project_path.0) {
                return;
            }

            ui.separator();

            ui.add(
                TextEdit::singleline(&mut project.package.name)
                    .text_style(TextStyle::Heading)
                    .frame(false),
            );

            ui.separator();

            let res =
                ScrollArea::auto_sized().show(ui, |ui| show_dir_ui(&project_path.dir(), world, ui));

            if let Err(e) = res {
                log::error!("error showing files: {}", e);
            }
        });
}

pub fn right_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.separator();
        });
}

pub fn bottom_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {});
}

pub fn scene_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    let mut scenes = world.write_resource::<Scenes>().unwrap();

    let scenes = &mut *scenes;
    let open = if let Some(open) = &mut scenes.open {
        open
    } else {
        return;
    };
    let loaded = &scenes.loaded;

    TopBottomPanel::top("scene_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for scene in loaded {
                ui.selectable_value(
                    open,
                    scene.clone(),
                    scene.file_name().unwrap().to_str().unwrap(),
                );
            }
        });
    });
}

pub fn main_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    CentralPanel::default().show(ctx, |ui| {
        let mut render_textures = world.write_system::<Assets<RenderTexture>>().unwrap();
        let views = world.read_system::<Assets<View>>().unwrap();
        let view = views.get(&PRIMARY_VIEW).unwrap();

        let image_size = ui.available_size();
        let size = UVec2::new(image_size.x.ceil() as u32, image_size.y.ceil() as u32);

        let texture = render_textures.get_mut(&view.texture).unwrap();
        if texture.should_resize(size) {
            let instance = world.read_resource::<Instance>().unwrap();
            texture.resize(&instance, size);

            let mut textures = world.write_system::<Assets<TextureView>>().unwrap();
            *textures.get_mut(&view.texture.clone().cast()).unwrap() = texture.texture.view();
        }

        ui.image(
            TextureId::User(view.texture.clone().unwrap_id()),
            image_size,
        );
    });
}
