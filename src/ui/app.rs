use crate::theme::Theme;
use crate::ui::components::{header, query_panel, sidebar, window};
use gpui::{
    px, size, App, AppContext, Application, Bounds, Context, Entity, IntoElement, Point, Render,
    SharedString, TitlebarOptions, View, WindowBackgroundAppearance, WindowBounds,
    WindowDecorations, WindowOptions,
};

// --- New: MainView Structure ---
pub struct MainView {
    sidebar: View<sidebar::Sidebar>,
    query_panel: View<query_panel::QueryPanel>,
    content_width: Pixels,
}

impl MainView {
    pub fn new(cx: &mut AppContext) -> Self {
        let sidebar_view = cx.new_view(|cx| sidebar::Sidebar::new(cx).into_view(cx));
        let query_panel_view = cx.new_view(|_| query_panel::QueryPanel::new());

        // Observe sidebar width changes
        cx.subscribe(&sidebar_view, |this, _, event, cx| {
            if let sidebar::SidebarWidthChanged(width) = event {
                // For simplicity, let's assume total window width is fixed for now,
                // and we subtract sidebar width from it. In a real app, you'd get the actual window width.
                let window_bounds = cx.window_bounds().get_bounds();
                this.content_width = window_bounds.size.width - *width;
                cx.notify();
            }
        })
        .detach();

        Self {
            sidebar: sidebar_view,
            query_panel: query_panel_view,
            content_width: px(0.0), // Will be set correctly on first render or sidebar width change
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().h_full().child(self.sidebar.clone()).child(
            div()
                .flex_1()
                .w(self.content_width.max(px(0.0))) // Ensure non-negative width
                .child(self.query_panel.clone()),
        )
    }
}

pub struct ZedGripApp {
    theme: Theme,
    window: Entity<window::AppWindow>,
    header: Entity<header::Header>,
    main_view: View<MainView>,
}

impl Render for ZedGripApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.main_view.clone()
    }
}

impl ZedGripApp {
    pub fn start(cx: &mut AppContext) -> Entity<Self> {
        let theme = Theme::dark();
        let main_view = cx.new_view(|cx| MainView::new(cx));

        let bounds = Bounds::centered(None, size(px(1000.0), px(700.0)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_background: WindowBackgroundAppearance::Transparent,
            window_decorations: Some(WindowDecorations::Client),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::new("ZedGrip")),
                appears_transparent: false,
                traffic_light_position: Some(Point {
                    x: px(12.0),
                    y: px(11.0),
                }),
            }),
            kind: gpui::WindowKind::Normal,
            app_id: Some("org.oprimogus.zedgrip".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, move |window, cx| {
            window.set_window_title("ZedGrip");
            let header = cx.new_view(|_| header::Header::new(theme));
            let app_window = cx.new_view(|cx| {
                cx.observe_window_appearance(window, |_, window, _| {
                    window.refresh();
                })
                .detach();
                window::AppWindow::new().child(header.clone())
            });

            cx.new(|_cx| ZedGripApp {
                theme,
                window: app_window,
                header: header,
                main_view: main_view,
            })
        })
        .unwrap()
        .into_entity()
    }
}
