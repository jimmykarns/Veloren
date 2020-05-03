use super::AaMode;
use image::GenericImageView;
use vek::Vec2;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    width: u32,
    height: u32,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24PlusStencil8;

    pub fn create_depth_stencil_texture(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
    ) -> Self {
        let desc = wgpu::TextureDescriptor {
            label: None,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            size: wgpu::Extent3d {
                width: sc_desc.width,
                height: sc_desc.height,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_default_view();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        Self {
            texture,
            view,
            sampler,
            width: sc_desc.width,
            height: sc_desc.height,
        }
    }

    pub fn create_multi_sample_texture(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        aa_mode: AaMode,
    ) -> Self {
        let (width, height, samples) = match aa_mode {
            AaMode::None | AaMode::Fxaa => (sc_desc.width, sc_desc.height, 1),
            // TODO: Ensure sampling in the shader is exactly between the 4 texels
            AaMode::SsaaX4 => (sc_desc.width * 2, sc_desc.height * 2, 1),
            AaMode::MsaaX4 => (sc_desc.width, sc_desc.height, 4),
            AaMode::MsaaX8 => (sc_desc.width, sc_desc.height, 8),
            AaMode::MsaaX16 => (sc_desc.width, sc_desc.height, 16),
        };

        let desc = wgpu::TextureDescriptor {
            label: None,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_default_view();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        Self {
            texture,
            view,
            sampler,
            width: sc_desc.width,
            height: sc_desc.height,
        }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        bytes: &[u8],
        tile: bool,
    ) -> (Self, wgpu::CommandBuffer) {
        let img = image::load_from_memory(bytes).unwrap();
        Self::from_image(device, &img, tile)
    }

    pub fn from_image(
        device: &wgpu::Device,
        img: &image::DynamicImage,
        tile: bool,
    ) -> (Self, wgpu::CommandBuffer) {
        let rgba = &*img.to_rgba();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        log::debug!("Texture::from_image: ({:?} × {:?} * 4)", dimensions.0, dimensions.1);
        let buffer = device.create_buffer_with_data(&rgba, wgpu::BufferUsage::COPY_SRC);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        let cmd_buffer = encoder.finish();

        let view = texture.create_default_view();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: if !tile {
                wgpu::AddressMode::ClampToEdge
            } else {
                wgpu::AddressMode::Repeat
            },
            address_mode_v: if !tile {
                wgpu::AddressMode::ClampToEdge
            } else {
                wgpu::AddressMode::Repeat
            },
            address_mode_w: if !tile {
                wgpu::AddressMode::ClampToEdge
            } else {
                wgpu::AddressMode::Repeat
            },
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        (
            Self {
                texture,
                view,
                sampler,
                width: dimensions.0,
                height: dimensions.1,
            },
            cmd_buffer,
        )
    }

    pub fn new_dynamic(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let view = texture.create_default_view();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        Self {
            texture,
            view,
            sampler,
            width,
            height,
        }
    }

    pub fn get_dimensions(&self) -> Vec2<u32> { Vec2::new(self.width, self.height) }

    pub fn update(
        &self,
        device: &wgpu::Device,
        data: &[u8],
        size: [u16; 2],
        offset: [u16; 2],
    ) -> wgpu::CommandBuffer {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if size[0] > 0 && size[1] > 0 {
            log::debug!("Texture::update: {:?} × {:?} * 4", size[0], size[1]);
            let buffer = device.create_buffer_with_data(data, wgpu::BufferUsage::COPY_SRC);

            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &buffer,
                    offset: 0,
                    bytes_per_row: 4 * size[0] as u32,
                    rows_per_image: size[1] as u32,
                },
                wgpu::TextureCopyView {
                    texture: &self.texture,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d {
                        x: offset[0] as u32,
                        y: offset[1] as u32,
                        z: 0,
                    },
                },
                wgpu::Extent3d {
                    width: size[0] as u32,
                    height: size[1] as u32,
                    depth: 1,
                },
            );
        }

        let cmd_buffer = encoder.finish();

        cmd_buffer
    }
}
