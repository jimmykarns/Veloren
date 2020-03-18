use super::{
    super::{widget::image, Rotation},
    IcedRenderer, Primitive,
};
use iced::MouseCursor;

impl image::Renderer for IcedRenderer {
    fn dimensions(&self, handle: image::Handle) -> (u32, u32) {
        self.cache
            .graphic_cache()
            .get_graphic_dims((handle, Rotation::None))
            // TODO: don't unwrap
            .unwrap()
    }

    fn draw(&mut self, handle: image::Handle, layout: iced::Layout<'_>) -> Self::Output {
        (
            Primitive::Image {
                handle: (handle, Rotation::None),
                bounds: layout.bounds(),
            },
            MouseCursor::OutOfBounds,
        )
    }
}
