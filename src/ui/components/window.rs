use gpui::{
    App, Bounds, Context, CursorStyle, Decorations, Entity, HitboxBehavior, Hsla, MouseButton, Pixels, Point, ResizeEdge, Size, Window, canvas, div, point, prelude::*, px, rgb
};

use crate::ui::components::header::Header;
use crate::ui::components::sidebar::Sidebar;

pub struct AppWindow {
    pub header: Entity<Header>,
    pub sidebar: Entity<Sidebar>,
}

impl AppWindow {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            header: Header::new(cx),
            sidebar: Sidebar::new(cx),
        })
    }
}

// Things to do:
// 1. We need a way of calculating which edge or corner the mouse is on,
//    and then dispatch on that
// 2. We need to improve the shadow rendering significantly
// 3. We need to implement the techniques in here in Zed

impl Render for AppWindow {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let decorations = window.window_decorations();
        let rounding = px(10.0);
        let shadow_size = px(4.0);
        let border_size = px(1.0);
        let grey = rgb(0x808080);
        let header = self.header.clone();
        let sidebar = self.sidebar.clone();

        window.set_client_inset(shadow_size);

        div()
            .id("window-backdrop")
            .size_full()
            .bg(gpui::transparent_black())
            .map(|div| match decorations {
                Decorations::Server => div,
                Decorations::Client { .. } => div
                    .bg(gpui::transparent_black())
                    .child(
                        canvas(
                            |_bounds, window, _cx| {
                                window.insert_hitbox(
                                    Bounds::new(
                                        point(px(0.0), px(0.0)),
                                        window.window_bounds().get_bounds().size,
                                    ),
                                    HitboxBehavior::Normal,
                                )
                            },
                            move |_bounds, hitbox, window, _cx| {
                                let mouse = window.mouse_position();
                                let size = window.window_bounds().get_bounds().size;
                                let Some(edge) = resize_edge(mouse, shadow_size, size) else {
                                    return;
                                };
                                window.set_cursor_style(
                                    match edge {
                                        ResizeEdge::Top | ResizeEdge::Bottom => CursorStyle::ResizeUpDown,
                                        ResizeEdge::Left | ResizeEdge::Right => CursorStyle::ResizeLeftRight,
                                        ResizeEdge::TopLeft | ResizeEdge::BottomRight => CursorStyle::ResizeUpLeftDownRight,
                                        ResizeEdge::TopRight | ResizeEdge::BottomLeft => CursorStyle::ResizeUpRightDownLeft,
                                    },
                                    &hitbox,
                                );
                            },
                        )
                        .size_full()
                        .absolute(),
                    )
                    .rounded(rounding)
                    .p(shadow_size)
                    .on_mouse_move(|_e, window, _cx| window.refresh())
                    .on_mouse_down(MouseButton::Left, move |e, window, _cx| {
                        let size = window.window_bounds().get_bounds().size;
                        let pos = e.position;
                        match resize_edge(pos, shadow_size, size) {
                            Some(edge) => window.start_window_resize(edge),
                            None => window.start_window_move(),
                        };
                    }),
            })
            .child(
                div()
                    .cursor(CursorStyle::Arrow)
                    .size_full()
                    .flex()
                    .flex_col()
                    .justify_start()
                    .bg(gpui::rgba(0x1a1a1ae6))
                    .rounded(rounding)
                    .border(border_size)
                    .border_color(grey)
                    .shadow(vec![gpui::BoxShadow {
                        color: Hsla { h: 0., s: 0., l: 0., a: 0.4 },
                        blur_radius: shadow_size / 2.,
                        spread_radius: px(0.),
                        offset: point(px(0.0), px(0.0)),
                    }])
                    .on_mouse_move(|_e, _, cx| {
                        cx.stop_propagation();
                    })
                    .child(header)
                    .child(sidebar)
            )
    }
}

fn resize_edge(pos: Point<Pixels>, shadow_size: Pixels, size: Size<Pixels>) -> Option<ResizeEdge> {
    let edge = if pos.y < shadow_size && pos.x < shadow_size {
        ResizeEdge::TopLeft
    } else if pos.y < shadow_size && pos.x > size.width - shadow_size {
        ResizeEdge::TopRight
    } else if pos.y < shadow_size {
        ResizeEdge::Top
    } else if pos.y > size.height - shadow_size && pos.x < shadow_size {
        ResizeEdge::BottomLeft
    } else if pos.y > size.height - shadow_size && pos.x > size.width - shadow_size {
        ResizeEdge::BottomRight
    } else if pos.y > size.height - shadow_size {
        ResizeEdge::Bottom
    } else if pos.x < shadow_size {
        ResizeEdge::Left
    } else if pos.x > size.width - shadow_size {
        ResizeEdge::Right
    } else {
        return None;
    };
    Some(edge)
}
