use super::{
    super::{Mesh, Pipeline, Tri, DEPTH_FORMAT},
    Globals, GlobalsLayouts,
};
use zerocopy::AsBytes;

// gfx_defines! {
//     vertex Vertex {
//         pos: [f32; 2] = "v_pos",
//     }

//     constant Locals {
//         nul: [f32; 4] = "nul",
//     }

//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),

//         locals: gfx::ConstantBuffer<Locals> = "u_locals",
//         globals: gfx::ConstantBuffer<Globals> = "u_globals",

//         src_sampler: gfx::TextureSampler<<WinColorFmt as
// gfx::format::Formatted>::View> = "src_color",

//         tgt_color: gfx::RenderTarget<WinColorFmt> = "tgt_color",
//         tgt_depth: gfx::DepthTarget<WinDepthFmt> =
// gfx::preset::depth::PASS_TEST,     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
    pos: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float2,
            }],
        }
    }
}

pub struct PostProcessLayout {
    pub locals: wgpu::BindGroupLayout,
}

impl PostProcessLayout {
    pub fn new(device: &wgpu::Device, multisampled: bool) -> Self {
        Self {
            locals: Self::locals_layout(device, multisampled),
        }
    }

    fn locals_layout(device: &wgpu::Device, multisampled: bool) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                        //todo
                        component_type: wgpu::TextureComponentType::Float,
                        multisampled,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
        })
    }
}

pub struct PostProcessPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl PostProcessPipeline {
    pub fn new(
        device: &wgpu::Device,
        vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule,
        sc_desc: &wgpu::SwapChainDescriptor,
        layouts: &GlobalsLayouts,
        layout: &PostProcessLayout,
    ) -> Self {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&layouts.globals, &layout.locals],
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
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
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

impl Pipeline for PostProcessPipeline {
    type Vertex = Vertex;
}

pub fn create_mesh() -> Mesh<PostProcessPipeline> {
    let mut mesh = Mesh::new();

    #[rustfmt::skip]
    mesh.push_tri(Tri::new(
        Vertex { pos: [ 1.0, -1.0] },
        Vertex { pos: [-1.0,  1.0] },
        Vertex { pos: [-1.0, -1.0] },
    ));

    #[rustfmt::skip]
    mesh.push_tri(Tri::new(
        Vertex { pos: [1.0, -1.0] },
        Vertex { pos: [1.0,  1.0] },
        Vertex { pos: [-1.0, 1.0] },
    ));

    mesh
}
