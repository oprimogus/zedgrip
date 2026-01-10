use gpui::{rgb, Rgba};

#[derive(Clone, Copy)]
pub struct Theme {
    pub background: Rgba,
    pub surface: Rgba,
    pub border: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,
    pub accent: Rgba,
    pub editor_bg: Rgba,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: rgb(0x1e1e1e),
            surface: rgb(0x252526),
            border: rgb(0x3e3e3e),
            text: rgb(0xcccccc),
            text_muted: rgb(0x858585),
            accent: rgb(0x007acc),
            editor_bg: rgb(0x1e1e1e),
        }
    }
}