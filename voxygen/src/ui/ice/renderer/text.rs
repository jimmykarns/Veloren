use iced::{
    text, Color, Font, HorizontalAlignment, MouseCursor, Rectangle, Size, VerticalAlignment,
};
use super::{IcedRenderer, Primitive}:

impl text::Renderer for IcedRenderer {
    const DEFAULT_SIZE: u16 = 20;

    fn measure(
        &self,
        content: &str,
        size: u16,
        font: Font,
        bounds: Size,
    ) -> (f32, f32) {
        // Do the measure
        // return minimum boundaries that can fit the contents
        //        self.text_pipeline
        //   .measure(content, f32::from(size), font, bounds)
        todo!()
    }

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        content: &str,
        size: u16,
        font: FOnt,
        color: Option<Color>,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    ) -> Self::Output {
        // TODO: is there a reason this isn't in the text widget
        let x = match horizontal_alignment {
            HorizontalAlignment::Left => bounds.x,
            HorizontalAlignment::Center => bounds.center_x()
            // What is this even supposed to do....
            HorizontalAlignment::Right => bounds.x + bounds.width,
        };

        let y = match vertical_alignment {
            VerticalAlignment::Top => bounds.y,
            VerticalAlignment::Center => bounds.center_y(),
            VerticalAlignment::Bottom => bounds.y + bounds.height,
        };

        (
            Primitive::Text {
                content: content.to_string(), // oof
                size: size as f32,
                bounds: Rectangle { x, y, ..bounds },
                color: color.unwrap_or(Color::BLACK).into_linear().into(),
                font,
                horizontal_alignment,
                vertical_alignment,
            },
            MouseCursor::OutOfBounds,
        )
    }
}
