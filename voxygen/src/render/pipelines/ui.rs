use super::{
    super::{Globals, Pipeline, Quad, Tri, DEPTH_FORMAT},
    GlobalsLayouts,
};
use vek::*;
use zerocopy::AsBytes;

// gfx_defines! {
//     vertex Vertex {
//         pos: [f32; 2] = "v_pos",
//         uv: [f32; 2] = "v_uv",
//         color: [f32; 4] = "v_color",
//         center: [f32; 2] = "v_center",
//         mode: u32 = "v_mode",
//     }

//     constant Locals {
//         pos: [f32; 4] = "w_pos",
//     }

//     pipeline pipe {
//         vbuf: gfx::VertexBuffer<Vertex> = (),

//         locals: gfx::ConstantBuffer<Locals> = "u_locals",
//         globals: gfx::ConstantBuffer<Globals> = "u_globals",
//         tex: gfx::TextureSampler<[f32; 4]> = "u_tex",

//         scissor: gfx::Scissor = (),

//         tgt_color: gfx::BlendTarget<WinColorFmt> = ("tgt_color",
// gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),         tgt_depth:
// gfx::DepthTarget<WinDepthFmt> = gfx::preset::depth::LESS_EQUAL_TEST,     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
    center: [f32; 2],
    mode: u32,
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
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress * 2,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: (mem::size_of::<[f32; 2]>() * 2 + mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: (mem::size_of::<[f32; 2]>() * 3 + mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Uint,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Locals {
    pos: [f32; 4],
}

pub struct UiPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub locals: wgpu::BindGroupLayout,
}

impl UiPipeline {
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
            locals,
        }
    }

    fn locals_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                        //todo
                        component_type: wgpu::TextureComponentType::Float,
                        multisampled: false,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
        })
    }
}

impl Pipeline for UiPipeline {
    type Vertex = Vertex;
}

impl From<Vec4<f32>> for Locals {
    fn from(pos: Vec4<f32>) -> Self {
        Self {
            pos: pos.into_array(),
        }
    }
}

impl Default for Locals {
    fn default() -> Self { Self { pos: [0.0; 4] } }
}

/// Draw text from the text cache texture `tex` in the fragment shader.
pub const MODE_TEXT: u32 = 0;
/// Draw an image from the texture at `tex` in the fragment shader.
pub const MODE_IMAGE: u32 = 1;
/// Ignore `tex` and draw simple, colored 2D geometry.
pub const MODE_GEOMETRY: u32 = 2;
/// Draw an image from the texture at `tex` in the fragment shader, with the
/// source rectangle rotated to face north.
///
/// FIXME: Make more principled.
pub const MODE_IMAGE_SOURCE_NORTH: u32 = 3;
/// Draw an image from the texture at `tex` in the fragment shader, with the
/// target rectangle rotated to face north.
///
/// FIXME: Make more principled.
pub const MODE_IMAGE_TARGET_NORTH: u32 = 5;

pub enum Mode {
    Text,
    Image,
    Geometry,
    ImageSourceNorth,
    ImageTargetNorth,
}

impl Mode {
    fn value(self) -> u32 {
        match self {
            Mode::Text => MODE_TEXT,
            Mode::Image => MODE_IMAGE,
            Mode::Geometry => MODE_GEOMETRY,
            Mode::ImageSourceNorth => MODE_IMAGE_SOURCE_NORTH,
            Mode::ImageTargetNorth => MODE_IMAGE_TARGET_NORTH,
        }
    }
}

pub fn create_quad(
    rect: Aabr<f32>,
    uv_rect: Aabr<f32>,
    color: Rgba<f32>,
    mode: Mode,
) -> Quad<UiPipeline> {
    let center = if let Mode::ImageSourceNorth = mode {
        uv_rect.center().into_array()
    } else {
        rect.center().into_array()
    };
    let mode_val = mode.value();
    let v = |pos, uv| Vertex {
        pos,
        uv,
        center,
        color: color.into_array(),
        mode: mode_val,
    };
    let aabr_to_lbrt = |aabr: Aabr<f32>| (aabr.min.x, -aabr.min.y, aabr.max.x, -aabr.max.y);

    let (l, b, r, t) = aabr_to_lbrt(rect);
    let (uv_l, uv_b, uv_r, uv_t) = aabr_to_lbrt(uv_rect);

    match (uv_b > uv_t, uv_l > uv_r) {
        (true, true) => Quad::new(
            v([r, t], [uv_l, uv_b]),
            v([l, t], [uv_l, uv_t]),
            v([l, b], [uv_r, uv_t]),
            v([r, b], [uv_r, uv_b]),
        ),
        (false, false) => Quad::new(
            v([r, t], [uv_l, uv_b]),
            v([l, t], [uv_l, uv_t]),
            v([l, b], [uv_r, uv_t]),
            v([r, b], [uv_r, uv_b]),
        ),
        _ => Quad::new(
            v([r, t], [uv_r, uv_t]),
            v([l, t], [uv_l, uv_t]),
            v([l, b], [uv_l, uv_b]),
            v([r, b], [uv_r, uv_b]),
        ),
    }
}

pub fn create_tri(
    tri: [[f32; 2]; 3],
    uv_tri: [[f32; 2]; 3],
    color: Rgba<f32>,
    mode: Mode,
) -> Tri<UiPipeline> {
    let center = [0.0, 0.0];
    let mode_val = mode.value();
    let v = |pos, uv| Vertex {
        pos,
        uv,
        center,
        color: color.into_array(),
        mode: mode_val,
    };
    Tri::new(
        v([tri[0][0], tri[0][1]], [uv_tri[0][0], uv_tri[0][1]]),
        v([tri[1][0], tri[1][1]], [uv_tri[1][0], uv_tri[1][1]]),
        v([tri[2][0], tri[2][1]], [uv_tri[2][0], uv_tri[2][1]]),
    )
}
