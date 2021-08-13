use std::{collections::HashMap, marker::PhantomData, path::PathBuf, sync::Arc};

use nova_core::{system::System, world::World};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InnerHandle {
    Id(u64),
    Path(PathBuf),
}

pub struct Handle<T: 'static> {
    inner: InnerHandle,
    marker: Option<Arc<PhantomData<&'static T>>>,
}

impl<T> Default for Handle<T> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: InnerHandle::Id(0),
            marker: None,
        }
    }
}

impl<T> Clone for Handle<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T> Handle<T> {
    #[inline]
    pub const fn from_u64(id: u64) -> Self {
        Self {
            inner: InnerHandle::Id(id),
            marker: None,
        }
    }
}

impl<T, P: Into<PathBuf>> From<P> for Handle<T> {
    #[inline]
    fn from(path: P) -> Self {
        Self {
            inner: InnerHandle::Path(path.into()),
            marker: Some(Arc::new(PhantomData)),
        }
    }
}

pub struct AssetEntry<T: 'static> {
    asset: T,
    handle: Option<Arc<PhantomData<&'static T>>>,
}

pub struct Assets<T: 'static> {
    assets: HashMap<InnerHandle, AssetEntry<T>>,
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

impl<T: 'static> Assets<T> {
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
            marker: Some(Arc::new(PhantomData)),
        }
    }

    #[inline]
    pub fn add(&mut self, asset: T) -> Handle<T> {
        let handle = self.make_handle();

        self.assets.insert(
            handle.inner.clone(),
            AssetEntry {
                asset,
                handle: handle.marker.clone(),
            },
        );

        handle
    }

    #[inline]
    pub fn insert_untracked(&mut self, handle: impl Into<Handle<T>>, asset: T) -> Handle<T> {
        let handle = handle.into();

        self.assets.insert(
            handle.inner.clone(),
            AssetEntry {
                asset,
                handle: None,
            },
        );

        handle
    }

    #[inline]
    pub fn get(&self, handle: &Handle<T>) -> Option<&T> {
        self.assets.get(&handle.inner).map(|entry| &entry.asset)
    }

    #[inline]
    pub fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        self.assets
            .get_mut(&handle.inner)
            .map(|entry| &mut entry.asset)
    }

    #[inline]
    pub fn clean(&mut self) {
        self.assets.retain(|_id, entry| {
            entry
                .handle
                .as_mut()
                .map_or(true, |h| Arc::get_mut(h).is_none())
        })
    }
}

impl<T: Send + Sync + 'static> System for Assets<T> {
    #[inline]
    fn post_update(&mut self, _world: &World) {
        self.clean()
    }
}
