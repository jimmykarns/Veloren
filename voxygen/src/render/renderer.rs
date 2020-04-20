use super::{
    consts::Consts,
    instances::Instances,
    mesh::Mesh,
    model::{DynamicModel, Model},
    pipelines::{
        figure, fluid, postprocess, skybox, sprite, terrain, ui, Globals, GlobalsLayouts, Light,
        Shadow,
    },
    texture::Texture,
    AaMode, CloudMode, FluidMode, Pipeline,
};
use common::assets::{self, watch::ReloadIndicator};
use log::error;
use vek::*;
use zerocopy::AsBytes;

/// A type that encapsulates rendering state. `Renderer` is central to Voxygen's
/// rendering subsystem and contains any state necessary to interact with the
/// GPU, along with pipeline state objects (PSOs) needed to renderer different
/// kinds of models to the screen.
pub struct Renderer {
    // device: gfx_backend::Device,
    // encoder: gfx::Encoder<gfx_backend::Resources, gfx_backend::CommandBuffer>,
    // factory: gfx_backend::Factory,

    // win_color_view: WinColorView,
    // win_depth_view: WinDepthView,

    // tgt_color_view: TgtColorView,
    // tgt_depth_view: TgtDepthView,

    // tgt_color_res: TgtColorRes,

    // sampler: Sampler<gfx_backend::Resources>,
    shader_reload_indicator: ReloadIndicator,

    // noise_tex: Texture<(gfx::format::R8, gfx::format::Unorm)>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain: wgpu::SwapChain,
    sc_desc: wgpu::SwapChainDescriptor,
    surface: wgpu::Surface,

    size: winit::dpi::PhysicalSize<u32>,

    noise_texture: Texture,
    noise_bind_group: wgpu::BindGroup,

    depth_stencil_texture: Texture,
    tgt_color_texture: Texture,

    win_tex: Option<wgpu::SwapChainOutput>,

    globals_layouts: GlobalsLayouts,

    skybox_pipeline: skybox::SkyboxPipeline,
    figure_pipeline: figure::FigurePipeline,
    terrain_pipeline: terrain::TerrainPipeline,
    fluid_pipeline: fluid::FluidPipeline,
    sprite_pipeline: sprite::SpritePipeline,
    ui_pipeline: ui::UiPipeline,
    postprocess_pipeline: postprocess::PostProcessPipeline,

    aa_mode: AaMode,
    cloud_mode: CloudMode,
    fluid_mode: FluidMode,
}

impl Renderer {
    /// Create a new `Renderer` from a variety of backend-specific components
    /// and the window targets.
    pub fn new(
        window: &winit::window::Window,
        aa_mode: AaMode,
        cloud_mode: CloudMode,
        fluid_mode: FluidMode,
    ) -> Self {
        let mut shader_reload_indicator = ReloadIndicator::new();

        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);

        let adapter = futures::executor::block_on(wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::all(),
        ))
        .unwrap();

        println!("Rocking {:?}", adapter.get_info());

        let (device, queue) =
            futures::executor::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits { max_bind_groups: 8 },
            }));

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Noise texture
        let (noise_texture, cmds) =
            Texture::from_image(&device, &assets::load_expect("voxygen.texture.noise"), true);
        queue.submit(&[cmds]);

        let noise_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("noise texture bind group layout"),
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                    },
                ],
            });

        let noise_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("noise texture bind group"),
            layout: &noise_texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&noise_texture.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&noise_texture.sampler),
                },
            ],
        });
        // Noise Texture End

        let depth_stencil_texture = Texture::create_depth_stencil_texture(&device, &sc_desc);
        let tgt_color_texture = Texture::create_multi_sample_texture(&device, &sc_desc, aa_mode);

        let globals_layouts = GlobalsLayouts::new(&device);

        let (
            skybox_pipeline,
            figure_pipeline,
            terrain_pipeline,
            fluid_pipeline,
            sprite_pipeline,
            ui_pipeline,
            postprocess_pipeline,
        ) = create_pipelines(
            &device,
            &sc_desc,
            aa_mode,
            cloud_mode,
            fluid_mode,
            &mut shader_reload_indicator,
            &globals_layouts,
        );

        // let dims = win_color_view.get_dimensions();
        // let (tgt_color_view, tgt_depth_view, tgt_color_res) =
        //     Self::create_rt_views(&mut factory, (dims.0, dims.1), aa_mode)?;

        // let sampler = factory.create_sampler_linear();

        // let noise_tex = Texture::new(
        //     &mut factory,
        //     &assets::load_expect("voxygen.texture.noise"),
        //     Some(gfx::texture::FilterMethod::Trilinear),
        //     Some(gfx::texture::WrapMode::Tile),
        // )?;

        let win_tex = Some(swap_chain.get_next_texture().unwrap());

        Self {
            shader_reload_indicator,

            device,
            queue,
            swap_chain,
            sc_desc,
            surface,

            size: window.inner_size(),

            noise_texture,
            noise_bind_group,

            depth_stencil_texture,
            tgt_color_texture,

            globals_layouts,

            skybox_pipeline,
            figure_pipeline,
            terrain_pipeline,
            fluid_pipeline,
            sprite_pipeline,
            ui_pipeline,
            postprocess_pipeline,

            win_tex,

            aa_mode,
            cloud_mode,
            fluid_mode,
        }
    }

    pub fn max_texture_size(&self) -> usize { 2048 }

    /// Change the anti-aliasing mode
    pub fn set_aa_mode(&mut self, aa_mode: AaMode) {
        self.aa_mode = aa_mode;

        // Recreate render target
        self.on_resize(self.size);

        // Recreate pipelines with the new AA mode
        self.recreate_pipelines();
    }

    /// Change the cloud rendering mode
    pub fn set_cloud_mode(&mut self, cloud_mode: CloudMode) {
        self.cloud_mode = cloud_mode;

        // Recreate render target
        self.on_resize(self.size);

        // Recreate pipelines with the new cloud mode
        self.recreate_pipelines();
    }

    /// Change the fluid rendering mode
    pub fn set_fluid_mode(&mut self, fluid_mode: FluidMode) {
        self.fluid_mode = fluid_mode;

        // Recreate render target
        self.on_resize(self.size);

        // Recreate pipelines with the new fluid mode
        self.recreate_pipelines();
    }

    /// Resize internal render targets to match window render target dimensions.
    pub fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.win_tex = None;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.flush();

        self.depth_stencil_texture =
            Texture::create_depth_stencil_texture(&self.device, &self.sc_desc);
        self.depth_stencil_texture =
            Texture::create_multi_sample_texture(&self.device, &self.sc_desc, self.aa_mode);
    }

    // fn create_rt_views(
    //     factory: &mut gfx_device_gl::Factory,
    //     size: (u16, u16),
    //     aa_mode: AaMode,
    // ) -> Result<(TgtColorView, TgtDepthView, TgtColorRes), RenderError> {
    //     let kind = match aa_mode {
    //         AaMode::None | AaMode::Fxaa => {
    //             gfx::texture::Kind::D2(size.0, size.1,
    // gfx::texture::AaMode::Single)         },
    //         // TODO: Ensure sampling in the shader is exactly between the 4
    // texels         AaMode::SsaaX4 => {
    //             gfx::texture::Kind::D2(size.0 * 2, size.1 * 2,
    // gfx::texture::AaMode::Single)         },
    //         AaMode::MsaaX4 => {
    //             gfx::texture::Kind::D2(size.0, size.1,
    // gfx::texture::AaMode::Multi(4))         },
    //         AaMode::MsaaX8 => {
    //             gfx::texture::Kind::D2(size.0, size.1,
    // gfx::texture::AaMode::Multi(8))         },
    //         AaMode::MsaaX16 => {
    //             gfx::texture::Kind::D2(size.0, size.1,
    // gfx::texture::AaMode::Multi(16))         },
    //     };
    //     let levels = 1;

    //     let color_cty = <<TgtColorFmt as gfx::format::Formatted>::Channel as
    // gfx::format::ChannelTyped             >::get_channel_type();
    //     let tgt_color_tex = factory.create_texture(
    //         kind,
    //         levels,
    //         gfx::memory::Bind::SHADER_RESOURCE |
    // gfx::memory::Bind::RENDER_TARGET,         gfx::memory::Usage::Data,
    //         Some(color_cty),
    //     )?;
    //     let tgt_color_res =
    // factory.view_texture_as_shader_resource::<TgtColorFmt>(
    //         &tgt_color_tex,
    //         (0, levels - 1),
    //         gfx::format::Swizzle::new(),
    //     )?;
    //     let tgt_color_view =
    // factory.view_texture_as_render_target(&tgt_color_tex, 0, None)?;

    //     let depth_cty = <<TgtDepthFmt as gfx::format::Formatted>::Channel as
    // gfx::format::ChannelTyped>::get_channel_type();     let tgt_depth_tex =
    // factory.create_texture(         kind,
    //         levels,
    //         gfx::memory::Bind::DEPTH_STENCIL,
    //         gfx::memory::Usage::Data,
    //         Some(depth_cty),
    //     )?;
    //     let tgt_depth_view =
    // factory.view_texture_as_depth_stencil_trivial(&tgt_depth_tex)?;

    //     Ok((tgt_color_view, tgt_depth_view, tgt_color_res))
    // }

    /// Get the resolution of the render target.
    pub fn get_resolution(&self) -> Vec2<u16> {
        Vec2::new(self.sc_desc.width as u16, self.sc_desc.height as u16)
    }

    /// Perform all queued draw calls for this frame and clean up discarded
    /// items.
    pub fn flush(&mut self) {
        std::mem::drop(self.win_tex.take());
        self.win_tex = Some(self.swap_chain.get_next_texture().unwrap());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::TRANSPARENT,
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });
        }

        self.queue.submit(&[encoder.finish()]);

        // If the shaders files were changed attempt to recreate the shaders
        if self.shader_reload_indicator.reloaded() {
            self.recreate_pipelines();
        }
    }

    /// Recreate the pipelines
    fn recreate_pipelines(&mut self) {
        let (
            skybox_pipeline,
            figure_pipeline,
            terrain_pipeline,
            fluid_pipeline,
            sprite_pipeline,
            ui_pipeline,
            postprocess_pipeline,
        ) = create_pipelines(
            &mut self.device,
            &mut self.sc_desc,
            self.aa_mode,
            self.cloud_mode,
            self.fluid_mode,
            &mut self.shader_reload_indicator,
            &self.globals_layouts,
        );

        self.skybox_pipeline = skybox_pipeline;
        self.figure_pipeline = figure_pipeline;
        self.terrain_pipeline = terrain_pipeline;
        self.fluid_pipeline = fluid_pipeline;
        self.sprite_pipeline = sprite_pipeline;
        self.ui_pipeline = ui_pipeline;
        self.postprocess_pipeline = postprocess_pipeline;
    }

    /// Create a new set of constants with the provided values.
    pub fn create_consts<T: Copy + AsBytes>(&mut self, vals: &[T]) -> Consts<T> {
        let mut consts = Consts::new(&mut self.device, std::mem::size_of_val(vals));
        consts.update(&mut self.device, &mut self.queue, vals);
        consts
    }

    /// Update a set of constants with the provided values.
    pub fn update_consts<T: Copy + AsBytes>(&mut self, consts: &mut Consts<T>, vals: &[T]) {
        consts.update(&mut self.device, &mut self.queue, vals)
    }

    /// Create a new set of instances with the provided values.
    pub fn create_instances<T: Copy + AsBytes>(&mut self, vals: &[T]) -> Instances<T> {
        let mut instances = Instances::new(&mut self.device, std::mem::size_of_val(vals));
        instances.update(&mut self.device, &mut self.queue, vals);
        instances
    }

    /// Create a new model from the provided mesh.
    pub fn create_model<P: Pipeline>(&mut self, mesh: &Mesh<P>) -> Model {
        Model::new(&mut self.device, mesh)
    }

    /// Create a new dynamic model with the specified size.
    pub fn create_dynamic_model(&mut self, size: usize) -> DynamicModel {
        DynamicModel::new(&mut self.device, size)
    }

    /// Update a dynamic model with a mesh and a offset.
    pub fn update_model<P: Pipeline>(
        &mut self,
        model: &mut DynamicModel,
        mesh: &Mesh<P>,
        offset: usize,
    ) {
        model.update(&mut self.device, &mut self.queue, mesh, offset)
    }

    /// Create a new texture from the provided image.
    pub fn create_texture(&mut self, image: &image::DynamicImage, tile: bool) -> Texture {
        let (texture, cmds) = Texture::from_image(&mut self.device, image, tile);
        self.queue.submit(&[cmds]);
        texture
    }

    /// Create a new texture from the provided image.
    pub fn create_dynamic_texture(&mut self, width: u32, height: u32) -> Texture {
        Texture::new_dynamic(&mut self.device, width, height)
    }

    /// Update a texture with the provided offset, size, and data.
    pub fn update_texture(
        &mut self,
        texture: &Texture,
        offset: [u16; 2],
        size: [u16; 2],
        data: &[u8],
    ) {
        let cmd = texture.update(&mut self.device, data, size, offset);
        self.queue.submit(&[cmd]);
    }

    /// Creates a download buffer, downloads the win_color_view, and converts to
    /// a image::DynamicImage.
    pub fn create_screenshot(&mut self) -> image::DynamicImage {
        unimplemented!()
        // let (width, height) = self.get_resolution().into_tuple();

        // let download = self
        //     .factory
        //     .create_download_buffer::<WinSurfaceData>(width as usize * height
        // as usize)?;

        // self.encoder.copy_texture_to_buffer_raw(
        //     self.win_color_view.raw().get_texture(),
        //     None,
        //     gfx::texture::RawImageInfo {
        //         xoffset: 0,
        //         yoffset: 0,
        //         zoffset: 0,
        //         width,
        //         height,
        //         depth: 0,
        //         format: WinColorFmt::get_format(),
        //         mipmap: 0,
        //     },
        //     download.raw(),
        //     0,
        // )?;
        // self.flush();

        // // Assumes that the format is Rgba8.
        // let raw_data = self
        //     .factory
        //     .read_mapping(&download)?
        //     .chunks_exact(width as usize)
        //     .rev()
        //     .flatten()
        //     .flatten()
        //     .map(|&e| e)
        //     .collect::<Vec<_>>();
        // image::DynamicImage::ImageRgba8(
        //     // Should not fail if the dimensions are correct.
        //     image::ImageBuffer::from_raw(width as u32, height as u32,
        // raw_data).unwrap(), )
    }

    /// Queue the rendering of the provided skybox model in the upcoming frame.
    pub fn render_skybox(
        &mut self,
        model: &Model,
        globals: &Consts<Globals>,
        locals: &Consts<skybox::Locals>,
    ) {
        // self.encoder.draw(
        //     &gfx::Slice {
        //         start: model.vertex_range().start,
        //         end: model.vertex_range().end,
        //         base_vertex: 0,
        //         instances: None,
        //         buffer: gfx::IndexBuffer::Auto,
        //     },
        //     &self.skybox_pipeline.pso,
        //     &skybox::pipe::Data {
        //         vbuf: model.vbuf.clone(),
        //         locals: locals.buf.clone(),
        //         globals: globals.buf.clone(),
        //         noise: (self.noise_tex.srv.clone(), self.noise_tex.sampler.clone()),
        //         tgt_color: self.tgt_color_view.clone(),
        //         tgt_depth: self.tgt_depth_view.clone(),
        //     },
        // );
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.globals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &globals.buf,
                    range: 0..globals.len() as wgpu::BufferAddress,
                },
            }],
        });

        let locals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.skybox_pipeline.locals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &locals.buf,
                    range: 0..locals.len() as wgpu::BufferAddress,
                },
            }],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(&self.skybox_pipeline.pipeline);
            render_pass.set_bind_group(0, &globals_bind_group, &[]);
            render_pass.set_bind_group(1, &locals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
            render_pass.draw(model.vertex_range().start..model.vertex_range().end, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    /// Queue the rendering of the provided figure model in the upcoming frame.
    pub fn render_figure(
        &mut self,
        model: &Model,
        globals: &Consts<Globals>,
        locals: &Consts<figure::Locals>,
        bones: &Consts<figure::BoneData>,
        lights: &Consts<Light>,
        shadows: &Consts<Shadow>,
    ) {
        // self.encoder.draw(
        //     &gfx::Slice {
        //         start: model.vertex_range().start,
        //         end: model.vertex_range().end,
        //         base_vertex: 0,
        //         instances: None,
        //         buffer: gfx::IndexBuffer::Auto,
        //     },
        //     &self.figure_pipeline.pso,
        //     &figure::pipe::Data {
        //         vbuf: model.vbuf.clone(),
        //         locals: locals.buf.clone(),
        //         globals: globals.buf.clone(),
        //         bones: bones.buf.clone(),
        //         lights: lights.buf.clone(),
        //         shadows: shadows.buf.clone(),
        //         noise: (self.noise_tex.srv.clone(), self.noise_tex.sampler.clone()),
        //         tgt_color: self.tgt_color_view.clone(),
        //         tgt_depth: self.tgt_depth_view.clone(),
        //     },
        // );
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.globals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &globals.buf,
                    range: 0..globals.len() as wgpu::BufferAddress,
                },
            }],
        });

        let lights_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.light,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &lights.buf,
                    range: 0..lights.len() as wgpu::BufferAddress,
                },
            }],
        });

        let shadows_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.shadow,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &shadows.buf,
                    range: 0..shadows.len() as wgpu::BufferAddress,
                },
            }],
        });

        let locals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.figure_pipeline.locals,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &locals.buf,
                        range: 0..locals.len() as wgpu::BufferAddress,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &bones.buf,
                        range: 0..bones.len() as wgpu::BufferAddress,
                    },
                },
            ],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(&self.figure_pipeline.pipeline);
            render_pass.set_bind_group(0, &globals_bind_group, &[]);
            render_pass.set_bind_group(1, &lights_bind_group, &[]);
            render_pass.set_bind_group(2, &shadows_bind_group, &[]);
            render_pass.set_bind_group(3, &locals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
            render_pass.draw(model.vertex_range().start..model.vertex_range().end, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    /// Queue the rendering of the provided terrain chunk model in the upcoming
    /// frame.
    pub fn render_terrain_chunk(
        &mut self,
        model: &Model,
        globals: &Consts<Globals>,
        locals: &Consts<terrain::Locals>,
        lights: &Consts<Light>,
        shadows: &Consts<Shadow>,
    ) {
        // self.encoder.draw(
        //     &gfx::Slice {
        //         start: model.vertex_range().start,
        //         end: model.vertex_range().end,
        //         base_vertex: 0,
        //         instances: None,
        //         buffer: gfx::IndexBuffer::Auto,
        //     },
        //     &self.terrain_pipeline.pso,
        //     &terrain::pipe::Data {
        //         vbuf: model.vbuf.clone(),
        //         locals: locals.buf.clone(),
        //         globals: globals.buf.clone(),
        //         lights: lights.buf.clone(),
        //         shadows: shadows.buf.clone(),
        //         noise: (self.noise_tex.srv.clone(), self.noise_tex.sampler.clone()),
        //         tgt_color: self.tgt_color_view.clone(),
        //         tgt_depth: self.tgt_depth_view.clone(),
        //     },
        // );
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.globals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &globals.buf,
                    range: 0..globals.len() as wgpu::BufferAddress,
                },
            }],
        });

        let lights_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.light,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &lights.buf,
                    range: 0..lights.len() as wgpu::BufferAddress,
                },
            }],
        });

        let shadows_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.shadow,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &shadows.buf,
                    range: 0..shadows.len() as wgpu::BufferAddress,
                },
            }],
        });

        let locals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.terrain_pipeline.locals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &locals.buf,
                    range: 0..locals.len() as wgpu::BufferAddress,
                },
            }],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(&self.terrain_pipeline.pipeline);
            render_pass.set_bind_group(0, &globals_bind_group, &[]);
            render_pass.set_bind_group(1, &lights_bind_group, &[]);
            render_pass.set_bind_group(2, &shadows_bind_group, &[]);
            render_pass.set_bind_group(3, &locals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
            render_pass.draw(model.vertex_range().start..model.vertex_range().end, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    /// Queue the rendering of the provided terrain chunk model in the upcoming
    /// frame.
    pub fn render_fluid_chunk(
        &mut self,
        model: &Model,
        globals: &Consts<Globals>,
        locals: &Consts<terrain::Locals>,
        lights: &Consts<Light>,
        shadows: &Consts<Shadow>,
        waves: &Texture,
    ) {
        // self.encoder.draw(
        //     &gfx::Slice {
        //         start: model.vertex_range().start,
        //         end: model.vertex_range().end,
        //         base_vertex: 0,
        //         instances: None,
        //         buffer: gfx::IndexBuffer::Auto,
        //     },
        //     &self.fluid_pipeline.pso,
        //     &fluid::pipe::Data {
        //         vbuf: model.vbuf.clone(),
        //         locals: locals.buf.clone(),
        //         globals: globals.buf.clone(),
        //         lights: lights.buf.clone(),
        //         shadows: shadows.buf.clone(),
        //         noise: (self.noise_tex.srv.clone(), self.noise_tex.sampler.clone()),
        //         waves: (waves.srv.clone(), waves.sampler.clone()),
        //         tgt_color: self.tgt_color_view.clone(),
        //         tgt_depth: self.tgt_depth_view.clone(),
        //     },
        // );
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.globals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &globals.buf,
                    range: 0..globals.len() as wgpu::BufferAddress,
                },
            }],
        });

        let lights_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.light,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &lights.buf,
                    range: 0..lights.len() as wgpu::BufferAddress,
                },
            }],
        });

        let shadows_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.shadow,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &shadows.buf,
                    range: 0..shadows.len() as wgpu::BufferAddress,
                },
            }],
        });

        let terrain_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.terrain_pipeline.locals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &locals.buf,
                    range: 0..locals.len() as wgpu::BufferAddress,
                },
            }],
        });

        let locals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.fluid_pipeline.locals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&waves.sampler),
            }],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(&self.fluid_pipeline.pipeline);
            render_pass.set_bind_group(0, &globals_bind_group, &[]);
            render_pass.set_bind_group(1, &lights_bind_group, &[]);
            render_pass.set_bind_group(2, &shadows_bind_group, &[]);
            render_pass.set_bind_group(3, &terrain_bind_group, &[]);
            render_pass.set_bind_group(4, &locals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
            render_pass.draw(model.vertex_range().start..model.vertex_range().end, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    /// Queue the rendering of the provided terrain chunk model in the upcoming
    /// frame.
    pub fn render_sprites(
        &mut self,
        model: &Model,
        globals: &Consts<Globals>,
        instances: &Instances<sprite::Instance>,
        lights: &Consts<Light>,
        shadows: &Consts<Shadow>,
    ) {
        // self.encoder.draw(
        //     &gfx::Slice {
        //         start: model.vertex_range().start,
        //         end: model.vertex_range().end,
        //         base_vertex: 0,
        //         instances: Some((instances.count() as u32, 0)),
        //         buffer: gfx::IndexBuffer::Auto,
        //     },
        //     &self.sprite_pipeline.pso,
        //     &sprite::pipe::Data {
        //         vbuf: model.vbuf.clone(),
        //         ibuf: instances.ibuf.clone(),
        //         globals: globals.buf.clone(),
        //         lights: lights.buf.clone(),
        //         shadows: shadows.buf.clone(),
        //         noise: (self.noise_tex.srv.clone(), self.noise_tex.sampler.clone()),
        //         tgt_color: self.tgt_color_view.clone(),
        //         tgt_depth: self.tgt_depth_view.clone(),
        //     },
        // );
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.globals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &globals.buf,
                    range: 0..globals.len() as wgpu::BufferAddress,
                },
            }],
        });

        let lights_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.light,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &lights.buf,
                    range: 0..lights.len() as wgpu::BufferAddress,
                },
            }],
        });

        let shadows_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.shadow,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &shadows.buf,
                    range: 0..shadows.len() as wgpu::BufferAddress,
                },
            }],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(&self.sprite_pipeline.pipeline);
            render_pass.set_bind_group(0, &globals_bind_group, &[]);
            render_pass.set_bind_group(1, &lights_bind_group, &[]);
            render_pass.set_bind_group(2, &shadows_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
            render_pass.set_vertex_buffer(1, &instances.ibuf, 0, 0);
            render_pass.draw(
                model.vertex_range().start..model.vertex_range().end,
                0..instances.count() as u32,
            );
        }

        self.queue.submit(&[encoder.finish()]);
    }

    /// Queue the rendering of the provided UI element in the upcoming frame.
    pub fn render_ui_element(
        &mut self,
        model: &Model,
        tex: &Texture,
        scissor: Aabr<u16>,
        globals: &Consts<Globals>,
        locals: &Consts<ui::Locals>,
    ) {
        let Aabr { min, max } = scissor;
        // self.encoder.draw(
        //     &gfx::Slice {
        //         start: model.vertex_range().start,
        //         end: model.vertex_range().end,
        //         base_vertex: 0,
        //         instances: None,
        //         buffer: gfx::IndexBuffer::Auto,
        //     },
        //     &self.ui_pipeline.pso,
        //     &ui::pipe::Data {
        //         vbuf: model.vbuf.clone(),
        //         scissor: gfx::Rect {
        //             x: min.x,
        //             y: min.y,
        //             w: max.x - min.x,
        //             h: max.y - min.y,
        //         },
        //         tex: (tex.srv.clone(), tex.sampler.clone()),
        //         locals: locals.buf.clone(),
        //         globals: globals.buf.clone(),
        //         tgt_color: self.win_color_view.clone(),
        //         tgt_depth: self.win_depth_view.clone(),
        //     },
        // );
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.globals,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &globals.buf,
                        range: 0..globals.len() as wgpu::BufferAddress,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.noise_texture.sampler),
                },
            ],
        });

        let locals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.ui_pipeline.locals,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &locals.buf,
                        range: 0..locals.len() as wgpu::BufferAddress,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&tex.view),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&tex.sampler),
                },
            ],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::TRANSPARENT,
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Load,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Load,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(&self.ui_pipeline.pipeline);
            render_pass.set_bind_group(0, &globals_bind_group, &[]);
            render_pass.set_bind_group(1, &locals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
            render_pass.set_scissor_rect(
                min.x as u32,
                min.y as u32,
                (max.x - min.x) as u32,
                (max.y - min.y) as u32,
            );
            render_pass.draw(model.vertex_range().start..model.vertex_range().end, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    pub fn render_post_process(
        &mut self,
        model: &Model,
        globals: &Consts<Globals>,
        locals: &Consts<postprocess::Locals>,
    ) {
        // self.encoder.draw(
        //     &gfx::Slice {
        //         start: model.vertex_range().start,
        //         end: model.vertex_range().end,
        //         base_vertex: 0,
        //         instances: None,
        //         buffer: gfx::IndexBuffer::Auto,
        //     },
        //     &self.postprocess_pipeline.pso,
        //     &postprocess::pipe::Data {
        //         vbuf: model.vbuf.clone(),
        //         locals: locals.buf.clone(),
        //         globals: globals.buf.clone(),
        //         src_sampler: (self.tgt_color_res.clone(), self.sampler.clone()),
        //         tgt_color: self.win_color_view.clone(),
        //         tgt_depth: self.win_depth_view.clone(),
        //     },
        // )
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skybox command encoder"),
            });

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals_layouts.globals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &globals.buf,
                    range: 0..globals.len() as wgpu::BufferAddress,
                },
            }],
        });

        let locals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.postprocess_pipeline.locals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &locals.buf,
                    range: 0..locals.len() as wgpu::BufferAddress,
                },
            }],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.win_tex.as_ref().unwrap().view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil_texture.view,
                    depth_load_op: wgpu::LoadOp::Load,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Load,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(&self.postprocess_pipeline.pipeline);
            render_pass.set_bind_group(0, &globals_bind_group, &[]);
            render_pass.set_bind_group(1, &locals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
            render_pass.draw(model.vertex_range().start..model.vertex_range().end, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }
}

/// Creates all the pipelines used to render.
fn create_pipelines(
    device: &wgpu::Device,
    sc_desc: &wgpu::SwapChainDescriptor,
    aa_mode: AaMode,
    cloud_mode: CloudMode,
    fluid_mode: FluidMode,
    shader_reload_indicator: &mut ReloadIndicator,
    layouts: &GlobalsLayouts,
) -> (
    skybox::SkyboxPipeline,
    figure::FigurePipeline,
    terrain::TerrainPipeline,
    fluid::FluidPipeline,
    sprite::SpritePipeline,
    ui::UiPipeline,
    postprocess::PostProcessPipeline,
) {
    let globals =
        assets::load_watched::<String>("voxygen.shaders.include.globals", shader_reload_indicator)
            .unwrap();
    let sky =
        assets::load_watched::<String>("voxygen.shaders.include.sky", shader_reload_indicator)
            .unwrap();
    let light =
        assets::load_watched::<String>("voxygen.shaders.include.light", shader_reload_indicator)
            .unwrap();
    let srgb =
        assets::load_watched::<String>("voxygen.shaders.include.srgb", shader_reload_indicator)
            .unwrap();
    let random =
        assets::load_watched::<String>("voxygen.shaders.include.random", shader_reload_indicator)
            .unwrap();

    let anti_alias = assets::load_watched::<String>(
        &["voxygen.shaders.antialias.", match aa_mode {
            AaMode::None | AaMode::SsaaX4 => "none",
            AaMode::Fxaa => "fxaa",
            AaMode::MsaaX4 => "msaa-x4",
            AaMode::MsaaX8 => "msaa-x8",
            AaMode::MsaaX16 => "msaa-x16",
        }]
        .concat(),
        shader_reload_indicator,
    )
    .unwrap();

    let cloud = assets::load_watched::<String>(
        &["voxygen.shaders.include.cloud.", match cloud_mode {
            CloudMode::None => "none",
            CloudMode::Regular => "regular",
        }]
        .concat(),
        shader_reload_indicator,
    )
    .unwrap();

    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_include_callback(|name, _, _, _| {
        Ok(match name {
            "globals.glsl" => shaderc::ResolvedInclude {
                resolved_name: String::from(name),
                content: globals.as_ref().clone(),
            },
            "sky.glsl" => shaderc::ResolvedInclude {
                resolved_name: String::from(name),
                content: sky.as_ref().clone(),
            },
            "light.glsl" => shaderc::ResolvedInclude {
                resolved_name: String::from(name),
                content: light.as_ref().clone(),
            },
            "srgb.glsl" => shaderc::ResolvedInclude {
                resolved_name: String::from(name),
                content: srgb.as_ref().clone(),
            },
            "random.glsl" => shaderc::ResolvedInclude {
                resolved_name: String::from(name),
                content: random.as_ref().clone(),
            },
            "anti-aliasing.glsl" => shaderc::ResolvedInclude {
                resolved_name: String::from(name),
                content: anti_alias.as_ref().clone(),
            },
            "cloud.glsl" => shaderc::ResolvedInclude {
                resolved_name: String::from(name),
                content: cloud.as_ref().clone(),
            },
            _ => return Err(format!("Invalid include: {:?}", name)),
        })
    });

    let vs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.skybox-vert", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Vertex,
            "skybox.vert",
            "main",
            Some(&options),
        )
        .unwrap();
    let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
    let vs_module = device.create_shader_module(vs_data.as_slice());
    let fs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.skybox-frag", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Fragment,
            "skybox.frag",
            "main",
            Some(&options),
        )
        .unwrap();
    let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
    let fs_module = device.create_shader_module(fs_data.as_slice());

    let skybox_pipeline =
        skybox::SkyboxPipeline::new(device, &vs_module, &fs_module, sc_desc, layouts);

    // // Construct a pipeline for rendering skyboxes
    // let skybox_pipeline = create_pipeline(
    //     factory,
    //     skybox::pipe::new(),
    //     &assets::load_watched::<String>("voxygen.shaders.skybox-vert",
    // shader_reload_indicator)         .unwrap(),
    //     &assets::load_watched::<String>("voxygen.shaders.skybox-frag",
    // shader_reload_indicator)         .unwrap(),
    //     &include_ctx,
    //     gfx::state::CullFace::Back,
    // )?;

    let vs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.figure-vert", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Vertex,
            "figure.vert",
            "main",
            Some(&options),
        )
        .unwrap();
    let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
    let vs_module = device.create_shader_module(vs_data.as_slice());
    let fs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.figure-frag", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Fragment,
            "figure.frag",
            "main",
            Some(&options),
        )
        .unwrap();
    let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
    let fs_module = device.create_shader_module(fs_data.as_slice());

    let figure_pipeline =
        figure::FigurePipeline::new(device, &vs_module, &fs_module, sc_desc, layouts);

    // // Construct a pipeline for rendering figures
    // let figure_pipeline = create_pipeline(
    //     factory,
    //     figure::pipe::new(),
    //     &assets::load_watched::<String>("voxygen.shaders.figure-vert",
    // shader_reload_indicator)         .unwrap(),
    //     &assets::load_watched::<String>("voxygen.shaders.figure-frag",
    // shader_reload_indicator)         .unwrap(),
    //     &include_ctx,
    //     gfx::state::CullFace::Back,
    // )?;

    let vs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>(
                "voxygen.shaders.terrain-vert",
                shader_reload_indicator,
            )
            .unwrap(),
            shaderc::ShaderKind::Vertex,
            "terrain.vert",
            "main",
            Some(&options),
        )
        .unwrap();
    let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
    let vs_module = device.create_shader_module(vs_data.as_slice());
    let fs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>(
                "voxygen.shaders.terrain-frag",
                shader_reload_indicator,
            )
            .unwrap(),
            shaderc::ShaderKind::Fragment,
            "terrain.frag",
            "main",
            Some(&options),
        )
        .unwrap();
    let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
    let fs_module = device.create_shader_module(fs_data.as_slice());

    let terrain_pipeline =
        terrain::TerrainPipeline::new(device, &vs_module, &fs_module, sc_desc, layouts);

    // // Construct a pipeline for rendering terrain
    // let terrain_pipeline = create_pipeline(
    //     factory,
    //     terrain::pipe::new(),
    //     &assets::load_watched::<String>("voxygen.shaders.terrain-vert",
    // shader_reload_indicator)         .unwrap(),
    //     &assets::load_watched::<String>("voxygen.shaders.terrain-frag",
    // shader_reload_indicator)         .unwrap(),
    //     &include_ctx,
    //     gfx::state::CullFace::Back,
    // )?;

    let vs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.fluid-vert", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Vertex,
            "fluid.vert",
            "main",
            Some(&options),
        )
        .unwrap();
    let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
    let vs_module = device.create_shader_module(vs_data.as_slice());
    let fs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>(
                &["voxygen.shaders.fluid-frag.", match fluid_mode {
                    FluidMode::Cheap => "cheap",
                    FluidMode::Shiny => "shiny",
                }]
                .concat(),
                shader_reload_indicator,
            )
            .unwrap(),
            shaderc::ShaderKind::Fragment,
            "fluid.frag",
            "main",
            Some(&options),
        )
        .unwrap();
    let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
    let fs_module = device.create_shader_module(fs_data.as_slice());

    let fluid_pipeline = fluid::FluidPipeline::new(
        device,
        &vs_module,
        &fs_module,
        sc_desc,
        layouts,
        &terrain_pipeline.locals,
    );

    // // Construct a pipeline for rendering fluids
    // let fluid_pipeline = create_pipeline(
    //     factory,
    //     fluid::pipe::new(),
    //     &assets::load_watched::<String>("voxygen.shaders.fluid-vert",
    // shader_reload_indicator)         .unwrap(),
    //     &assets::load_watched::<String>(
    //         &["voxygen.shaders.fluid-frag.", match fluid_mode {
    //             FluidMode::Cheap => "cheap",
    //             FluidMode::Shiny => "shiny",
    //         }]
    //         .concat(),
    //         shader_reload_indicator,
    //     )
    //     .unwrap(),
    //     &include_ctx,
    //     gfx::state::CullFace::Nothing,
    // )?;

    let vs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.sprite-vert", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Vertex,
            "sprite.vert",
            "main",
            Some(&options),
        )
        .unwrap();
    let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
    let vs_module = device.create_shader_module(vs_data.as_slice());
    let fs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.sprite-frag", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Fragment,
            "sprite.frag",
            "main",
            Some(&options),
        )
        .unwrap();
    let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
    let fs_module = device.create_shader_module(fs_data.as_slice());

    let sprite_pipeline =
        sprite::SpritePipeline::new(device, &vs_module, &fs_module, sc_desc, layouts);

    // // Construct a pipeline for rendering sprites
    // let sprite_pipeline = create_pipeline(
    //     factory,
    //     sprite::pipe::new(),
    //     &assets::load_watched::<String>("voxygen.shaders.sprite-vert",
    // shader_reload_indicator)         .unwrap(),
    //     &assets::load_watched::<String>("voxygen.shaders.sprite-frag",
    // shader_reload_indicator)         .unwrap(),
    //     &include_ctx,
    //     gfx::state::CullFace::Back,
    // )?;

    let vs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.ui-vert", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Vertex,
            "ui.vert",
            "main",
            Some(&options),
        )
        .unwrap();
    let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
    let vs_module = device.create_shader_module(vs_data.as_slice());
    let fs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>("voxygen.shaders.ui-frag", shader_reload_indicator)
                .unwrap(),
            shaderc::ShaderKind::Fragment,
            "ui.frag",
            "main",
            Some(&options),
        )
        .unwrap();
    let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
    let fs_module = device.create_shader_module(fs_data.as_slice());

    let ui_pipeline = ui::UiPipeline::new(device, &vs_module, &fs_module, sc_desc, layouts);

    // // Construct a pipeline for rendering UI elements
    // let ui_pipeline = create_pipeline(
    //     factory,
    //     ui::pipe::new(),
    //     &assets::load_watched::<String>("voxygen.shaders.ui-vert",
    // shader_reload_indicator)         .unwrap(),
    //     &assets::load_watched::<String>("voxygen.shaders.ui-frag",
    // shader_reload_indicator)         .unwrap(),
    //     &include_ctx,
    //     gfx::state::CullFace::Back,
    // )?;

    let vs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>(
                "voxygen.shaders.postprocess-vert",
                shader_reload_indicator,
            )
            .unwrap(),
            shaderc::ShaderKind::Vertex,
            "postprocess.vert",
            "main",
            Some(&options),
        )
        .unwrap();
    let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
    let vs_module = device.create_shader_module(vs_data.as_slice());
    let fs_spirv = compiler
        .compile_into_spirv(
            &assets::load_watched::<String>(
                "voxygen.shaders.postprocess-frag",
                shader_reload_indicator,
            )
            .unwrap(),
            shaderc::ShaderKind::Fragment,
            "postprocess.frag",
            "main",
            Some(&options),
        )
        .unwrap();
    let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
    let fs_module = device.create_shader_module(fs_data.as_slice());

    let postprocess_pipeline =
        postprocess::PostProcessPipeline::new(device, &vs_module, &fs_module, sc_desc, layouts);

    // // Construct a pipeline for rendering our post-processing
    // let postprocess_pipeline = create_pipeline(
    //     factory,
    //     postprocess::pipe::new(),
    //     &assets::load_watched::<String>(
    //         "voxygen.shaders.postprocess-vert",
    //         shader_reload_indicator,
    //     )
    //     .unwrap(),
    //     &assets::load_watched::<String>(
    //         "voxygen.shaders.postprocess-frag",
    //         shader_reload_indicator,
    //     )
    //     .unwrap(),
    //     &include_ctx,
    //     gfx::state::CullFace::Back,
    // )?;

    (
        skybox_pipeline,
        figure_pipeline,
        terrain_pipeline,
        fluid_pipeline,
        sprite_pipeline,
        ui_pipeline,
        postprocess_pipeline,
    )
}
