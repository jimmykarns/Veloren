use super::{mesh::Mesh, Pipeline};
use std::{ops::Range, sync::Arc};
use zerocopy::AsBytes;

/// Represents a mesh on the GPU which can be updated dynamically.
pub struct Model {
    pub vbuf: Arc<wgpu::Buffer>,
    size: usize,
}

impl Model {
    pub fn new(device: &wgpu::Device, size: usize) -> Self {
        let vbuf = device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: None,
                size: size as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        Self {
            vbuf: Arc::new(vbuf),
            size,
        }
    }

    pub fn len(&self) -> usize { self.size }

    pub fn update<P: Pipeline>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mesh: &Mesh<P>,
        offset: usize,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if self.size < std::mem::size_of_val(mesh.vertices()) {
            *Arc::get_mut(&mut self.vbuf).unwrap() = device
                .create_buffer_mapped(&wgpu::BufferDescriptor {
                    label: None,
                    size: std::mem::size_of_val(mesh.vertices()) as wgpu::BufferAddress,
                    usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
                })
                .finish();

            self.size = std::mem::size_of_val(mesh.vertices());
        }

        let staging_buffer = device.create_buffer_with_data(
            mesh.vertices()
                .iter()
                .map(|v| v.as_bytes())
                .flatten()
                .map(|v| *v)
                .collect::<Vec<u8>>()
                .as_slice(),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.vbuf,
            offset as wgpu::BufferAddress,
            std::mem::size_of_val(mesh.vertices()) as wgpu::BufferAddress,
        );

        queue.submit(&[encoder.finish()]);
    }
}
