use zerocopy::AsBytes;

/// Represents a mesh that has been sent to the GPU.
pub struct Instances<T: Copy + AsBytes> {
    pub ibuf: wgpu::Buffer,
    pub len: usize,
    t: std::marker::PhantomData<T>,
}

impl<T: Copy + AsBytes> Instances<T> {
    pub fn new(device: &wgpu::Device, len: usize) -> Self {
        let instance_buffer = device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: None,
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        Self {
            ibuf: instance_buffer,
            len,
            t: std::marker::PhantomData::default(),
        }
    }

    pub fn count(&self) -> usize { self.len }

    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, vals: &[T]) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let staging_buffer = device.create_buffer_with_data(
            vals.iter()
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
            &self.ibuf,
            0,
            std::mem::size_of_val(vals) as wgpu::BufferAddress,
        );

        self.len = vals.len();

        queue.submit(&[encoder.finish()]);
    }
}
