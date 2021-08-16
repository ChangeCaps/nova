use std::{
    collections::HashMap,
    hash::Hash,
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};

use nova_core::{system::System, world::SystemWorld};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InnerHandle {
    Id(u64),
    Path(PathBuf),
}

pub struct Handle<T: 'static> {
    inner: InnerHandle,
    marker: Option<Arc<PhantomData<&'static T>>>,
}

impl<T> Handle<T> {
    #[inline]
    pub const fn from_u64(id: u64) -> Self {
        Self {
            inner: InnerHandle::Id(id),
            marker: None,
        }
    }

    #[inline]
    pub fn from_inner(inner: InnerHandle) -> Self {
        Self {
            inner,
            marker: Some(Arc::new(PhantomData)),
        }
    }

    #[inline]
    pub fn cast<U: 'static>(self) -> Handle<U> {
        Handle {
            inner: self.inner,
            marker: Some(Arc::new(PhantomData)),
        }
    }

    #[inline]
    pub fn unwrap_id(self) -> u64 {
        match self.inner {
            InnerHandle::Id(id) => id,
            _ => panic!("tried to unwrap path handle"),
        }
    }

    #[inline]
    pub fn unwrap_path(self) -> PathBuf {
        match self.inner {
            InnerHandle::Path(path) => path,
            _ => panic!("tried to unwrap id handle"),
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

impl<T> PartialEq for Handle<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> PartialOrd for Handle<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> Ord for Handle<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Handle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Handle<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Handle::from_inner(InnerHandle::deserialize(deserializer)?))
    }
}

impl<T> From<&str> for Handle<T> {
    #[inline]
    fn from(path: &str) -> Self {
        Self {
            inner: InnerHandle::Path(path.into()),
            marker: Some(Arc::new(PhantomData)),
        }
    }
}

impl<T> From<PathBuf> for Handle<T> {
    #[inline]
    fn from(path: PathBuf) -> Self {
        Self {
            inner: InnerHandle::Path(path),
            marker: Some(Arc::new(PhantomData)),
        }
    }
}

impl<T> From<&Path> for Handle<T> {
    #[inline]
    fn from(path: &Path) -> Self {
        Self {
            inner: InnerHandle::Path(path.into()),
            marker: Some(Arc::new(PhantomData)),
        }
    }
}

impl<T> From<u64> for Handle<T> {
    #[inline]
    fn from(id: u64) -> Self {
        Self {
            inner: InnerHandle::Id(id),
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
    pub fn contains(&self, handle: &Handle<T>) -> bool {
        self.assets.contains_key(&handle.inner)
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
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.assets.values().map(|entry| &entry.asset)
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
    fn post_update(&mut self, _world: &mut SystemWorld) {
        self.clean()
    }
}
