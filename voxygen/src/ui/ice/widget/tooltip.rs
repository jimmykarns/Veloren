use iced::{layout, Clipboard, Element, Event, Hasher, Layout, Length, Point, Widget};
use std::hash::Hash;

/// TODO: block input through the tooltip
pub struct Tooltip<'a, M, R: self::Renderer> {
    title_text: &'a str,
    desc_text: &'a str,
    style: R::Style,
    content: Element<'a, M, R>,
}

impl<'a, M, R> Tooltip<'a, M, R>
where
    R: self::Renderer,
{
    pub fn new(
        title_text: &'a str,
        desc_text: &'a str,
        content: impl Into<Element<'a, M, R>>,
    ) -> Self {
        Self {
            title_text,
            desc_text,
            style: Default::default(),
            content: content.into(),
        }
    }

    /// Set the ratio (width/height)
    pub fn style(mut self, style: R::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, M, R> Widget<M, R> for Tooltip<'a, M, R>
where
    R: self::Renderer,
{
    fn width(&self) -> Length { Length::Fill }

    fn height(&self) -> Length { Length::Fill }

    fn layout(&self, renderer: &R, limits: &layout::Limits) -> layout::Node {
        self.content.layout(renderer, &limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<M>,
        renderer: &R,
        clipboard: Option<&dyn Clipboard>,
    ) {
        self.content.on_event(
            event,
            layout.children().next().unwrap(),
            cursor_position,
            messages,
            renderer,
            clipboard,
        );
    }

    fn draw(
        &self,
        renderer: &mut R,
        defaults: &R::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> R::Output {
        renderer.draw(
            defaults,
            cursor_position,
            self.title_text,
            self.desc_text,
            &self.style,
            &self.content,
            layout,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.content.hash_layout(state);
    }
}

pub trait Renderer: iced::Renderer {
    type Style: Default;

    fn draw<M>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: Point,
        title_text: &str,
        desc_text: &str,
        style: &Self::Style,
        content: &Element<'_, M, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output;
}

impl<'a, M, R> From<Tooltip<'a, M, R>> for Element<'a, M, R>
where
    R: 'a + self::Renderer,
    M: 'a,
{
    fn from(tooltip: Tooltip<'a, M, R>) -> Element<'a, M, R> { Element::new(tooltip) }
}
