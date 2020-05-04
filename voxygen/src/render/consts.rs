use zerocopy::AsBytes;

/// A handle to a series of constants sitting on the GPU. This is used to hold
/// information used in the rendering process that does not change throughout a
/// single render pass.
pub struct Consts<T: Copy + AsBytes> {
    pub bind_group: wgpu::BindGroup,
    buf: Option<wgpu::Buffer>,
    len: usize,
    t: std::marker::PhantomData<T>,
}

impl<T: Copy + AsBytes> Consts<T> {
    /// Create a new `Const<T>`.
    pub fn new(len: usize, buf: Option<wgpu::Buffer>, bind_group: wgpu::BindGroup) -> Self {
        Self {
            bind_group,
            buf,
            len,
            t: std::marker::PhantomData::default(),
        }
    }

    pub fn len(&self) -> usize { self.len }

    /// Update the GPU-side value represented by this constant handle, by adding
    /// a command to the current encoder queue.
    pub fn update_queue(&self, device: &wgpu::Device, queue: &wgpu::Queue, vals: &[T]) {
        log::debug!("Consts::update: {:?}", std::mem::size_of_val(vals));
        if let Some(buf) = self.buf.as_ref() {
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
                buf,
                0,
                std::mem::size_of_val(vals) as wgpu::BufferAddress,
            );

            queue.submit(&[encoder.finish()]);
        }
    }

    /// Update the GPU-side value represented by this constant handle.
    pub fn update(&self, device: &wgpu::Device, queue: &wgpu::Queue, vals: &[T]) {
        if std::mem::size_of_val(vals) == 0 {
            return;
        }
        log::debug!("Consts::update: {:?}", std::mem::size_of_val(vals));
        if let Some(buf) = self.buf.as_ref() {
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
                buf,
                0,
                std::mem::size_of_val(vals) as wgpu::BufferAddress,
            );

            queue.submit(&[encoder.finish()]);
        }
    }
}
