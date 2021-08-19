pub mod key;
pub mod mouse_button;

use glam::Vec2;
use key::Key;
use mouse_button::MouseButton;
use nova_core::{plugin::Plugin, stage, systems::Runnable, AppBuilder, SystemBuilder};
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Input<T> {
    pressed: BTreeSet<T>,
    down: BTreeSet<T>,
    released: BTreeSet<T>,
}

impl<T: Ord> Default for Input<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> Input<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            pressed: BTreeSet::new(),
            down: BTreeSet::new(),
            released: BTreeSet::new(),
        }
    }

    #[inline]
    pub fn press(&mut self, event: T)
    where
        T: Clone,
    {
        self.pressed.insert(event.clone());
        self.down.insert(event);
    }

    #[inline]
    pub fn release(&mut self, event: T) {
        self.down.remove(&event);
        self.released.insert(event);
    }

    #[inline]
    pub fn pressed(&self, event: &T) -> bool {
        self.pressed.contains(event)
    }

    #[inline]
    pub fn down(&self, event: &T) -> bool {
        self.down.contains(event)
    }

    #[inline]
    pub fn released(&self, event: &T) -> bool {
        self.released.contains(event)
    }

    #[inline]
    pub fn iter_pressed(&self) -> impl Iterator<Item = &T> {
        self.pressed.iter()
    }

    #[inline]
    pub fn iter_down(&self) -> impl Iterator<Item = &T> {
        self.down.iter()
    }

    #[inline]
    pub fn iter_released(&self) -> impl Iterator<Item = &T> {
        self.released.iter()
    }

    #[inline]
    pub fn all_pressed(&self) -> &BTreeSet<T> {
        &self.pressed
    }

    #[inline]
    pub fn all_down(&self) -> &BTreeSet<T> {
        &self.down
    }

    #[inline]
    pub fn all_released(&self) -> &BTreeSet<T> {
        &self.released
    }

    #[inline]
    pub fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }
}

#[derive(Clone, Default)]
pub struct TextInput {
    pub chars: Vec<char>,
}

#[derive(Clone, Default)]
pub struct Mouse {
    pub position: Vec2,
}

pub fn input_system() -> impl Runnable {
    SystemBuilder::new("input_system")
        .write_resource::<Input<Key>>()
        .write_resource::<Input<MouseButton>>()
        .write_resource::<TextInput>()
        .build(
            |_commands, _world, (key_input, mouse_input, text_input), _queries| {
                key_input.clear();
                mouse_input.clear();
                text_input.chars.clear();
            },
        )
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    #[inline]
    fn build(self, app: &mut AppBuilder) {
        app.register_resource::<Input<Key>>();
        app.register_resource::<Input<MouseButton>>();
        app.register_resource::<TextInput>();
        app.register_resource::<Mouse>();

        app.add_system_to_stage(stage::END, input_system());
        #[cfg(feature = "editor")]
        app.add_editor_system_to_stage(stage::END, input_system());
    }
}
