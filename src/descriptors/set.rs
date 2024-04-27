use std::sync::Arc;

use ash::vk;

use super::{DescriptorLayout, DescriptorPool};
#[derive(Clone)]
pub struct DescriptorSetAllocation {
    set: vk::DescriptorSet,
    pool: Arc<DescriptorPool>,
    layout: Arc<DescriptorLayout>,
}

impl DescriptorSetAllocation {
    // the descriptor set must have come from the specified pool and layout
    pub(crate) fn new(set: vk::DescriptorSet, pool: Arc<DescriptorPool>, layout: Arc<DescriptorLayout>) -> Self {
        Self { set, pool, layout }
    }
    #[inline]
    pub fn set(&self) -> vk::DescriptorSet {
        self.set
    }
    #[inline]
    pub fn pool(&self) -> Arc<DescriptorPool> {
        self.pool.clone()
    }
    #[inline]
    pub fn layout(&self) -> Arc<DescriptorLayout> {
        self.layout.clone()
    }
}