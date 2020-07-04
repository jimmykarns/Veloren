use super::super::{
    super::{style::tooltip::Style, widget::tooltip},
    IcedRenderer,
};
use iced::{Element, Layout, Point, Rectangle};

impl tooltip::Renderer for IcedRenderer {
    type Style = Style;

    fn draw<M>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: Point,
        title_text: &str,
        desc_text: &str,
        style: &Self::Style,
        content: &Element<'_, M, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        // Tooltip magic here

        let glyphs = self.position_glyphs(
            bounds,
            horizontal_alignment,
            vertical_alignment,
            content,
            size,
            font,
        );

        (
            Primitive::Text {
                glyphs,
                //size: size as f32,
                bounds,
                linear_color: color.unwrap_or(defaults.text_color).into_linear().into(),
                /*font,
                 *horizontal_alignment,
                 *vertical_alignment, */
            },
            content.draw(self, defaults, content_layout, cursor_position),
        )
        }
    }
}

// Spacing between the tooltip and mouse
const MOUSE_PAD_Y: f64 = 15.0;

// Identifies tooltipped widget by their bounding box
#[derive(Copy, Clone, PartialEq)]
struct Identifier(Rectangle);

// Not wrapped into a Primitive so that we can adjust the transparency easily
struct TooltipDetails {
    title: Vec<glyph_brush::SectionGlyph>,
    title_bounds: Rectangle,
    desc: Vec<glyph_brush::SectionGlyph>,
    desc_bounds: Rectangle,
    bounds: Rectangle,
    style: style,
}

#[derive(Clone)]
enum State {
    Tooltip(TooltipDetails, Identifier),
    Fading(Instant, TooltipDetails, Identifier, Option<(Instant, Identifier)>),
    Hover(Instant, Identifier),
    None,
}
struct TooltipManager {
    state: State,
    // How long before a tooltip is displayed when hovering
    hover_dur: Duration,
    // How long it takes a tooltip to disappear
    fade_dur: Duration,
}


impl TooltipManager {
    pub fn new(
        hover_dur: Duration,
        fade_dur: DUration,
    ) -> Self {
        Self {
            state: State::None,
            hover_dur,
            fade_dur,
        }
    }

    pub fn proccess_tooltip(&mut self, cursor_pos: Point, title: &str, desc: &str, style: Style, hover_rect: Rectangle) {
        let under_mouse = point_in_rect(cursor_pos, hover_rect);


        let self.state = if under_mouse {
            match self.state {
                State::Tooltip(tooltip, identifier) if identifier.0 != hover_rect => {
                    State::Fading(Instant::now(), tooltip, identifier, Some((Instant::now(), Identifier(hover_rect)))) 
                }
                State::Fading(fade_start, tooltip, identifier, new_hover)
                state => state,
            }
        }

            State::Tooltip(tooltip, identifier) => {
                if point_in_rect(cursor_pos, hover_rect) {
                    if identifier.0 != hover_rect {
                    } else {
                        State::Tooltip(tooltip, identifier)
                    }
                } else if identifier.0 != hover_rect {
                    State::Tooltip(tooltip, identifier)
                } else {
                    State::Fading(Instant::now(), tooltip, identifier, None)
                }
            }
            State::Fading(fade_start, tooltip, identifier, new_hover) => {
                if point_in_rect(cursor_pos, hover_rect) {
                    if new_hover.1.0 == hover_rect {

                    } else if identifier == hover_rect {

                    } else {

                    }             
            }
    }
    
    pub fn tooltip_details(&self) -> Option<&Primitive> {
        match self.state {
            State::Tooltip(prim) => Some(prim),
            State::Fading(_, prim, _) => Some(prim), // TODO figure out transparency
            State::Hover(..) | State::None => None, 
        }
    }
}

fn point_in_rect(p: Point, r: Rectangle) -> bool {
    p.x > r.x && p.x < r.x + r.width && p.y > r.y && p.y < r.y + r.height
}
