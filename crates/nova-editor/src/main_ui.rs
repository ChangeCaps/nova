use std::{mem::ManuallyDrop, path::Path};

use egui::*;
use erased_serde::Serializer;
use glam::UVec2;
use nova_assets::Assets;
use nova_core::{AppBuilder, Resources, World};
use nova_render::{render_target::RenderTarget, render_texture::RenderTexture};
use nova_wgpu::{Instance, TextureView};

use crate::{
    build::Builder,
    load::Game,
    project::{Project, ProjectPath},
    scenes::{SceneInstance, Scenes},
    view::{View, PRIMARY_VIEW},
};

fn save(_world: &World, resources: &Resources) -> Result<(), Box<dyn std::error::Error>> {
    let builder = resources.get::<Builder>().unwrap();
    let scenes = resources.get::<Scenes>().unwrap();

    if let Some(open) = &scenes.open {
        let scene = scenes.instances.get(open).unwrap();

        let mut file = std::fs::File::create(open)?;

        let pretty_config = if builder.release {
            None
        } else {
            Some(Default::default())
        };

        let mut serializer = ron::ser::Serializer::new(&mut file, pretty_config, false)?;
        let mut serializer = <dyn Serializer>::erase(&mut serializer);

        (scene.app.serialize)(&scene.app.world, &scene.app.registry, &mut serializer)?;
    }

    Ok(())
}

pub fn main_ui(ctx: &CtxRef, world: &World, resources: &Resources) {
    top_panel_ui(ctx, world, resources);
    left_panel_ui(ctx, world, resources);
    right_panel_ui(ctx, world, resources);
    bottom_panel_ui(ctx, world, resources);
    scene_panel_ui(ctx, world, resources);
    main_panel_ui(ctx, world, resources);

    let input = ctx.input();

    if input.modifiers.ctrl && input.key_pressed(Key::S) {
        if let Err(e) = save(world, resources) {
            log::error!("failed to save: {}", e);
        }
    }
}

pub fn top_panel_ui(ctx: &CtxRef, world: &World, resources: &Resources) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let project_path = resources.get::<ProjectPath>().unwrap();
            let mut project = resources.get_mut::<Project>().unwrap();

            if ui.button("File").clicked() {}

            let mut builder = resources.get_mut::<Builder>().unwrap();

            if ui
                .add(Button::new("Build").enabled(!builder.is_building()))
                .clicked()
                && project.update(&project_path.0)
            {
                let mut scenes = resources.get_mut::<Scenes>().unwrap();

                scenes.unload();

                let mut game = resources.get_mut::<Game>().unwrap();
                // SAFETY: we just unloaded all scenes
                unsafe { game.unload() };
                drop(game);

                let res = builder.build(
                    &project_path.dir().join(&project.manifest_path()),
                    &project_path.dir().join(&project.target_dir()),
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

            let mut scenes_ref = resources.get_mut::<Scenes>().unwrap();
            let scenes = &mut *scenes_ref;

            if let Some(path) = &scenes.open {
                let run = ui
                    .add(Button::new("Run").enabled(!scenes.running))
                    .clicked();

                let stop = ui
                    .add(Button::new("Stop").enabled(scenes.running))
                    .clicked();

                if run {
                    scenes.running = true;
                }

                if stop {
                    scenes.running = false;

                    let app = AppBuilder::new();

                    let game = resources.get::<Game>().unwrap();
                    let instance = resources.get::<Instance>().unwrap();
                    let views = resources.get::<Assets<View>>().unwrap();
                    let textures = resources.get::<Assets<RenderTexture>>().unwrap();

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

                    let scene_instance = match SceneInstance::load(app, path) {
                        Ok(scene) => scene,
                        Err(err) => {
                            log::error!("failed to load scene: {}", err);
                            return;
                        }
                    };

                    scenes
                        .instances
                        .insert(path.clone(), ManuallyDrop::new(scene_instance));
                }

                if run {
                    let scene = &mut **scenes.instances.get_mut(path).unwrap();

                    (scene.app.update)(
                        &mut scene.app.startup_schedule,
                        &mut scene.app.world,
                        &mut scene.app.resources,
                    );

                    drop(scenes_ref);
                    drop(builder);

                    if let Err(err) = save(world, resources) {
                        log::error!("failed saving scene: {}", err);
                    }
                }
            }
        });
    });
}

fn show_dir_ui(
    path: &Path,
    world: &World,
    resources: &Resources,
    ui: &mut Ui,
) -> Result<(), std::io::Error> {
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
        let ret = ui.collapsing(name, |ui| show_dir_ui(&path, world, resources, ui));

        if let Some(ret) = ret.body_returned {
            ret?;
        }
    }

    for (name, _path) in files {
        ui.add(Label::new(name).sense(Sense::click()));
    }

    Ok(())
}

pub fn left_panel_ui(ctx: &CtxRef, world: &World, resources: &Resources) {
    SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            let project_path = resources.get::<ProjectPath>().unwrap();
            let mut project = resources.get_mut::<Project>().unwrap();

            if !project.update(&project_path.0) {
                return;
            }

            ui.separator();

            ui.label(&project.package.name);

            ui.separator();

            let res = ScrollArea::auto_sized().show(ui, |ui| {
                show_dir_ui(&project_path.dir(), world, resources, ui)
            });

            if let Err(e) = res {
                log::error!("error showing files: {}", e);
            }
        });
}

pub fn right_panel_ui(ctx: &CtxRef, world: &World, resources: &Resources) {
    SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.separator();
        });
}

pub fn bottom_panel_ui(ctx: &CtxRef, world: &World, resources: &Resources) {
    TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.separator();
        });
}

pub fn scene_panel_ui(ctx: &CtxRef, world: &World, resources: &Resources) {
    let mut scenes = resources.get_mut::<Scenes>().unwrap();

    let scenes = &mut *scenes;
    let open = if let Some(open) = &mut scenes.open {
        open
    } else {
        return;
    };
    let paths = scenes.instances.keys();

    TopBottomPanel::top("scene_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for scene in paths {
                ui.selectable_value(
                    open,
                    scene.clone(),
                    scene.file_name().unwrap().to_str().unwrap(),
                );
            }
        });
    });
}

pub fn main_panel_ui(ctx: &CtxRef, world: &World, resources: &Resources) {
    CentralPanel::default().show(ctx, |ui| {
        let mut render_textures = resources.get_mut::<Assets<RenderTexture>>().unwrap();
        let views = resources.get::<Assets<View>>().unwrap();
        let view = views.get(&PRIMARY_VIEW).unwrap();

        let image_size = ui.available_size();
        let size = UVec2::new(image_size.x.ceil() as u32, image_size.y.ceil() as u32);

        let texture = render_textures.get_mut(&view.texture).unwrap();
        if texture.should_resize(size) {
            let instance = resources.get::<Instance>().unwrap();
            texture.resize(&instance, size);

            let mut textures = resources.get_mut::<Assets<TextureView>>().unwrap();
            *textures.get_mut(&view.texture.clone().cast()).unwrap() = texture.texture.view();
        }

        ui.image(
            TextureId::User(view.texture.clone().unwrap_id()),
            image_size,
        );
    });
}
