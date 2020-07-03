use super::{img_ids::Imgs, Show, TEXT_COLOR, TEXT_COLOR_3, UI_MAIN};

use crate::{i18n::VoxygenLocalization, ui::fonts::ConrodVoxygenFonts};
use client::{self, Client};
use conrod_core::{
    color,
    widget::{self, Button, Image, Rectangle, Scrollbar, Text},
    widget_ids, Colorable, Labelable, Positionable, Sizeable, Widget, WidgetCommon,
};

widget_ids! {
    pub struct Ids {
        social_frame,
        social_close,
        social_title,
        frame,
        align,
        scrollbar,
        content_align,
    }
}

pub enum Event {
    Close,
}

#[derive(WidgetCommon)]
pub struct Crafting<'a> {
    show: &'a Show,
    client: &'a Client,
    imgs: &'a Imgs,
    fonts: &'a ConrodVoxygenFonts,
    localized_strings: &'a std::sync::Arc<VoxygenLocalization>,

    #[conrod(common_builder)]
    common: widget::CommonBuilder,
}

impl<'a> Crafting<'a> {
    pub fn new(
        show: &'a Show,
        client: &'a Client,
        imgs: &'a Imgs,
        fonts: &'a ConrodVoxygenFonts,
        localized_strings: &'a std::sync::Arc<VoxygenLocalization>,
    ) -> Self {
        Self {
            show,
            client,
            imgs,
            fonts,
            localized_strings,
            common: widget::CommonBuilder::default(),
        }
    }
}

impl<'a> Widget for Crafting<'a> {
    type Event = Vec<Event>;
    type State = Ids;
    type Style = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State { Ids::new(id_gen) }

    #[allow(clippy::unused_unit)] // TODO: Pending review in #587
    fn style(&self) -> Self::Style { () }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            /* id, */ state: ids,
            ui,
            ..
        } = args;

        let mut events = Vec::new();

        Image::new(self.imgs.window_3)
            .top_left_with_margins_on(ui.window, 200.0, 25.0)
            .color(Some(UI_MAIN))
            .w_h(103.0 * 4.0, 122.0 * 4.0)
            .set(ids.social_frame, ui);

        // X-Button
        if Button::image(self.imgs.close_button)
            .w_h(28.0, 28.0)
            .hover_image(self.imgs.close_button_hover)
            .press_image(self.imgs.close_button_press)
            .top_right_with_margins_on(ids.social_frame, 0.0, 0.0)
            .set(ids.social_close, ui)
            .was_clicked()
        {
            events.push(Event::Close);
        }

        // Title
        Text::new(&self.localized_strings.get("hud.crafting"))
            .mid_top_with_margin_on(ids.social_frame, 6.0)
            .font_id(self.fonts.cyri.conrod_id)
            .font_size(self.fonts.cyri.scale(14))
            .color(TEXT_COLOR)
            .set(ids.social_title, ui);

        // Alignment
        Rectangle::fill_with([99.0 * 4.0, 112.0 * 4.0], color::TRANSPARENT)
            .mid_top_with_margin_on(ids.social_frame, 8.0 * 4.0)
            .set(ids.align, ui);
        // Content Alignment
        Rectangle::fill_with([94.0 * 4.0, 94.0 * 4.0], color::TRANSPARENT)
            .middle_of(ids.frame)
            .scroll_kids()
            .scroll_kids_vertically()
            .set(ids.content_align, ui);
        Scrollbar::y_axis(ids.content_align)
            .thickness(5.0)
            .rgba(0.33, 0.33, 0.33, 1.0)
            .set(ids.scrollbar, ui);
        // Frame
        Image::new(self.imgs.social_frame)
            .w_h(99.0 * 4.0, 100.0 * 4.0)
            .mid_bottom_of(ids.align)
            .color(Some(UI_MAIN))
            .set(ids.frame, ui);

        events
    }
}
