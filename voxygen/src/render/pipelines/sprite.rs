use super::{
    super::{util::arr_to_mat, AaMode, Pipeline, DEPTH_FORMAT},
    Globals, GlobalsLayouts, Light, Shadow,
};
use vek::*;
use zerocopy::AsBytes;

// gfx_defines! {
//     vertex Vertex {
//         pos: [f32; 3] = "v_pos",
//         norm: [f32; 3] = "v_norm",
//         col: [f32; 3] = "v_col",
//     }

//     vertex Instance {
//         inst_mat0: [f32; 4] = "inst_mat0",
//         inst_mat1: [f32; 4] = "inst_mat1",
//         inst_mat2: [f32; 4] = "inst_mat2",
//         inst_mat3: [f32; 4] = "inst_mat3",
//         inst_col: [f32; 3] = "inst_col",
//         inst_wind_sway: f32 = "inst_wind_sway",
//     }

//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),
//         ibuf: gfx::InstanceBuffer<Instance> = (),

//         globals: gfx::ConstantBuffer<Globals> = "u_globals",
//         lights: gfx::ConstantBuffer<Light> = "u_lights",
//         shadows: gfx::ConstantBuffer<Shadow> = "u_shadows",

//         noise: gfx::TextureSampler<f32> = "t_noise",

//         tgt_color: gfx::BlendTarget<TgtColorFmt> = ("tgt_color",
// ColorMask::all(), gfx::preset::blend::ALPHA),         tgt_depth:
// gfx::DepthTarget<TgtDepthFmt> = gfx::preset::depth::LESS_EQUAL_WRITE,     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
    pos: [f32; 3],
    norm: [f32; 3],
    col: [f32; 3],
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
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress * 2,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }

    pub fn new(pos: Vec3<f32>, norm: Vec3<f32>, col: Rgb<f32>) -> Self {
        Self {
            pos: pos.into_array(),
            col: col.into_array(),
            norm: norm.into_array(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Instance {
    inst_mat0: [f32; 4],
    inst_mat1: [f32; 4],
    inst_mat2: [f32; 4],
    inst_mat3: [f32; 4],
    inst_col: [f32; 3],
    inst_wind_sway: f32,
}

impl Instance {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress * 2,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress * 3,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress * 4,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: (mem::size_of::<[f32; 4]>() * 4 + mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float,
                },
            ],
        }
    }

    pub fn new(mat: Mat4<f32>, col: Rgb<f32>, wind_sway: f32) -> Self {
        let mat_arr = arr_to_mat(mat.into_col_array());
        Self {
            inst_mat0: mat_arr[0],
            inst_mat1: mat_arr[1],
            inst_mat2: mat_arr[2],
            inst_mat3: mat_arr[3],
            inst_col: col.into_array(),
            inst_wind_sway: wind_sway,
        }
    }
}

impl Default for Instance {
    fn default() -> Self { Self::new(Mat4::identity(), Rgb::broadcast(1.0), 0.0) }
}

pub struct SpriteLayout;

impl SpriteLayout {
    pub fn new(_device: &wgpu::Device) -> Self {
        Self
    }
}

pub struct SpritePipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl SpritePipeline {
    pub fn new(
        device: &wgpu::Device,
        vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule,
        sc_desc: &wgpu::SwapChainDescriptor,
        layouts: &GlobalsLayouts,
        _layout: &SpriteLayout,
        aa_mode: AaMode,
    ) -> Self {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&layouts.globals, &layouts.light, &layouts.shadow],
            });

        let samples = match aa_mode {
            AaMode::None | AaMode::Fxaa => 1,
            // TODO: Ensure sampling in the shader is exactly between the 4 texels
            AaMode::SsaaX4 => 1,
            AaMode::MsaaX4 => 4,
            AaMode::MsaaX8 => 8,
            AaMode::MsaaX16 => 16,
        };

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
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
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
                vertex_buffers: &[Vertex::desc(), Instance::desc()],
            },
            sample_count: samples,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            pipeline: render_pipeline,
        }
    }
}

impl Pipeline for SpritePipeline {
    type Vertex = Vertex;
}
