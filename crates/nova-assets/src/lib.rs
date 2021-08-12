use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use nova_core::system::System;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InnerHandle {
    Id(u64),
    Path(PathBuf),
}

pub struct Handle<T: 'static> {
    inner: InnerHandle,
    _marker: PhantomData<&'static T>,
}

impl<T> Clone for Handle<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Handle<T> {
    #[inline]
    pub const fn new_from_u64(id: u64) -> Self {
        Self {
            inner: InnerHandle::Id(id),
            _marker: PhantomData,
        }
    }
}

impl<T, P: Into<PathBuf>> From<P> for Handle<T> {
    #[inline]
    fn from(path: P) -> Self {
        Self {
            inner: InnerHandle::Path(path.into()),
            _marker: PhantomData,
        }
    }
}

pub struct Assets<T> {
    assets: HashMap<InnerHandle, T>,
    next_id: u64,
}

impl<T> Default for Assets<T> {
    #[inline]
    fn default() -> Self {
        Self {
            assets: Default::default(),
            next_id: 0,
        }
    }
}

impl<T> Assets<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            assets: Default::default(),
            next_id: 0,
        }
    }

    #[inline]
    pub fn make_handle(&mut self) -> Handle<T> {
        while self.assets.contains_key(&InnerHandle::Id(self.next_id)) {
            self.next_id += 1;
        }

        let id = self.next_id;
        self.next_id += 1;

        Handle {
            inner: InnerHandle::Id(id),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn add(&mut self, asset: T) -> Handle<T> {
        let handle = self.make_handle();

        self.assets.insert(handle.inner.clone(), asset);

        handle
    }

    #[inline]
    pub fn insert_untracked(&mut self, handle: impl Into<Handle<T>>, asset: T) -> Handle<T> {
        let handle = handle.into();

        self.assets.insert(handle.inner.clone(), asset);

        handle
    }

    #[inline]
    pub fn get(&self, handle: &Handle<T>) -> Option<&T> {
        self.assets.get(&handle.inner)
    }

    #[inline]
    pub fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        self.assets.get_mut(&handle.inner)
    }
}

impl<T: Send + Sync + 'static> System for Assets<T> {}
