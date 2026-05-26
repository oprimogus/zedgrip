use gpui::{
    App, InteractiveElement, IntoElement, MouseButton, ParentElement, Pixels, RenderOnce, Rgba, SharedString, StatefulInteractiveElement, Styled, Window, WindowControlArea, div, px, rgb
};

use crate::theme::Theme;

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Default,
}

pub struct Button {
    text: SharedString,
    variant: ButtonVariant,
    theme: Theme
}

impl Button {
    pub fn new(text: impl Into<SharedString>, variant: ButtonVariant, theme: Theme) -> Self {
        Self {
            text: text.into(),
            variant,
            theme
        }
    }

    pub fn primary(text: impl Into<SharedString>, theme: Theme) -> Self {
        Self::new(text, ButtonVariant::Primary, theme)
    }

    pub fn secondary(text: impl Into<SharedString>, theme: Theme) -> Self {
        Self::new(text, ButtonVariant::Secondary, theme)
    }

    pub fn default(text: impl Into<SharedString>, theme: Theme) -> Self {
        Self::new(text, ButtonVariant::Default, theme)
    }

    fn colors(&self) -> ButtonColors {
        match self.variant {
            ButtonVariant::Primary => ButtonColors {
                bg: self.theme.accent,
                bg_hover: adjust_brightness(self.theme.accent, 1.2),
                text: rgb(0xffffff),
            },
            ButtonVariant::Secondary => ButtonColors {
                bg: self.theme.surface,
                bg_hover: adjust_brightness(self.theme.surface, 1.3),
                text: self.theme.text,
            },
            ButtonVariant::Default => ButtonColors {
                bg: Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
                bg_hover: self.theme.surface,
                text: self.theme.text,
            }
        }
    }

    fn padding(&self) -> (Pixels, Pixels) {
        match self.variant {
            _ => (px(16.0), px(8.0)),
        }
    }
}

struct ButtonColors {
    bg: Rgba,
    bg_hover: Rgba,
    text: Rgba,
}

#[derive(PartialEq, Clone, Copy, IntoElement)]
pub enum WindowButton {
    Close,
    Minimize,
    Maximize,
}

impl RenderOnce for WindowButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .w(px(28.0))
            .h(px(24.0))
            .items_center()
            .justify_center()
            .cursor_pointer()
            .id(match self {
                WindowButton::Close => "close",
                WindowButton::Minimize => "minimize",
                WindowButton::Maximize => "maximize",
            })
            .bg(Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
            .text_color(rgb(0xffffff))
            .hover(move |this| {
                this
                    .bg(rgb(0xef444433))
                    .text_color(rgb(0xffffff))
            })
            .window_control_area(match self {
                WindowButton::Close => WindowControlArea::Close,
                WindowButton::Minimize => WindowControlArea::Min,
                WindowButton::Maximize => WindowControlArea::Max,
            })
            .on_mouse_down(MouseButton::Left, |_, window, cx| {
                cx.stop_propagation();
                window.prevent_default();
            })
            .child(match self {
                WindowButton::Close => "×",
                WindowButton::Minimize => "−",
                WindowButton::Maximize => "□",
            })
            .on_click(move |_, window, cx| match self {
                WindowButton::Close => cx.quit(),
                WindowButton::Minimize => window.minimize_window(),
                WindowButton::Maximize => window.zoom_window(),
            })
    }
}

fn adjust_brightness(color: Rgba, factor: f32) -> Rgba {
    Rgba {
        r: (color.r * factor).min(1.0),
        g: (color.g * factor).min(1.0),
        b: (color.b * factor).min(1.0),
        a: color.a,
    }
}
