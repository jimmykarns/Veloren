use super::{
    super::{Pipeline, TerrainLocals, DEPTH_FORMAT},
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

//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),

//         locals: gfx::ConstantBuffer<TerrainLocals> = "u_locals",
//         globals: gfx::ConstantBuffer<Globals> = "u_globals",
//         lights: gfx::ConstantBuffer<Light> = "u_lights",
//         shadows: gfx::ConstantBuffer<Shadow> = "u_shadows",

//         noise: gfx::TextureSampler<f32> = "t_noise",
//         waves: gfx::TextureSampler<[f32; 4]> = "t_waves",

//         tgt_color: gfx::BlendTarget<TgtColorFmt> = ("tgt_color",
// ColorMask::all(), gfx::preset::blend::ALPHA),         tgt_depth:
// gfx::DepthTarget<TgtDepthFmt> = gfx::preset::depth::LESS_EQUAL_TEST,     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
    pos_norm: u32,
    col_light: u32,
}

impl Vertex {
    pub fn new(pos: Vec3<f32>, norm: Vec3<f32>, col: Rgb<f32>, light: f32, _opac: f32) -> Self {
        let (norm_axis, norm_dir) = norm
            .as_slice()
            .into_iter()
            .enumerate()
            .find(|(_i, e)| **e != 0.0)
            .unwrap_or((0, &1.0));
        let norm_bits = (norm_axis << 1) | if *norm_dir > 0.0 { 1 } else { 0 };

        Self {
            pos_norm: 0
                | ((pos.x as u32) & 0x00FF) << 0
                | ((pos.y as u32) & 0x00FF) << 8
                | ((pos.z.max(0.0).min((1 << 13) as f32) as u32) & 0x1FFF) << 16
                | ((norm_bits as u32) & 0x7) << 29,
            col_light: 0
                | ((col.r.mul(200.0) as u32) & 0xFF) << 8
                | ((col.g.mul(200.0) as u32) & 0xFF) << 16
                | ((col.b.mul(200.0) as u32) & 0xFF) << 24
                | ((light.mul(255.0) as u32) & 0xFF) << 0,
            //| ((opac.mul(0.4) as u32) & 0xFF) << 0,
        }
    }

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
}

// This is a marker type for the consts
#[derive(Copy, Clone, Debug, AsBytes)]
#[repr(C)]
pub struct Locals;

pub struct FluidLayout {
    pub locals: wgpu::BindGroupLayout,
}

impl FluidLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let locals = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                        //todo
                        component_type: wgpu::TextureComponentType::Float,
                        multisampled: false,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
        });
        Self {
            locals,
        }
    }

}

pub struct FluidPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl FluidPipeline {
    pub fn new(
        device: &wgpu::Device,
        vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule,
        sc_desc: &wgpu::SwapChainDescriptor,
        layouts: &GlobalsLayouts,
        terrain_layout: &super::terrain::TerrainLayout,
        layout: &FluidLayout,
    ) -> Self {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[
                    &layouts.globals,
                    &layouts.light,
                    &layouts.shadow,
                    &terrain_layout.locals,
                    &layout.locals,
                ],
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
                cull_mode: wgpu::CullMode::None,
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
                depth_write_enabled: false,
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
        }
    }
}

impl Pipeline for FluidPipeline {
    type Vertex = Vertex;
}
