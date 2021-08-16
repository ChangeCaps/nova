use std::ops::{Deref, DerefMut};

use crossbeam::sync::{ShardedLockReadGuard, ShardedLockWriteGuard};

pub mod component;
pub mod node;
pub mod plugin;
pub mod resources;
pub mod system;
pub mod world;

pub struct Read<'a, T: ?Sized>(pub(crate) ShardedLockReadGuard<'a, Box<T>>);

impl<'a, T: ?Sized> Deref for Read<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub struct Write<'a, T: ?Sized>(pub(crate) ShardedLockWriteGuard<'a, Box<T>>);

impl<'a, T: ?Sized> Deref for Write<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<'a, T: ?Sized> DerefMut for Write<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

pub struct IsEditor;
