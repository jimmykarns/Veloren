use super::{
    super::{Pipeline, DEPTH_FORMAT},
    Globals, GlobalsLayouts, Light, Shadow,
};
use std::ops::Mul;
use vek::*;
use zerocopy::AsBytes;

// gfx_defines! {
//     vertex Vertex {
//         pos_norm: u32 = "v_pos_norm",
//         col_light: u32 = "v_col_light",
//     }

//     constant Locals {
//         model_offs: [f32; 3] = "model_offs",
//         load_time: f32 = "load_time",
//     }

//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),

//         locals: gfx::ConstantBuffer<Locals> = "u_locals",
//         globals: gfx::ConstantBuffer<Globals> = "u_globals",
//         lights: gfx::ConstantBuffer<Light> = "u_lights",
//         shadows: gfx::ConstantBuffer<Shadow> = "u_shadows",

//         noise: gfx::TextureSampler<f32> = "t_noise",

//         tgt_color: gfx::RenderTarget<TgtColorFmt> = "tgt_color",
//         tgt_depth: gfx::DepthTarget<TgtDepthFmt> =
// gfx::preset::depth::LESS_EQUAL_WRITE,     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
    pos_norm: u32,
    col_light: u32,
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Uint,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<u32>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint,
                },
            ],
        }
    }

    pub fn new(norm_bits: u32, light: u32, pos: Vec3<f32>, col: Rgb<f32>) -> Self {
        Self {
            pos_norm: 0
                | ((pos.x as u32) & 0x00FF) << 0
                | ((pos.y as u32) & 0x00FF) << 8
                | ((pos.z.max(0.0).min((1 << 13) as f32) as u32) & 0x1FFF) << 16
                | (norm_bits & 0x7) << 29,
            col_light: 0
                | ((col.r.mul(255.0) as u32) & 0xFF) << 8
                | ((col.g.mul(255.0) as u32) & 0xFF) << 16
                | ((col.b.mul(255.0) as u32) & 0xFF) << 24
                | (light & 0xFF) << 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Locals {
    pub model_offs: [f32; 3],
    pub load_time: f32,
}

impl Default for Locals {
    fn default() -> Self {
        Self {
            model_offs: [0.0; 3],
            load_time: 0.0,
        }
    }
}

pub struct TerrainPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub locals: wgpu::BindGroupLayout,
}

impl TerrainPipeline {
    pub fn new(
        device: &wgpu::Device,
        vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule,
        sc_desc: &wgpu::SwapChainDescriptor,
        layouts: &GlobalsLayouts,
    ) -> Self {
        let locals = Self::locals_layout(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&layouts.globals, &layouts.light, &layouts.shadow, &locals],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_read_mask: !0,
                stencil_write_mask: !0,
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            pipeline: render_pipeline,
            locals,
        }
    }

    fn locals_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            bindings: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        })
    }
}

impl Pipeline for TerrainPipeline {
    type Vertex = Vertex;
}
