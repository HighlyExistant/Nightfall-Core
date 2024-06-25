use ash::vk;
pub use nightfall_core_macros::Vertex;
pub trait Vertex {
    fn binding(binding: u32, rate: ash::vk::VertexInputRate) -> vk::VertexInputBindingDescription;
    fn attributes(binding: u32) -> Vec<vk::VertexInputAttributeDescription>;
}