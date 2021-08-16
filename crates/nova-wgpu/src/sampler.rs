use std::{any::Any, num::NonZeroU8};

use wgpu_types::{AddressMode, CompareFunction, FilterMode, SamplerBorderColor};

#[derive(Clone, Debug)]
pub struct SamplerDescriptor<'a> {
    pub label: Option<&'a str>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: Option<NonZeroU8>,
    pub border_color: Option<SamplerBorderColor>,
}

impl<'a> Default for SamplerDescriptor<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            label: None,
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: std::f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}

pub struct Sampler(pub(crate) Box<dyn Any + Send + Sync>);
