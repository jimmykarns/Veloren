use zerocopy::AsBytes;

/// A handle to a series of constants sitting on the GPU. This is used to hold
/// information used in the rendering process that does not change throughout a
/// single render pass.
pub struct Consts<T: Copy + AsBytes> {
    pub buf: wgpu::Buffer,
    len: usize,
    t: std::marker::PhantomData<T>,
}

impl<T: Copy + AsBytes> Consts<T> {
    /// Create a new `Const<T>`.
    pub fn new(device: &mut wgpu::Device, len: usize) -> Self {
        let uniform_buffer = device
            .create_buffer_mapped(&wgpu::BufferDescriptor {
                label: None,
                size: len as wgpu::BufferAddress,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
            .finish();

        Self {
            buf: uniform_buffer,
            len,
            t: std::marker::PhantomData::default(),
        }
    }

    pub fn len(&self) -> usize { self.len }

    /// Update the GPU-side value represented by this constant handle.
    pub fn update(&mut self, device: &mut wgpu::Device, queue: &mut wgpu::Queue, vals: &[T]) {
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
            &self.buf,
            0,
            std::mem::size_of_val(vals) as wgpu::BufferAddress,
        );

        self.len = std::mem::size_of_val(vals);

        queue.submit(&[encoder.finish()]);
    }
}
