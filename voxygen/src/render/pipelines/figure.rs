use super::{
    super::{util::arr_to_mat, Pipeline, DEPTH_FORMAT},
    Globals, GlobalsLayouts, Light, Shadow,
};
use vek::*;
use zerocopy::AsBytes;

// gfx_defines! {
//     vertex Vertex {
//         pos: [f32; 3] = "v_pos",
//         norm: [f32; 3] = "v_norm",
//         col: [f32; 3] = "v_col",
//         bone_idx: u8 = "v_bone_idx",
//     }

//     constant Locals {
//         model_mat: [[f32; 4]; 4] = "model_mat",
//         model_col: [f32; 4] = "model_col",
//     }

//     constant BoneData {
//         bone_mat: [[f32; 4]; 4] = "bone_mat",
//     }

//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),

//         locals: gfx::ConstantBuffer<Locals> = "u_locals",
//         globals: gfx::ConstantBuffer<Globals> = "u_globals",
//         bones: gfx::ConstantBuffer<BoneData> = "u_bones",
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
    pos: [f32; 3],
    norm: [f32; 3],
    col: [f32; 3],
    bone_idx: u32,
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
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress * 3,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint,
                },
            ],
        }
    }

    pub fn new(pos: Vec3<f32>, norm: Vec3<f32>, col: Rgb<f32>, bone_idx: u8) -> Self {
        Self {
            pos: pos.into_array(),
            col: col.into_array(),
            norm: norm.into_array(),
            bone_idx: bone_idx as u32,
        }
    }

    pub fn with_bone_idx(mut self, bone_idx: u8) -> Self {
        self.bone_idx = bone_idx as u32;
        self
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Locals {
    model_mat: [[f32; 4]; 4],
    model_col: [f32; 4],
}

impl Locals {
    pub fn new(model_mat: Mat4<f32>, col: Rgba<f32>) -> Self {
        Self {
            model_mat: arr_to_mat(model_mat.into_col_array()),
            model_col: col.into_array(),
        }
    }
}

impl Default for Locals {
    fn default() -> Self { Self::new(Mat4::identity(), Rgba::broadcast(1.0)) }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct BoneData {
    bone_mat: [[f32; 4]; 4],
}

impl BoneData {
    pub fn new(bone_mat: Mat4<f32>) -> Self {
        Self {
            bone_mat: arr_to_mat(bone_mat.into_col_array()),
        }
    }

    pub fn default() -> Self { Self::new(Mat4::identity()) }
}

pub struct FigurePipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub locals: wgpu::BindGroupLayout,
}

impl FigurePipeline {
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
            bindings: &[
                // Locals
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                // BoneData
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
            ],
        })
    }
}

impl Pipeline for FigurePipeline {
    type Vertex = Vertex;
}
