use std::{
    collections::HashMap,
    fs::read_to_string,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
};

use erased_serde::Deserializer;
use nova_core::{serialize::Canon, App, Resources, World};

pub fn scenes_system(_world: &mut World, resources: &mut Resources) {
    let mut scenes = resources.get_mut::<Scenes>().unwrap();
    let scenes = &mut *scenes;

    if !scenes.running {
        if let Some(open) = &scenes.open {
            let scene = &mut **scenes.instances.get_mut(open).unwrap();

            (scene.app.update)(
                &mut scene.app.editor_schedule,
                &mut scene.app.world,
                &mut scene.app.resources,
            );
        }

        return;
    }

    if let Some(open) = &scenes.open {
        let scene = &mut **scenes.instances.get_mut(open).unwrap();

        (scene.app.update)(
            &mut scene.app.schedule,
            &mut scene.app.world,
            &mut scene.app.resources,
        );
    }
}

pub struct SceneInstance {
    pub app: App,
}

impl SceneInstance {
    pub fn load(mut app: App, path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let scene_data = read_to_string(path)?;
        let mut deserializer = ron::Deserializer::from_str(&scene_data)?;
        let mut deserializer = <dyn Deserializer>::erase(&mut deserializer);

        (app.deserialize)(&mut app.world, &app.registry, &mut deserializer)?;

        Ok(Self { app })
    }

    /// For safety reasons, we must drop the app in the apps code.
    pub fn unload(self) {
        let unload = self.app.unload;
        unload(self.app);
    }
}

#[derive(Default)]
pub struct Scenes {
    pub instances: HashMap<PathBuf, ManuallyDrop<SceneInstance>>,
    pub open: Option<PathBuf>,
    pub running: bool,
}

impl Scenes {
    pub fn unload(&mut self) {
        for (_path, instance) in self.instances.drain() {
            ManuallyDrop::into_inner(instance).unload();
        }

        self.open = None;
        self.running = false;
    }
}
