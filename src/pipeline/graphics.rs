use std::sync::Arc;

use ash::{vk::{self, GraphicsPipelineCreateInfo, PipelineBindPoint, PipelineDepthStencilStateCreateInfo, PipelineShaderStageCreateInfo, PipelineTessellationStateCreateInfo, PipelineVertexInputStateCreateInfo, PolygonMode, VertexInputAttributeDescription}, vk_bitflags_wrapped};

use crate::{device::{self, LogicalDevice}, error::VulkanError, image::SampleCountFlags};

use super::{cache::PipelineCache, compute::ComputePipeline, layout::PipelineLayout, shader::Shader, VulkanPipeline};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html>"]
pub struct CullModeFlags(pub(crate) u32);
vk_bitflags_wrapped!(CullModeFlags, u32);
impl CullModeFlags {
    pub const NONE: Self = Self(0);
    pub const FRONT: Self = Self(0b1);
    pub const BACK: Self = Self(0b10);
    pub const FRONT_AND_BACK: Self = Self(0x0000_0003);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(i32)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html>"]
pub enum FrontFace {
    #[default]
    CounterClockwise,
    Clockwise,
}

impl FrontFace {
}
#[derive(Default, Clone, Debug)]
struct GraphicsPipelineData {
    // DONT CHECK THIS IS FINE
    flags: Vec<vk::PipelineCreateFlags>,
    attributes: Vec<Vec<vk::VertexInputAttributeDescription>>,
    binding: Vec<Vec<vk::VertexInputBindingDescription>>,
    shaders: Vec<Vec<Arc<Shader>>>,
    // DONT CHECK THIS IS FINE
    vertex_input_state: Vec<vk::PipelineVertexInputStateCreateInfo>,
    // DONT CHECK THIS IS FINE
    viewport_state: Vec<vk::PipelineViewportStateCreateInfo>,
    // DONT CHECK THIS IS FINE
    rasterization_state: Vec<vk::PipelineRasterizationStateCreateInfo>,
    dynamic_state: Vec<vk::PipelineDynamicStateCreateInfo>,
    // DONT CHECK THIS IS FINE
    layout: Vec<vk::PipelineLayout>,
    // DONT CHECK THIS IS FINE
    render_pass: Vec<vk::RenderPass>,
    // DONT CHECK THIS IS FINE
    subpass: Vec<u32>,
    // DONT CHECK THIS IS FINE
    input_assembly_state: Vec<vk::PipelineInputAssemblyStateCreateInfo>,
    // DONT CHECK THIS IS FINE
    tessellation_state: Vec<PipelineTessellationStateCreateInfo>,
    // DONT CHECK THIS IS FINE
    multisample_state: Vec<vk::PipelineMultisampleStateCreateInfo>,
    color_blend: Vec<Vec<vk::PipelineColorBlendAttachmentState>>,
    color_blend_state: Vec<vk::PipelineColorBlendStateCreateInfo>,
    shader_stages: Vec<Vec<vk::PipelineShaderStageCreateInfo>>,
    depth_stencil_state: Vec<vk::PipelineDepthStencilStateCreateInfo>,
}
#[derive(Debug)]
pub struct GraphicsPipelineBuilder {
    // data: Box<GraphicsPipelineData>,
    data: GraphicsPipelineData,
    create_infos: Vec<vk::GraphicsPipelineCreateInfo>
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html>"]
pub enum TriangleTopology {
    List,
    Strip { restart: bool },
    ListWithAdjacency,
    StripWithAdjacency { restart: bool },
    Fan { restart: bool },
}
impl TriangleTopology {
    pub const fn primitive_restart_enable(&self) -> bool {
        match *self {
            Self::Fan { restart } => { restart }
            Self::List => { false }
            Self::ListWithAdjacency => { false }
            Self::Strip { restart } => { restart }
            Self::StripWithAdjacency { restart } => { restart }
        }
    }
    pub const fn primitive_topology(&self) -> vk::PrimitiveTopology {
        match self {
            Self::Fan { restart } => { vk::PrimitiveTopology::TRIANGLE_FAN }
            Self::List => { vk::PrimitiveTopology::TRIANGLE_LIST }
            Self::ListWithAdjacency => { vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY }
            Self::Strip { restart } => { vk::PrimitiveTopology::TRIANGLE_STRIP }
            Self::StripWithAdjacency { restart } => { vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html>"]
pub enum LineTopology {
    List,
    Strip { restart: bool },
    ListWithAdjacency,
    StripWithAdjacency { restart: bool },
}
impl LineTopology {
    pub const fn primitive_restart_enable(&self) -> bool {
        match *self {
            Self::List => { false }
            Self::ListWithAdjacency => { false }
            Self::Strip { restart } => { restart }
            Self::StripWithAdjacency { restart } => { restart }
        }
    }
    
    pub const fn primitive_topology(&self) -> vk::PrimitiveTopology {
        match *self {
            Self::List => { vk::PrimitiveTopology::LINE_LIST }
            Self::ListWithAdjacency => { vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY }
            Self::Strip { restart } => { vk::PrimitiveTopology::LINE_STRIP }
            Self::StripWithAdjacency { restart } => { vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY }
        }
    }
}
pub enum RasterizationMode {
    Fill { 
        topology: TriangleTopology,
        cull: CullModeFlags, 
        front: FrontFace,
    },
    Line { 
        topology: LineTopology,
        line_width: f32,
    },
    Point { 
    },
    Rect { 
        topology: TriangleTopology,
        cull: CullModeFlags, 
        front: FrontFace,
    },
}
impl RasterizationMode {
    pub const fn triangle(topology: TriangleTopology, cull: CullModeFlags, front: FrontFace) -> Self {
        Self::Fill { topology, cull, front }
    }
    pub const fn line(topology: LineTopology, line_width: f32) -> Self {
        RasterizationMode::Line { topology, line_width }
    }
    pub const fn point() -> Self {
        Self::Point {  }
    }
    pub const fn rect(topology: TriangleTopology, cull: CullModeFlags, front: FrontFace) -> Self {
        Self::Rect { topology, cull, front }
    }
    pub const fn get_polygon_mode(&self) -> vk::PolygonMode {
        match self {
            RasterizationMode::Fill { topology, cull, front } => { vk::PolygonMode::FILL }
            RasterizationMode::Line { topology, line_width } => { vk::PolygonMode::LINE }
            &RasterizationMode::Point {  } => { vk::PolygonMode::POINT }
            RasterizationMode::Rect { topology, cull, front } => { vk::PolygonMode::FILL_RECTANGLE_NV }
        }
    }
    pub const fn get_line_width(&self) -> f32 {
        match self {
            RasterizationMode::Fill { topology, cull, front } => { 1.0 }
            RasterizationMode::Line { topology, line_width } => { *line_width }
            RasterizationMode::Point {  } => { 1.0 }
            RasterizationMode::Rect { topology, cull, front } => { 1.0 }
        }
    }
    pub const fn get_culling(&self) -> vk::CullModeFlags {
        match self {
            RasterizationMode::Fill { topology, cull, front } => { vk::CullModeFlags::from_raw(cull.0) }
            RasterizationMode::Line { topology, line_width } => { vk::CullModeFlags::empty() }
            RasterizationMode::Point {  } => { vk::CullModeFlags::empty() }
            RasterizationMode::Rect { topology, cull, front } => { vk::CullModeFlags::empty() }
        }
    }
    pub const fn get_front_face(&self) -> vk::FrontFace {
        match self {
            RasterizationMode::Fill { topology, cull, front } => { vk::FrontFace::from_raw(*front as i32) }
            RasterizationMode::Line { topology, line_width } => { vk::FrontFace::CLOCKWISE }
            RasterizationMode::Point {  } => { vk::FrontFace::CLOCKWISE }
            RasterizationMode::Rect { topology, cull, front } => { vk::FrontFace::CLOCKWISE }
        }
    }
    pub const fn get_primitive_restart_enable(&self) -> bool {
        match self {
            RasterizationMode::Fill { topology, cull, front } => { topology.primitive_restart_enable() }
            RasterizationMode::Line { topology, line_width } => { topology.primitive_restart_enable() }
            RasterizationMode::Point {  } => { false }
            RasterizationMode::Rect { topology, cull, front } => { topology.primitive_restart_enable() }
        }
    }
    pub const fn get_primitive_topology(&self) -> vk::PrimitiveTopology {
        match self {
            RasterizationMode::Fill { topology, cull, front } => { topology.primitive_topology() }
            RasterizationMode::Line { topology, line_width } => { topology.primitive_topology() }
            RasterizationMode::Point {  } => { vk::PrimitiveTopology::POINT_LIST }
            RasterizationMode::Rect { topology, cull, front } => { topology.primitive_topology() }
        }
    }
}
pub struct PipelineColorBlendAttachmentState {

}
#[derive(Clone, Copy, Default)]
pub struct DepthBias {
    depth_bias_constant_factor: f32,
    depth_bias_slope_factor: f32,
}
impl GraphicsPipelineBuilder {
    pub fn new() -> Self {
        Self { data: GraphicsPipelineData::default(), create_infos: vec![] }
    }
    pub fn vertex_input_state(mut self, binding: Option<&Vec<vk::VertexInputBindingDescription>>, attribute: Option<&Vec<VertexInputAttributeDescription>>) -> Self {
        self.data.vertex_input_state.push(vk::PipelineVertexInputStateCreateInfo {
            vertex_attribute_description_count: attribute.map(|v|{ v.len() as u32 }).unwrap_or(0),
            p_vertex_attribute_descriptions: attribute.map(|v| {
                self.data.attributes.push(v.clone());
                self.data.attributes.last().unwrap().as_ptr()
            }
            ).unwrap_or(std::ptr::null()),
            vertex_binding_description_count: binding.map(|v|{ v.len() as u32 }).unwrap_or(0),
            p_vertex_binding_descriptions: binding.map(|v| {
                self.data.binding.push(v.clone());
                self.data.binding.last().unwrap().as_ptr()
            } ).unwrap_or(std::ptr::null()),
            ..Default::default()
        });
        self
    }
    pub fn pipeline_layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.data.layout.push(layout);
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.data.render_pass.push(render_pass);
        self
    }
    pub fn clear_shader_stages(mut self) -> Self {
        self.data.shader_stages.clear();
        self
    }
    pub fn viewport_state(mut self, state: vk::PipelineViewportStateCreateInfo) -> Self {
        self.data.viewport_state.push(state);
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {self.data.subpass.push(subpass); self }
    pub fn rasterization(mut self, mode: RasterizationMode, rasterizer_discard_enable: bool, depth_bias: Option<DepthBias>, depth_clamp: Option<f32>) -> Self {
        let mut raster_info = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: depth_clamp.is_some() as u32,
            rasterizer_discard_enable: rasterizer_discard_enable as u32,
            polygon_mode: mode.get_polygon_mode(),
            line_width: mode.get_line_width(),
            cull_mode: mode.get_culling(),
            front_face: mode.get_front_face(),
            depth_bias_enable: depth_bias.is_some() as u32,
            depth_bias_clamp: depth_clamp.unwrap_or_default(),
            depth_bias_constant_factor: depth_bias.as_ref().map(|v|v.depth_bias_constant_factor).unwrap_or_default(),
            depth_bias_slope_factor: depth_bias.as_ref().map(|v|v.depth_bias_slope_factor).unwrap_or_default(),
            ..Default::default()
        };
        let mut input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            primitive_restart_enable: mode.get_primitive_restart_enable() as u32,
            topology: mode.get_primitive_topology(),
            ..Default::default()
        };
        self.data.input_assembly_state.push(input_assembly);
        self.data.rasterization_state.push(raster_info);
        self
    }
    pub fn multisample_state(mut self, min_sample_shading: Option<f32>, sample_mask: Option<*const u32>, samples: SampleCountFlags, alpha_to_one: bool, alpha_to_coverage: bool) -> Self {
        let multisample_info = vk::PipelineMultisampleStateCreateInfo {
            sample_shading_enable: min_sample_shading.is_some() as u32,
            min_sample_shading: min_sample_shading.unwrap_or(1.0),
            rasterization_samples: vk::SampleCountFlags::from_raw(samples.0),
            alpha_to_coverage_enable: alpha_to_coverage as u32,
            alpha_to_one_enable: alpha_to_one as u32,
            p_sample_mask: sample_mask.unwrap_or(std::ptr::null()),
            ..Default::default()
        };
        self.data.multisample_state.push(multisample_info);
        self
    }
    pub fn color_blending(mut self, attachment: Vec<vk::PipelineColorBlendAttachmentState>, blend_constants: [f32; 4], logic_op: Option<vk::LogicOp>) -> Self {
        self.data.color_blend.push(attachment);
        let color_blend = self.data.color_blend.last().unwrap();
        let state = vk::PipelineColorBlendStateCreateInfo {
            p_attachments: color_blend.as_ptr(),
            attachment_count: color_blend.len() as u32,
            blend_constants,
            logic_op_enable: logic_op.is_some() as u32,
            logic_op: logic_op.unwrap_or_default(),
            ..Default::default()
        };
        self.data.color_blend_state.push(state);
        self
    }
    pub fn raw_rasterization(mut self, create_info: vk::PipelineRasterizationStateCreateInfo) -> Self {
        self.data.rasterization_state.push(create_info);
        self
    }
    pub fn depth_stencil_state(mut self, create_info: vk::PipelineDepthStencilStateCreateInfo) -> Self {
        self.data.depth_stencil_state.push(create_info);
        self
    }
    pub fn dynamic_states(mut self, dynamic: &[vk::DynamicState]) -> Self {
        self.data.dynamic_state.push(vk::PipelineDynamicStateCreateInfo {
            p_dynamic_states: dynamic.as_ptr(),
            dynamic_state_count: dynamic.len() as u32,
            ..Default::default()
        });
        self
    }
    pub fn create(mut self, flags: vk::PipelineCreateFlags, shaders: Vec<Arc<Shader>>) -> Self {
        self.data.shaders.push(shaders);
        let last = self.data.shaders.last().unwrap();
        let pipeline_info = last.iter().map(|val| val.stage).collect::<Vec<_>>();
        self.data.shader_stages.push(pipeline_info);
        let create_info = vk::GraphicsPipelineCreateInfo {
            flags,
            stage_count: last.len() as u32,
            p_stages: self.data.shader_stages.last().unwrap().as_ptr(),
            p_vertex_input_state: self.data.vertex_input_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_input_assembly_state: self.data.input_assembly_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_tessellation_state: self.data.tessellation_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_viewport_state: self.data.viewport_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_rasterization_state: self.data.rasterization_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_multisample_state: self.data.multisample_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_depth_stencil_state: self.data.depth_stencil_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_color_blend_state: self.data.color_blend_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            p_dynamic_state: self.data.dynamic_state.last().map(|v|v as *const _).unwrap_or(std::ptr::null()),
            layout: *self.data.layout.last().unwrap(),
            render_pass: *self.data.render_pass.last().unwrap(),
            subpass: self.data.subpass.last().cloned().unwrap_or_default(),
            base_pipeline_index: -1,
            ..Default::default()
        };
        self.create_infos.push(create_info);
        self
    }
    pub fn build<'a>(
        mut self, 
        device: Arc<LogicalDevice>, 
        layouts: &'a[Arc<PipelineLayout>]) -> Result<impl ExactSizeIterator<Item = GraphicsPipeline>+'a, VulkanError> {
        let mut new_shaders = self.data.shaders.into_iter();
        let pipelines = unsafe { device.device.create_graphics_pipelines(vk::PipelineCache::null(), &self.create_infos, None) }
            .unwrap()
            .into_iter()
            .enumerate()
            .map(move |(i, pipeline)|{
                let layout = layouts.get(i).unwrap_or_else(||{
                    layouts.last().unwrap()
                });
                unsafe {
                    GraphicsPipeline::from_handle(
                        layout.clone(), 
                        new_shaders.next().unwrap().clone(), 
                        pipeline
                    )
                }
            });
        Ok(pipelines)
    }
}
pub struct GraphicsPipeline {
    pub(crate) layout: Arc<PipelineLayout>,
    pub(crate) shaders: Vec<Arc<Shader>>,
    pub(crate) pipeline: vk::Pipeline,
}

impl GraphicsPipeline {
    pub unsafe fn from_handle(layout: Arc<PipelineLayout>, shaders: Vec<Arc<Shader>>, pipeline: vk::Pipeline) -> Self {
        Self { layout, shaders, pipeline }
    }
    pub fn bind(&self, command_buffer: vk::CommandBuffer) {
        unsafe { self.layout.device.device.cmd_bind_pipeline(command_buffer, PipelineBindPoint::GRAPHICS, self.pipeline) };
    }
    pub fn draw(&self, command_buffer: vk::CommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
        unsafe { self.layout.device.device.cmd_draw(command_buffer, vertex_count, instance_count, first_vertex, first_instance) }
    }
    pub fn draw_indexed(&self, command_buffer: vk::CommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
        unsafe { self.layout.device.device.cmd_draw_indexed(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) }
    }
}
impl VulkanPipeline for GraphicsPipeline {
    fn pipeline_handle(&self) -> vk::Pipeline {
        self.pipeline
    }
}
impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        unsafe { self.layout.device.device.destroy_pipeline(self.pipeline, None) }
    }
}