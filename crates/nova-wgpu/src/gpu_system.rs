use std::sync::Arc;

use nova_core::system::System;

use crate::{instance::Instance, SwapChain};

pub struct GpuSystem {
    pub instance: Arc<dyn Instance + Send + Sync>,
    pub swapchain: Box<dyn SwapChain + Send + Sync>,
}

impl GpuSystem {
    #[inline]
    pub fn new<T, S>(instance: T, swapchain: S) -> Self
    where
        T: Instance + Send + Sync + 'static,
        S: SwapChain + Send + Sync + 'static,
    {
        Self {
            instance: Arc::new(instance),
            swapchain: Box::new(swapchain),
        }
    }
}

impl System for GpuSystem {}
