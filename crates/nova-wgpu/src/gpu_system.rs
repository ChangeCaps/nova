use nova_core::system::System;

use crate::instance::Instance;

pub struct GpuSystem {
    pub instance: Box<dyn Instance>,
}

impl GpuSystem {
    #[inline]
    pub fn new<T: Instance + 'static>(instance: T) -> Self {
        Self {
            instance: Box::new(instance),
        }
    }
}

impl System for GpuSystem {}
