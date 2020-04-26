use super::{mesh::Mesh, Pipeline};
use std::{ops::Range, sync::Arc};
use zerocopy::AsBytes;

/// Represents a mesh that has been sent to the GPU.
pub struct Model {
    pub vbuf: Arc<wgpu::Buffer>,
    pub vertex_range: Range<u32>,
}

impl Model {
    pub fn new<P: Pipeline>(device: &wgpu::Device, mesh: &Mesh<P>) -> Self {
        let vbuf = device.create_buffer_with_data(
            mesh.vertices()
                .iter()
                .map(|v| v.as_bytes())
                .flatten()
                .map(|v| *v)
                .collect::<Vec<u8>>()
                .as_slice(),
            wgpu::BufferUsage::VERTEX,
        );

        Self {
            vbuf: Arc::new(vbuf),
            vertex_range: 0..mesh.vertices().len() as u32,
        }
    }

    pub fn vertex_range(&self) -> Range<u32> { self.vertex_range.clone() }
}

/// Represents a mesh on the GPU which can be updated dynamically.
pub struct DynamicModel {
    pub vbuf: Arc<wgpu::Buffer>,
    size: usize,
}

impl DynamicModel {
    pub fn new(device: &mut wgpu::Device, size: usize) -> Self {
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

    /// Create a model with a slice of a portion of this model to send to the
    /// renderer.
    pub fn submodel(&self, range: Range<usize>) -> Model {
        Model {
            vbuf: self.vbuf.clone(),
            vertex_range: range.start as u32..range.end as u32,
        }
    }

    pub fn len(&self) -> usize { self.size }

    pub fn update<P: Pipeline>(
        &mut self,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
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
