use gpui::{
    App, AppContext, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb
};

use crate::ui::components::button::{WindowButton};

pub struct Header {}

impl Header {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self {})
    }
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(32.0))
            // .border_b_2()
            // .border_color(rgb(0xffffff))
            // .px_4()
            // .bg(self.theme.surface)
            // .border_b_1()
            // .border_color(self.theme.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .text_center()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .px_4()
                            // .pt_4()
                            .child("ZedGrip")
                    )
            )
            .child(
                div()
                    .flex()
                    // .gap_2()
                    .child(WindowButton::Minimize)
                    .child(WindowButton::Maximize)
                    .child(WindowButton::Close)
            )
    }
}