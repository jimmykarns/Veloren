use super::{
    super::{Mesh, Pipeline, Quad, DEPTH_FORMAT},
    Globals, GlobalsLayouts,
};
use zerocopy::AsBytes;

// gfx_defines! {
//     vertex Vertex {
//         pos: [f32; 3] = "v_pos",
//     }

//     constant Locals {
//         nul: [f32; 4] = "nul",
//     }

//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),

//         locals: gfx::ConstantBuffer<Locals> = "u_locals",
//         globals: gfx::ConstantBuffer<Globals> = "u_globals",

//         noise: gfx::TextureSampler<f32> = "t_noise",

//         tgt_color: gfx::RenderTarget<TgtColorFmt> = "tgt_color",
//         tgt_depth: gfx::DepthTarget<TgtDepthFmt> =
// gfx::preset::depth::LESS_EQUAL_WRITE,     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
    pos: [f32; 3],
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
                format: wgpu::VertexFormat::Float3,
            }],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Locals {
    nul: [f32; 4],
}

impl Locals {
    pub fn default() -> Self { Self { nul: [0.0; 4] } }
}

pub struct SkyboxPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub locals: wgpu::BindGroupLayout,
}

impl SkyboxPipeline {
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
                bind_group_layouts: &[&layouts.globals, &locals],
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

impl Pipeline for SkyboxPipeline {
    type Vertex = Vertex;
}

pub fn create_mesh() -> Mesh<SkyboxPipeline> {
    let mut mesh = Mesh::new();

    // -x
    #[rustfmt::skip]
    mesh.push_quad(Quad::new(
        Vertex { pos: [-1.0, -1.0, -1.0] },
        Vertex { pos: [-1.0,  1.0, -1.0] },
        Vertex { pos: [-1.0,  1.0,  1.0] },
        Vertex { pos: [-1.0, -1.0,  1.0] },
    ));
    // +x
    #[rustfmt::skip]
    mesh.push_quad(Quad::new(
        Vertex { pos: [ 1.0, -1.0,  1.0] },
        Vertex { pos: [ 1.0,  1.0,  1.0] },
        Vertex { pos: [ 1.0,  1.0, -1.0] },
        Vertex { pos: [ 1.0, -1.0, -1.0] },
    ));
    // -y
    #[rustfmt::skip]
    mesh.push_quad(Quad::new(
        Vertex { pos: [ 1.0, -1.0, -1.0] },
        Vertex { pos: [-1.0, -1.0, -1.0] },
        Vertex { pos: [-1.0, -1.0,  1.0] },
        Vertex { pos: [ 1.0, -1.0,  1.0] },
    ));
    // +y
    #[rustfmt::skip]
    mesh.push_quad(Quad::new(
        Vertex { pos: [ 1.0,  1.0,  1.0] },
        Vertex { pos: [-1.0,  1.0,  1.0] },
        Vertex { pos: [-1.0,  1.0, -1.0] },
        Vertex { pos: [ 1.0,  1.0, -1.0] },
    ));
    // -z
    #[rustfmt::skip]
    mesh.push_quad(Quad::new(
        Vertex { pos: [-1.0, -1.0, -1.0] },
        Vertex { pos: [ 1.0, -1.0, -1.0] },
        Vertex { pos: [ 1.0,  1.0, -1.0] },
        Vertex { pos: [-1.0,  1.0, -1.0] },
    ));
    // +z
    #[rustfmt::skip]
    mesh.push_quad(Quad::new(
        Vertex { pos: [-1.0,  1.0,  1.0] },
        Vertex { pos: [ 1.0,  1.0,  1.0] },
        Vertex { pos: [ 1.0, -1.0,  1.0] },
        Vertex { pos: [-1.0, -1.0,  1.0] },
    ));

    mesh
}
