use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub mod component;
pub mod node;
pub mod plugin;
pub mod system;
pub mod world;

pub type Read<'a, T> = RwLockReadGuard<'a, T>;
pub type Write<'a, T> = RwLockWriteGuard<'a, T>;
