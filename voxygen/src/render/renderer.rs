use super::{
    consts::Consts,
    instances::Instances,
    mesh::Mesh,
    model::Model,
    pipelines::{
        figure, fluid, postprocess, skybox, sprite, terrain, ui, Globals, GlobalsLayouts, Light,
        Shadow,
    },
    texture::Texture,
    AaMode, CloudMode, FigureBoneData, FigureLocals, FluidLocals, FluidMode, Pipeline,
    TerrainLocals, UiLocals,
};
use common::assets::{self, watch::ReloadIndicator};
use log::error;
use vek::*;
use zerocopy::AsBytes;

mod drawer;

pub use drawer::{Drawer, FirstDrawer, SecondDrawer};

/// A type that stores all the layouts associated with this renderer.
pub struct Layouts {
    pub(self) global: GlobalsLayouts,

    pub(self) skybox: skybox::SkyboxLayout,
    pub(self) figure: figure::FigureLayout,
    pub(self) terrain: terrain::TerrainLayout,
    pub(self) fluid: fluid::FluidLayout,
    pub(self) sprite: sprite::SpriteLayout,
    pub(self) ui: ui::UiLayout,
    pub(self) postprocess: postprocess::PostProcessLayout,
}

/// A type that encapsulates rendering state. `Renderer` is central to Voxygen's
/// rendering subsystem and contains any state necessary to interact with the
/// GPU, along with pipeline state objects (PSOs) needed to renderer different
/// kinds of models to the screen.
pub struct Renderer {
    shader_reload_indicator: ReloadIndicator,

    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain: wgpu::SwapChain,
    sc_desc: wgpu::SwapChainDescriptor,
    surface: wgpu::Surface,

    size: winit::dpi::PhysicalSize<u32>,

    pub(self) noise_texture: Texture,

    depth_stencil_texture: Texture,
    tgt_color_texture: Texture,

    // pub(self) globals_layouts: GlobalsLayouts,
    // pub(self) terrain_layout: wgpu::RenderLayout,
    layouts: Layouts,

    pub(self) skybox_pipeline: skybox::SkyboxPipeline,
    pub(self) figure_pipeline: figure::FigurePipeline,
    pub(self) terrain_pipeline: terrain::TerrainPipeline,
    pub(self) fluid_pipeline: fluid::FluidPipeline,
    pub(self) sprite_pipeline: sprite::SpritePipeline,
    pub(self) ui_pipeline: ui::UiPipeline,
    pub(self) postprocess_pipeline: postprocess::PostProcessPipeline,

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

        log::info!("Rocking {:?}", adapter.get_info());

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
        // Noise Texture End

        let depth_stencil_texture = Texture::create_depth_stencil_texture(&device, &sc_desc);
        let tgt_color_texture = Texture::create_multi_sample_texture(&device, &sc_desc, aa_mode);

        let layouts = Layouts {
            global: GlobalsLayouts::new(&device),
            skybox: skybox::SkyboxLayout::new(&device),
            figure: figure::FigureLayout::new(&device),
            terrain: terrain::TerrainLayout::new(&device),
            fluid: fluid::FluidLayout::new(&device),
            sprite: sprite::SpriteLayout::new(&device),
            ui: ui::UiLayout::new(&device),
            postprocess: postprocess::PostProcessLayout::new(&device, match aa_mode {
                AaMode::None | AaMode::SsaaX4 | AaMode::Fxaa => false,
                _ => true,
            }),
        };

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
            &layouts,
        );

        Self {
            shader_reload_indicator,

            device,
            queue,
            swap_chain,
            sc_desc,
            surface,

            size: window.inner_size(),

            noise_texture,

            depth_stencil_texture,
            tgt_color_texture,

            // globals_layouts,
            layouts,

            skybox_pipeline,
            figure_pipeline,
            terrain_pipeline,
            fluid_pipeline,
            sprite_pipeline,
            ui_pipeline,
            postprocess_pipeline,

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
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        self.depth_stencil_texture =
            Texture::create_depth_stencil_texture(&self.device, &self.sc_desc);
        self.tgt_color_texture =
            Texture::create_multi_sample_texture(&self.device, &self.sc_desc, self.aa_mode);

        self.flush();
    }

    /// Get the resolution of the render target.
    pub fn get_resolution(&self) -> Vec2<u16> {
        Vec2::new(self.sc_desc.width as u16, self.sc_desc.height as u16)
    }

    /// Perform all queued draw calls for this frame and clean up discarded
    /// items.
    pub fn flush(&mut self) {
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
            &self.device,
            &self.sc_desc,
            self.aa_mode,
            self.cloud_mode,
            self.fluid_mode,
            &mut self.shader_reload_indicator,
            &self.layouts,
            // &self.globals_layouts,
        );

        self.skybox_pipeline = skybox_pipeline;
        self.figure_pipeline = figure_pipeline;
        self.terrain_pipeline = terrain_pipeline;
        self.fluid_pipeline = fluid_pipeline;
        self.sprite_pipeline = sprite_pipeline;
        self.ui_pipeline = ui_pipeline;
        self.postprocess_pipeline = postprocess_pipeline;
    }

    pub fn create_consts_globals(&self, vals: &[Globals]) -> Consts<Globals> {
        let len = std::mem::size_of_val(vals);

        log::debug!("Bind consts_globabls");
        let buf = self
            .device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: Some("Globals buffer"),
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Globals bind group"),
            layout: &self.layouts.global.globals,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buf,
                        range: 0..len as wgpu::BufferAddress,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.noise_texture.view),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.noise_texture.sampler),
                },
            ],
        });

        self.create_consts_internal(len, vals, Some(buf), bind_group)
    }

    pub fn create_consts_light(&self, vals: &[Light]) -> Consts<Light> {
        let len = std::mem::size_of_val(vals);

        log::debug!("Bind consts_light");
        let buf = self
            .device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: Some("Light buffer"),
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light bind group"),
            layout: &self.layouts.global.light,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buf,
                    range: 0..len as wgpu::BufferAddress,
                },
            }],
        });

        self.create_consts_internal(len, vals, Some(buf), bind_group)
    }

    pub fn create_consts_shadows(&self, vals: &[Shadow]) -> Consts<Shadow> {
        let len = std::mem::size_of_val(vals);

        log::debug!("Bind consts_shadows");
        let buf = self
            .device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: Some("Shadow buffer"),
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shadow bind group"),
            layout: &self.layouts.global.shadow,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buf,
                    range: 0..len as wgpu::BufferAddress,
                },
            }],
        });

        self.create_consts_internal(len, vals, Some(buf), bind_group)
    }

    pub fn create_consts_figure_locals(&self, vals: &[FigureLocals]) -> Consts<FigureLocals> {
        let len = std::mem::size_of_val(vals);

        log::debug!("Bind consts_figure_locals");
        let buf = self
            .device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: Some("Figure locals buffer"),
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Figure locals bind group"),
            layout: &self.layouts.figure.locals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buf,
                    range: 0..len as wgpu::BufferAddress,
                },
            }],
        });

        self.create_consts_internal(len, vals, Some(buf), bind_group)
    }

    pub fn create_consts_bone_data(&self, vals: &[FigureBoneData]) -> Consts<FigureBoneData> {
        let len = std::mem::size_of_val(vals);

        log::debug!("Bind create_consts_bone_data");
        let buf = self
            .device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: Some("Bone data buffer"),
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bone data bind group"),
            layout: &self.layouts.figure.bone_data,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buf,
                    range: 0..len as wgpu::BufferAddress,
                },
            }],
        });

        self.create_consts_internal(len, vals, Some(buf), bind_group)
    }

    pub fn create_consts_fluid_locals(&self, waves: Texture) -> Consts<FluidLocals> {
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Fluid locals bind group"),
            layout: &self.layouts.fluid.locals,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&waves.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&waves.sampler),
                },
            ],
        });

        self.create_consts_internal(0, &[], None, bind_group)
    }

    pub fn create_consts_terrain_locals(&self, vals: &[TerrainLocals]) -> Consts<TerrainLocals> {
        let len = std::mem::size_of_val(vals);

        log::debug!("Bind create_consts_terrain_locals");
        let buf = self
            .device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: Some("Terrain locals buffer"),
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Terrain locals bind group"),
            layout: &self.layouts.terrain.locals, //&self.terrain_pipeline.locals,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buf,
                    range: 0..len as wgpu::BufferAddress,
                },
            }],
        });

        self.create_consts_internal(len, vals, Some(buf), bind_group)
    }

    pub fn create_consts_ui_locals(&self, vals: &[UiLocals], tex: &Texture) -> Consts<UiLocals> {
        let len = std::mem::size_of_val(vals);

        log::debug!("Bind create_consts_ui_locals");
        let buf = self
            .device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: Some("UI locals buffer"),
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("UI locals bind group"),
            layout: &self.layouts.ui.locals,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buf,
                        range: 0..len as wgpu::BufferAddress,
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

        self.create_consts_internal(len, vals, Some(buf), bind_group)
    }

    fn create_consts_internal<T: Copy + AsBytes>(
        &self,
        len: usize,
        vals: &[T],
        buf: Option<wgpu::Buffer>,
        bind_group: wgpu::BindGroup,
    ) -> Consts<T> {
        let mut consts = Consts::new(len, buf, bind_group);
        consts.update(&self.device, &self.queue, vals);
        consts
    }

    /// Update a set of constants with the provided values.
    pub fn update_consts<T: Copy + AsBytes>(&self, consts: &Consts<T>, vals: &[T]) {
        consts.update(&self.device, &self.queue, vals)
    }

    /// Create a new set of instances with the provided values.
    pub fn create_instances<T: Copy + AsBytes>(&self, vals: &[T]) -> Instances<T> {
        let mut instances = Instances::new(&self.device, std::mem::size_of_val(vals));
        instances.update(&self.device, &self.queue, vals);
        instances
    }

    /// Create a new model from the provided mesh.
    pub fn create_model<P: Pipeline>(&self, mesh: &Mesh<P>) -> Model {
        let mut model = Model::new(&self.device, std::mem::size_of_val(mesh.vertices()));
        model.update(&self.device, &self.queue, mesh, 0);
        model
    }

    /// Create a new dynamic model with the specified size.
    pub fn create_dynamic_model(&self, size: usize) -> Model { Model::new(&self.device, size) }

    /// Update a dynamic model with a mesh and a offset.
    pub fn update_model<P: Pipeline>(&self, model: &mut Model, mesh: &Mesh<P>, offset: usize) {
        model.update(&self.device, &self.queue, mesh, offset)
    }

    /// Create a new texture from the provided image.
    pub fn create_texture(&self, image: &image::DynamicImage, tile: bool) -> Texture {
        let (texture, cmds) = Texture::from_image(&self.device, image, tile);
        self.queue.submit(&[cmds]);
        texture
    }

    /// Create a new texture from the provided image.
    pub fn create_dynamic_texture(&self, width: u32, height: u32) -> Texture {
        Texture::new_dynamic(&self.device, width, height)
    }

    /// Update a texture with the provided offset, size, and data.
    pub fn update_texture(&self, texture: &Texture, offset: [u16; 2], size: [u16; 2], data: &[u8]) {
        let cmd = texture.update(&self.device, data, size, offset);
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

    pub fn drawer(&mut self) -> Drawer {
        let mut encoder = Some(self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render pass encoder"),
            },
        ));

        let postprocess_locals = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("PostProcess bind group"),
            layout: &self.layouts.postprocess.locals,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.tgt_color_texture.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.tgt_color_texture.sampler),
                },
            ],
        });

        Drawer {
            encoder,
            tex: self.swap_chain.get_next_texture().unwrap(),
            renderer: self,
            postprocess_locals,
        }
    }
}

/* /// Creates all the layouts used to render.
fn create_layouts(
    device: &wgpu::Device,
) -> (
    skybox::SkyboxLayout,
    figure::FigureLayout,
    terrain::TerrainLayout,
    fluid::FluidLayout,
    sprite::SpriteLayout,
    ui::UiLayout,
    postprocess::PostProcessLayout,
) {
    (
        GlobalsLayouts::new(&device);
        skybox::SkyboxLayout::new(device),
        figure::FigureLayout::new(device),
        terrain::TerrainLayout::new(device),
        fluid::FluidLayout::new(device),
        sprite::SpriteLayout::new(device),
        ui::UiLayout::new(device),
        postprocess::PostProcessLayout::new(device),
    )
} */

/// Creates all the pipelines used to render.
fn create_pipelines(
    device: &wgpu::Device,
    sc_desc: &wgpu::SwapChainDescriptor,
    aa_mode: AaMode,
    cloud_mode: CloudMode,
    fluid_mode: FluidMode,
    shader_reload_indicator: &mut ReloadIndicator,
    layouts: &Layouts,
    // layouts: &GlobalsLayouts,
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

    let skybox_pipeline = skybox::SkyboxPipeline::new(
        device,
        &vs_module,
        &fs_module,
        sc_desc,
        &layouts.global,
        &layouts.skybox,
        aa_mode,
    );

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

    let figure_pipeline = figure::FigurePipeline::new(
        device,
        &vs_module,
        &fs_module,
        sc_desc,
        &layouts.global,
        &layouts.figure,
        aa_mode,
    );

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

    let terrain_pipeline = terrain::TerrainPipeline::new(
        device,
        &vs_module,
        &fs_module,
        sc_desc,
        &layouts.global,
        &layouts.terrain,
        aa_mode,
    );

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
        &layouts.global,
        &layouts.terrain,
        &layouts.fluid,
        aa_mode,
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

    let sprite_pipeline = sprite::SpritePipeline::new(
        device,
        &vs_module,
        &fs_module,
        sc_desc,
        &layouts.global,
        &layouts.sprite,
        aa_mode,
    );

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

    let ui_pipeline = ui::UiPipeline::new(
        device,
        &vs_module,
        &fs_module,
        sc_desc,
        &layouts.global,
        &layouts.ui,
    );

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

    let postprocess_pipeline = postprocess::PostProcessPipeline::new(
        device,
        &vs_module,
        &fs_module,
        sc_desc,
        &layouts.global,
        &layouts.postprocess,
    );

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
