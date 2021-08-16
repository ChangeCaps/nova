pub mod key;
pub mod mouse_button;

use glam::Vec2;
use key::Key;
use mouse_button::MouseButton;
use nova_core::{
    plugin::Plugin,
    system::System,
    world::{SystemWorld, World},
};
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

impl<T: Ord + Send + Sync + 'static> System for Input<T> {
    #[inline]
    fn post_update(&mut self, _world: &mut SystemWorld) {
        self.clear();
    }
}

#[derive(Clone, Default)]
pub struct TextInput {
    pub text: Vec<String>,
}

#[derive(Clone, Default)]
pub struct Mouse {
    pub position: Vec2,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    #[inline]
    fn build(self, world: &mut World) {
        world.register_system::<Input<Key>>();
        world.register_system::<Input<MouseButton>>();
        world.register_resource::<Mouse>();
    }
}
