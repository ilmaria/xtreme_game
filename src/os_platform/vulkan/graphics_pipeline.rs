use ash::vk;
use ash::version::DeviceV1_0;
use glsl_to_spirv;

use std::ptr;
use std::error::Error;
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;

use super::Renderer;
use super::RendererError;
use super::Vertex;

impl Renderer {
    pub fn create_graphics_pipeline(&mut self) -> Result<&mut Renderer, Box<Error>> {
        let device = self.device.ok_or(RendererError::NoDevice)?;
        let render_pass = self.render_pass.ok_or(RendererError::NoRenderPass)?;
        let surface_resolution = self.surface_resolution.ok_or(
            RendererError::NoSurfaceResolution,
        )?;

        let entry_name = CString::new("main")?;

        let vert_shader_stage_info = {
            let vert_shader = create_shader_module(
                &device,
                "data/shaders/cube.vert",
                glsl_to_spirv::ShaderType::Vertex,
            )?;

            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PipelineShaderStageCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                module: vert_shader,
                p_name: entry_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::SHADER_STAGE_VERTEX_BIT,
            }
        };

        let frag_shader_stage_info = {
            let frag_shader = create_shader_module(
                &device,
                "data/shaders/cube.frag",
                glsl_to_spirv::ShaderType::Fragment,
            )?;

            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PipelineShaderStageCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                module: frag_shader,
                p_name: entry_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::SHADER_STAGE_FRAGMENT_BIT,
            }
        };

        let shader_stage_create_infos = [vert_shader_stage_info, frag_shader_stage_info];

        let vertex_input_state_info = {
            let attribute_descriptions = Vertex::attribute_descriptions();
            let binding_descriptions = Vertex::binding_descriptions();

            vk::PipelineVertexInputStateCreateInfo {
                s_type: vk::StructureType::PipelineVertexInputStateCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                vertex_attribute_description_count: attribute_descriptions.len() as u32,
                p_vertex_attribute_descriptions: attribute_descriptions.as_ptr(),
                vertex_binding_description_count: binding_descriptions.len() as u32,
                p_vertex_binding_descriptions: binding_descriptions.as_ptr(),
            }
        };

        let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::StructureType::PipelineInputAssemblyStateCreateInfo,
            flags: Default::default(),
            p_next: ptr::null(),
            primitive_restart_enable: 0,
            topology: vk::PrimitiveTopology::TriangleList,
        };

        let viewport_state_info = {
            let viewports = [
                vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: surface_resolution.width as f32,
                    height: surface_resolution.height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                },
            ];
            let scissors = [
                vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: surface_resolution.clone(),
                },
            ];

            vk::PipelineViewportStateCreateInfo {
                s_type: vk::StructureType::PipelineViewportStateCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                scissor_count: scissors.len() as u32,
                p_scissors: scissors.as_ptr(),
                viewport_count: viewports.len() as u32,
                p_viewports: viewports.as_ptr(),
            }
        };

        let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PipelineRasterizationStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            cull_mode: vk::CULL_MODE_BACK_BIT,
            depth_bias_clamp: 0.0,
            depth_bias_constant_factor: 0.0,
            depth_bias_enable: 0,
            depth_bias_slope_factor: 0.0,
            depth_clamp_enable: 0,
            front_face: vk::FrontFace::Clockwise,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::Fill,
            rasterizer_discard_enable: 0,
        };

        let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PipelineMultisampleStateCreateInfo,
            flags: Default::default(),
            p_next: ptr::null(),
            rasterization_samples: vk::SAMPLE_COUNT_1_BIT,
            sample_shading_enable: 0,
            min_sample_shading: 1.0,
            p_sample_mask: ptr::null(),
            alpha_to_one_enable: 0,
            alpha_to_coverage_enable: 0,
        };

        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::Keep,
            pass_op: vk::StencilOp::Keep,
            depth_fail_op: vk::StencilOp::Keep,
            compare_op: vk::CompareOp::Always,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };

        let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PipelineDepthStencilStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: vk::CompareOp::LessOrEqual,
            depth_bounds_test_enable: 0,
            stencil_test_enable: 0,
            front: noop_stencil_state.clone(),
            back: noop_stencil_state.clone(),
            max_depth_bounds: 1.0,
            min_depth_bounds: 0.0,
        };

        let color_blend_state = {
            let color_blend_attachment_states = [
                vk::PipelineColorBlendAttachmentState {
                    blend_enable: 0,
                    src_color_blend_factor: vk::BlendFactor::SrcColor,
                    dst_color_blend_factor: vk::BlendFactor::OneMinusDstColor,
                    color_blend_op: vk::BlendOp::Add,
                    src_alpha_blend_factor: vk::BlendFactor::Zero,
                    dst_alpha_blend_factor: vk::BlendFactor::Zero,
                    alpha_blend_op: vk::BlendOp::Add,
                    color_write_mask: vk::ColorComponentFlags::all(),
                },
            ];

            vk::PipelineColorBlendStateCreateInfo {
                s_type: vk::StructureType::PipelineColorBlendStateCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                logic_op_enable: 0,
                logic_op: vk::LogicOp::Clear,
                attachment_count: color_blend_attachment_states.len() as u32,
                p_attachments: color_blend_attachment_states.as_ptr(),
                blend_constants: [0.0, 0.0, 0.0, 0.0],
            }
        };

        let pipeline_layout = {
            let layout_create_info = vk::PipelineLayoutCreateInfo {
                s_type: vk::StructureType::PipelineLayoutCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                set_layout_count: 0,
                p_set_layouts: ptr::null(),
                push_constant_range_count: 0,
                p_push_constant_ranges: ptr::null(),
            };

            device.create_pipeline_layout(&layout_create_info, None)?
        };

        let graphics_pipelines = {
            let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo {
                s_type: vk::StructureType::GraphicsPipelineCreateInfo,
                p_next: ptr::null(),
                flags: vk::PipelineCreateFlags::empty(),
                stage_count: shader_stage_create_infos.len() as u32,
                p_stages: shader_stage_create_infos.as_ptr(),
                p_vertex_input_state: &vertex_input_state_info,
                p_input_assembly_state: &vertex_input_assembly_state_info,
                p_tessellation_state: ptr::null(),
                p_viewport_state: &viewport_state_info,
                p_rasterization_state: &rasterization_info,
                p_multisample_state: &multisample_state_info,
                p_depth_stencil_state: &depth_state_info,
                p_color_blend_state: &color_blend_state,
                p_dynamic_state: ptr::null(),
                layout: pipeline_layout,
                render_pass: render_pass,
                subpass: 0,
                base_pipeline_handle: vk::Pipeline::null(),
                base_pipeline_index: 0,
            };

            device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[graphics_pipeline_create_info],
                    None,
                )
                .map_err(|err| "Unable to create graphics pipeline")?
        };

        self.graphics_pipeline = Some(graphics_pipelines[0]);

        Ok(self)
    }
}

fn create_shader_module(
    device: &DeviceV1_0,
    path: &str,
    shader_type: glsl_to_spirv::ShaderType,
) -> Result<vk::types::ShaderModule, Box<Error>> {
    let shader_code = {
        let shader_file = File::open(path)?;
        let mut code = String::new();
        shader_file.read_to_string(&mut code)?;
        code
    };

    let spv_file = glsl_to_spirv::compile(&shader_code, shader_type)?;

    let spv_code: Vec<u8> = spv_file.bytes().filter_map(|byte| byte.ok()).collect();

    let shader_info = vk::ShaderModuleCreateInfo {
        s_type: vk::StructureType::ShaderModuleCreateInfo,
        p_next: ptr::null(),
        flags: Default::default(),
        code_size: spv_code.len(),
        p_code: spv_code.as_ptr() as *const u32,
    };

    let shader_module = device.create_shader_module(&shader_info, None)?;
    Ok(shader_module)
}