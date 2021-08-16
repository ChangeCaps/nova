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
    project::Project,
    view::{View, PRIMARY_VIEW},
    world_system::WorldSystem,
};

pub fn main_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    top_panel_ui(ctx, world);
    left_panel_ui(ctx, world);
    main_panel_ui(ctx, world);
}

pub fn top_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let project = world.read_resource::<Project>().unwrap();

            if ui.button("File").clicked() {}

            let mut builder = world.write_system::<BuildSystem>().unwrap();

            if ui
                .add(Button::new("Build").enabled(!builder.is_building()))
                .clicked()
            {
                let mut world_system = world.write_system::<WorldSystem>().unwrap();
                world_system.unload();
                drop(world_system);

                let mut game = world.write_resource::<Game>().unwrap();
                // SAFETY: we just unloaded world_system
                unsafe { game.unload() };
                drop(game);

                builder.build(&project.path).unwrap();
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
                if ui
                    .add(Button::new("Run").enabled(!world_instance.running))
                    .clicked()
                {
                    let scene_string = ron::ser::to_string_pretty(
                        &WorldSerializer {
                            world: &world_instance.world.ref_world(),
                            type_registry: &world_instance.type_registry,
                        },
                        Default::default(),
                    )
                    .unwrap();

                    std::fs::write(project.path.join("scene.scn"), scene_string.as_bytes())
                        .unwrap();

                    world_instance.running = true;
                }

                if ui
                    .add(Button::new("Stop").enabled(world_instance.running))
                    .clicked()
                {
                    world_instance.running = false;

                    drop(world_system);

                    let game = world.read_resource::<Game>().unwrap();
                    unsafe {
                        game.init(world, Some(&project.path.join("scene.scn")))
                            .unwrap()
                    };
                }
            }
        });
    });
}

pub fn left_panel_ui(ctx: &CtxRef, world: &mut SystemWorld) {
    SidePanel::left("left_panel").show(ctx, |ui| {});
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
